//! ユーザーインターフェースモジュール
//!
//! dialoguerを使用したインタラクティブな選択UIを提供します。

use crate::scanner::Project;
use anyhow::{Context, Result};
use colored::Colorize;
use console::Term;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};

/// プロジェクト選択UIを表示
///
/// あいまい検索で絞り込み、矢印キーで選択できるUIを表示します。
pub fn select_project(projects: &[Project]) -> Result<Option<&Project>> {
    if projects.is_empty() {
        return Ok(None);
    }

    // 表示用の文字列リストを作成
    let items: Vec<String> = projects
        .iter()
        .map(|p| format_project_item(p))
        .collect();

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a project")
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .context("Failed to show selection UI")?;

    Ok(selection.map(|idx| &projects[idx]))
}

/// プロジェクト項目のフォーマット
fn format_project_item(project: &Project) -> String {
    // パスからホームディレクトリを短縮
    let path_display = shorten_home_path(&project.path.to_string_lossy());

    format!(
        "{} {}",
        project.name.bold(),
        format!("({})", path_display).dimmed()
    )
}

/// ホームディレクトリを ~ に短縮
fn shorten_home_path(path: &str) -> String {
    if let Ok(home) = std::env::var("HOME") {
        if path.starts_with(&home) {
            return path.replacen(&home, "~", 1);
        }
    }
    path.to_string()
}

/// スキャン結果のサマリーを表示
pub fn print_scan_summary(projects: &[Project], elapsed_ms: u128) {
    println!();
    println!(
        "{} {} projects found in {}ms",
        "✓".green().bold(),
        projects.len().to_string().cyan(),
        elapsed_ms
    );
}

/// プロジェクト一覧を表示
pub fn print_project_list(projects: &[Project]) {
    if projects.is_empty() {
        println!("{}", "No projects found.".yellow());
        return;
    }

    println!();
    println!("{}", "Projects:".bold());
    println!();

    for project in projects {
        let path_display = shorten_home_path(&project.path.to_string_lossy());
        println!(
            "  {} {} {}",
            "•".cyan(),
            project.name.bold(),
            format!("({})", path_display).dimmed()
        );
    }

    println!();
    println!("Total: {} projects", projects.len().to_string().cyan());
}

/// 登録済みパスの一覧を表示
pub fn print_root_paths(paths: &[std::path::PathBuf]) {
    if paths.is_empty() {
        println!("{}", "No root paths configured.".yellow());
        println!();
        println!("Add a path with:");
        println!("  {} {}", "quick-proj add".cyan(), "<PATH>".dimmed());
        return;
    }

    println!();
    println!("{}", "Registered paths:".bold());
    println!();

    for (i, path) in paths.iter().enumerate() {
        let path_display = shorten_home_path(&path.to_string_lossy());
        let exists = path.exists();
        let status = if exists {
            "✓".green()
        } else {
            "✗".red()
        };

        println!("  {} {}. {}", status, i + 1, path_display);
    }

    println!();
}

/// 設定ファイルのパスを表示
pub fn print_config_path(path: &std::path::Path) {
    println!();
    println!("{}", "Configuration:".bold());
    println!();
    println!("  Path: {}", path.display().to_string().cyan());
    println!(
        "  Exists: {}",
        if path.exists() {
            "Yes".green()
        } else {
            "No".yellow()
        }
    );
    println!();
}

/// エラーメッセージを表示
pub fn print_error(message: &str) {
    eprintln!("{} {}", "Error:".red().bold(), message);
}

/// 成功メッセージを表示
pub fn print_success(message: &str) {
    println!("{} {}", "✓".green().bold(), message);
}

/// 警告メッセージを表示
pub fn print_warning(message: &str) {
    println!("{} {}", "⚠".yellow().bold(), message);
}

/// 情報メッセージを表示
#[allow(dead_code)]
pub fn print_info(message: &str) {
    println!("{} {}", "ℹ".blue().bold(), message);
}

/// バナーを表示
#[allow(dead_code)]
pub fn print_banner() {
    println!(
        "{}",
        r#"
  ╔═══════════════════════════════════════════╗
  ║                                           ║
  ║   🚀 quick-proj                           ║
  ║   Fast project launcher for developers    ║
  ║                                           ║
  ╚═══════════════════════════════════════════╝
"#
        .cyan()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shorten_home_path() {
        // HOMEが設定されている場合のテスト
        if let Ok(home) = std::env::var("HOME") {
            let path = format!("{}/projects/test", home);
            let shortened = shorten_home_path(&path);
            assert!(shortened.starts_with('~'));
            assert!(!shortened.contains(&home));
        }
    }

    #[test]
    fn test_format_project_item() {
        let project = Project {
            path: std::path::PathBuf::from("/tmp/test-project"),
            name: "test-project".to_string(),
            marker: ".git".to_string(),
        };

        let formatted = format_project_item(&project);
        assert!(formatted.contains("test-project"));
    }
}
