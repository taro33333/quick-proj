//! エディタ起動モジュール
//!
//! 選択されたプロジェクトを指定のエディタで開きます。

use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

/// エディタコマンドのエイリアスマッピング
const EDITOR_ALIASES: &[(&str, &[&str])] = &[
    ("code", &["code"]),
    ("vscode", &["code"]),
    ("cursor", &["cursor"]),
    ("vim", &["vim"]),
    ("nvim", &["nvim"]),
    ("neovim", &["nvim"]),
    ("emacs", &["emacs"]),
    ("sublime", &["subl"]),
    ("subl", &["subl"]),
    ("atom", &["atom"]),
    ("idea", &["idea"]),
    ("intellij", &["idea"]),
    ("webstorm", &["webstorm"]),
    ("pycharm", &["pycharm"]),
    ("goland", &["goland"]),
    ("rustrover", &["rustrover"]),
    ("zed", &["zed"]),
];

/// エディタランチャー
pub struct Launcher {
    /// エディタコマンド
    editor: String,
}

impl Launcher {
    /// 新しいランチャーを作成
    pub fn new(editor: &str) -> Self {
        Self {
            editor: editor.to_string(),
        }
    }

    /// プロジェクトをエディタで開く
    pub fn launch(&self, project_path: &Path) -> Result<()> {
        let editor_cmd = self.resolve_editor();

        // エディタを起動
        Command::new(&editor_cmd)
            .arg(project_path)
            .spawn()
            .with_context(|| {
                format!(
                    "Failed to launch editor '{}'. Is it installed and in PATH?",
                    editor_cmd
                )
            })?;

        Ok(())
    }

    /// エディタコマンドを解決（エイリアスを展開）
    fn resolve_editor(&self) -> String {
        let editor_lower = self.editor.to_lowercase();

        // エイリアスをチェック
        for (alias, commands) in EDITOR_ALIASES {
            if *alias == editor_lower {
                return commands[0].to_string();
            }
        }

        // エイリアスになければそのまま返す
        self.editor.clone()
    }

    /// エディタが利用可能かチェック
    pub fn check_editor_available(&self) -> bool {
        let editor_cmd = self.resolve_editor();

        // `which` コマンドでチェック（Unix系）
        #[cfg(unix)]
        {
            Command::new("which")
                .arg(&editor_cmd)
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
        }

        // `where` コマンドでチェック（Windows）
        #[cfg(windows)]
        {
            Command::new("where")
                .arg(&editor_cmd)
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
        }
    }
}

/// 利用可能なエディタの一覧を取得
#[allow(dead_code)]
pub fn get_available_editors() -> Vec<String> {
    let mut available = Vec::new();

    for (alias, commands) in EDITOR_ALIASES {
        let launcher = Launcher::new(commands[0]);
        if launcher.check_editor_available() {
            available.push(alias.to_string());
        }
    }

    available
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_editor_alias() {
        let launcher = Launcher::new("vscode");
        assert_eq!(launcher.resolve_editor(), "code");

        let launcher = Launcher::new("neovim");
        assert_eq!(launcher.resolve_editor(), "nvim");
    }

    #[test]
    fn test_resolve_editor_no_alias() {
        let launcher = Launcher::new("my-custom-editor");
        assert_eq!(launcher.resolve_editor(), "my-custom-editor");
    }

    #[test]
    fn test_editor_aliases_exist() {
        // エイリアスが正しく定義されているか
        assert!(!EDITOR_ALIASES.is_empty());

        for (alias, commands) in EDITOR_ALIASES {
            assert!(!alias.is_empty());
            assert!(!commands.is_empty());
        }
    }
}
