//! 設定管理モジュール
//!
//! アプリケーションの設定をTOMLファイルで永続化します。
//! OS標準の設定ディレクトリに保存されます。

use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// アプリケーション名
const APP_NAME: &str = "quick-proj";
/// 設定ファイル名
const CONFIG_FILE_NAME: &str = "config.toml";

/// アプリケーション設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// 検索対象のルートパス一覧
    #[serde(default)]
    pub root_paths: Vec<PathBuf>,

    /// デフォルトのエディタコマンド
    #[serde(default)]
    pub editor: Option<String>,

    /// スキャンの最大深度
    #[serde(default = "default_max_depth")]
    pub max_depth: usize,

    /// プロジェクトとみなすマーカーファイル/ディレクトリ
    #[serde(default = "default_project_markers")]
    pub project_markers: Vec<String>,

    /// 除外するディレクトリ名
    #[serde(default = "default_exclude_dirs")]
    pub exclude_dirs: Vec<String>,
}

fn default_max_depth() -> usize {
    4
}

fn default_project_markers() -> Vec<String> {
    vec![
        ".git".to_string(),
        "Cargo.toml".to_string(),
        "package.json".to_string(),
        "go.mod".to_string(),
        "pyproject.toml".to_string(),
        "setup.py".to_string(),
        "pom.xml".to_string(),
        "build.gradle".to_string(),
        "Makefile".to_string(),
        "CMakeLists.txt".to_string(),
        "composer.json".to_string(),
        "Gemfile".to_string(),
        "mix.exs".to_string(),
        "deno.json".to_string(),
    ]
}

fn default_exclude_dirs() -> Vec<String> {
    vec![
        "node_modules".to_string(),
        "target".to_string(),
        ".venv".to_string(),
        "venv".to_string(),
        "__pycache__".to_string(),
        ".cache".to_string(),
        "dist".to_string(),
        "build".to_string(),
        ".next".to_string(),
        ".nuxt".to_string(),
        "vendor".to_string(),
    ]
}

impl Default for Config {
    fn default() -> Self {
        Self {
            root_paths: vec![],
            editor: None,
            max_depth: default_max_depth(),
            project_markers: default_project_markers(),
            exclude_dirs: default_exclude_dirs(),
        }
    }
}

impl Config {
    /// 設定ファイルを読み込む
    ///
    /// ファイルが存在しない場合はデフォルト設定を返します。
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;

        let config: Config = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", config_path.display()))?;

        Ok(config)
    }

    /// 設定ファイルに保存する
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;

        // 親ディレクトリを作成
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config directory: {}", parent.display()))?;
        }

        let content = toml::to_string_pretty(self)
            .with_context(|| "Failed to serialize config")?;

        fs::write(&config_path, content)
            .with_context(|| format!("Failed to write config file: {}", config_path.display()))?;

        Ok(())
    }

    /// 設定ファイルのパスを取得
    pub fn config_path() -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("", "", APP_NAME)
            .context("Failed to determine config directory")?;

        Ok(proj_dirs.config_dir().join(CONFIG_FILE_NAME))
    }

    /// ルートパスを追加
    pub fn add_root_path(&mut self, path: &Path) -> Result<bool> {
        // パスを展開して正規化
        let expanded = expand_path(path)?;
        let canonical = fs::canonicalize(&expanded)
            .with_context(|| format!("Path does not exist or is not accessible: {}", expanded.display()))?;

        // 既に登録済みかチェック
        if self.root_paths.contains(&canonical) {
            return Ok(false);
        }

        self.root_paths.push(canonical);
        Ok(true)
    }

    /// ルートパスを削除
    pub fn remove_root_path(&mut self, path: &Path) -> Result<bool> {
        let expanded = expand_path(path)?;

        // 正規化を試みる（存在しない場合は展開後のパスで比較）
        let target = fs::canonicalize(&expanded).unwrap_or(expanded);

        let original_len = self.root_paths.len();
        self.root_paths.retain(|p| p != &target);

        Ok(self.root_paths.len() < original_len)
    }

    /// エディタを設定
    pub fn set_editor(&mut self, editor: &str) {
        self.editor = Some(editor.to_string());
    }

    /// 使用するエディタを取得（優先順位に従う）
    pub fn get_editor(&self, cli_editor: Option<&str>) -> String {
        // 1. CLIオプション
        if let Some(editor) = cli_editor {
            return editor.to_string();
        }

        // 2. 設定ファイル
        if let Some(ref editor) = self.editor {
            return editor.clone();
        }

        // 3. 環境変数 EDITOR
        if let Ok(editor) = std::env::var("EDITOR") {
            return editor;
        }

        // 4. デフォルト
        "code".to_string()
    }
}

/// パスを展開する（~ をホームディレクトリに展開）
pub fn expand_path(path: &Path) -> Result<PathBuf> {
    let path_str = path.to_string_lossy();
    let expanded = shellexpand::tilde(&path_str);
    Ok(PathBuf::from(expanded.as_ref()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.root_paths.is_empty());
        assert_eq!(config.max_depth, 4);
        assert!(config.project_markers.contains(&".git".to_string()));
    }

    #[test]
    fn test_add_root_path() {
        let dir = tempdir().unwrap();
        let mut config = Config::default();

        // 追加成功
        let added = config.add_root_path(dir.path()).unwrap();
        assert!(added);
        assert_eq!(config.root_paths.len(), 1);

        // 重複追加は失敗
        let added_again = config.add_root_path(dir.path()).unwrap();
        assert!(!added_again);
        assert_eq!(config.root_paths.len(), 1);
    }

    #[test]
    fn test_remove_root_path() {
        let dir = tempdir().unwrap();
        let mut config = Config::default();

        config.add_root_path(dir.path()).unwrap();
        assert_eq!(config.root_paths.len(), 1);

        let removed = config.remove_root_path(dir.path()).unwrap();
        assert!(removed);
        assert!(config.root_paths.is_empty());
    }

    #[test]
    fn test_get_editor_priority() {
        let mut config = Config::default();

        // デフォルト
        assert_eq!(config.get_editor(None), "code");

        // 設定ファイル
        config.set_editor("vim");
        assert_eq!(config.get_editor(None), "vim");

        // CLIオプションが最優先
        assert_eq!(config.get_editor(Some("nvim")), "nvim");
    }

    #[test]
    fn test_expand_path() {
        let path = PathBuf::from("~/test");
        let expanded = expand_path(&path).unwrap();
        assert!(!expanded.to_string_lossy().contains('~'));
    }
}
