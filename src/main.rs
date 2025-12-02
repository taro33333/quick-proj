//! quick-proj
//!
//! 開発者のための高速プロジェクトランチャー
//!
//! # 機能
//! - 高速スキャン: 登録ディレクトリからプロジェクトを並列探索
//! - あいまい検索: Fuzzy Selectで素早く絞り込み
//! - エディタ起動: 選択したプロジェクトを即座に開く
//! - 設定管理: 検索対象パスの追加・削除

mod cli;
mod config;
mod launcher;
mod scanner;
mod ui;

use anyhow::Result;
use cli::{Args, Command};
use colored::Colorize;
use config::Config;
use launcher::Launcher;
use scanner::Scanner;
use std::time::Instant;

fn main() -> Result<()> {
    let args = Args::parse_args();

    match args.command {
        Some(Command::Add { path }) => cmd_add(&path),
        Some(Command::Remove { path }) => cmd_remove(&path),
        Some(Command::List) => cmd_list(),
        Some(Command::Config) => cmd_config(),
        Some(Command::Scan) => cmd_scan(args.max_depth),
        Some(Command::SetEditor { editor }) => cmd_set_editor(&editor),
        None => cmd_select(args.editor.as_deref(), args.max_depth),
    }
}

/// プロジェクト選択モード（メイン機能）
fn cmd_select(cli_editor: Option<&str>, cli_max_depth: Option<usize>) -> Result<()> {
    let mut config = Config::load()?;

    // CLI引数で上書き
    if let Some(depth) = cli_max_depth {
        config.max_depth = depth;
    }

    // ルートパスが未設定の場合
    if config.root_paths.is_empty() {
        ui::print_warning("No root paths configured.");
        println!();
        println!("Add a search path first:");
        println!("  {} {}", "quick-proj add".cyan(), "~/src".dimmed());
        println!("  {} {}", "quick-proj add".cyan(), "~/projects".dimmed());
        return Ok(());
    }

    // スキャン開始
    let start = Instant::now();
    let scanner = Scanner::from_config(&config);
    let projects = scanner.scan(&config.root_paths)?;
    let elapsed = start.elapsed().as_millis();

    if projects.is_empty() {
        ui::print_warning("No projects found in registered paths.");
        println!();
        println!("Check if your paths contain projects with markers like:");
        println!("  .git, Cargo.toml, package.json, go.mod, etc.");
        return Ok(());
    }

    // サマリー表示
    ui::print_scan_summary(&projects, elapsed);

    // プロジェクト選択UI
    match ui::select_project(&projects)? {
        Some(project) => {
            let editor = config.get_editor(cli_editor);
            let launcher = Launcher::new(&editor);

            println!();
            println!(
                "Opening {} with {}...",
                project.name.cyan().bold(),
                editor.green()
            );

            launcher.launch(&project.path)?;
        }
        None => {
            println!();
            println!("{}", "Selection cancelled.".dimmed());
        }
    }

    Ok(())
}

/// パス追加コマンド
fn cmd_add(path: &std::path::Path) -> Result<()> {
    let mut config = Config::load()?;

    match config.add_root_path(path) {
        Ok(true) => {
            config.save()?;
            let expanded = config::expand_path(path)?;
            ui::print_success(&format!("Added: {}", expanded.display()));
        }
        Ok(false) => {
            ui::print_warning("Path is already registered.");
        }
        Err(e) => {
            ui::print_error(&format!("{}", e));
            return Err(e);
        }
    }

    Ok(())
}

/// パス削除コマンド
fn cmd_remove(path: &std::path::Path) -> Result<()> {
    let mut config = Config::load()?;

    if config.remove_root_path(path)? {
        config.save()?;
        ui::print_success(&format!("Removed: {}", path.display()));
    } else {
        ui::print_warning("Path not found in configuration.");
    }

    Ok(())
}

/// パス一覧コマンド
fn cmd_list() -> Result<()> {
    let config = Config::load()?;
    ui::print_root_paths(&config.root_paths);
    Ok(())
}

/// 設定ファイルパス表示コマンド
fn cmd_config() -> Result<()> {
    let path = Config::config_path()?;
    ui::print_config_path(&path);

    // 現在の設定を表示
    let config = Config::load()?;
    println!("{}", "Current settings:".bold());
    println!();
    println!(
        "  Editor:      {}",
        config.editor.as_deref().unwrap_or("(not set, using $EDITOR or 'code')").cyan()
    );
    println!("  Max depth:   {}", config.max_depth.to_string().cyan());
    println!(
        "  Markers:     {} items",
        config.project_markers.len().to_string().cyan()
    );
    println!(
        "  Exclude:     {} patterns",
        config.exclude_dirs.len().to_string().cyan()
    );
    println!();

    Ok(())
}

/// スキャンコマンド（デバッグ用）
fn cmd_scan(cli_max_depth: Option<usize>) -> Result<()> {
    let mut config = Config::load()?;

    if let Some(depth) = cli_max_depth {
        config.max_depth = depth;
    }

    if config.root_paths.is_empty() {
        ui::print_warning("No root paths configured.");
        return Ok(());
    }

    let start = Instant::now();
    let scanner = Scanner::from_config(&config);
    let projects = scanner.scan(&config.root_paths)?;
    let elapsed = start.elapsed().as_millis();

    ui::print_project_list(&projects);
    println!("Scan completed in {}ms", elapsed.to_string().green());

    Ok(())
}

/// エディタ設定コマンド
fn cmd_set_editor(editor: &str) -> Result<()> {
    let mut config = Config::load()?;

    // エディタの存在チェック
    let launcher = Launcher::new(editor);
    if !launcher.check_editor_available() {
        ui::print_warning(&format!(
            "Editor '{}' not found in PATH. Setting anyway.",
            editor
        ));
    }

    config.set_editor(editor);
    config.save()?;

    ui::print_success(&format!("Default editor set to: {}", editor.cyan()));

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_main_exists() {
        // main関数が存在することを確認
    }
}
