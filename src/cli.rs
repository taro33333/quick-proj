//! CLI引数の定義モジュール
//!
//! clapのderiveパターンを使用して、サブコマンドを持つCLIインターフェースを定義します。

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// quick-proj: 開発者のための高速プロジェクトランチャー
///
/// 登録したディレクトリからプロジェクトを検索し、
/// あいまい検索で選択してエディタで開きます。
#[derive(Parser, Debug)]
#[command(
    name = "quick-proj",
    version,
    author,
    about = "開発者のための高速プロジェクトランチャー",
    long_about = "登録したディレクトリからプロジェクトを高速スキャンし、\n\
                  あいまい検索で絞り込んでエディタで開きます。\n\n\
                  使用例:\n\
                    quick-proj add ~/src      # 検索対象パスを追加\n\
                    quick-proj                # プロジェクトを選択して開く"
)]
pub struct Args {
    /// サブコマンド（省略時はプロジェクト選択モード）
    #[command(subcommand)]
    pub command: Option<Command>,

    /// 使用するエディタコマンド（例: code, vim, nvim）
    #[arg(short, long, global = true, help = "使用するエディタコマンド")]
    pub editor: Option<String>,

    /// 検索の最大深度
    #[arg(short = 'd', long, global = true, help = "検索の最大深度")]
    pub max_depth: Option<usize>,
}

/// サブコマンドの定義
#[derive(Subcommand, Debug)]
pub enum Command {
    /// 検索対象のパスを追加
    #[command(about = "検索対象のパスを追加")]
    Add {
        /// 追加するパス
        #[arg(help = "追加するディレクトリパス")]
        path: PathBuf,
    },

    /// 検索対象のパスを削除
    #[command(about = "検索対象のパスを削除")]
    Remove {
        /// 削除するパス
        #[arg(help = "削除するディレクトリパス")]
        path: PathBuf,
    },

    /// 登録済みのパスを一覧表示
    #[command(about = "登録済みのパスを一覧表示")]
    List,

    /// 設定ファイルのパスを表示
    #[command(about = "設定ファイルのパスを表示")]
    Config,

    /// プロジェクト一覧をスキャンして表示
    #[command(about = "プロジェクト一覧をスキャンして表示")]
    Scan,

    /// デフォルトのエディタを設定
    #[command(about = "デフォルトのエディタを設定")]
    SetEditor {
        /// エディタコマンド（例: code, vim, nvim, cursor）
        #[arg(help = "エディタコマンド")]
        editor: String,
    },
}

impl Args {
    /// コマンドライン引数をパースしてArgs構造体を返す
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_no_command() {
        let args = Args::try_parse_from(["quick-proj"]).unwrap();
        assert!(args.command.is_none());
        assert!(args.editor.is_none());
    }

    #[test]
    fn test_args_add_command() {
        let args = Args::try_parse_from(["quick-proj", "add", "/tmp/test"]).unwrap();
        match args.command {
            Some(Command::Add { path }) => {
                assert_eq!(path, PathBuf::from("/tmp/test"));
            }
            _ => panic!("Expected Add command"),
        }
    }

    #[test]
    fn test_args_with_editor() {
        let args = Args::try_parse_from(["quick-proj", "--editor", "vim"]).unwrap();
        assert_eq!(args.editor, Some("vim".to_string()));
    }

    #[test]
    fn test_args_list_command() {
        let args = Args::try_parse_from(["quick-proj", "list"]).unwrap();
        assert!(matches!(args.command, Some(Command::List)));
    }
}
