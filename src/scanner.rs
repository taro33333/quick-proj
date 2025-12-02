//! ディレクトリ探索モジュール
//!
//! プロジェクトフォルダを高速にスキャンします。
//! `ignore` クレートを使用して .gitignore を考慮し、
//! `rayon` で並列処理を行います。

use crate::config::Config;
use anyhow::Result;
use ignore::WalkBuilder;
use rayon::prelude::*;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

/// スキャンされたプロジェクト情報
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Project {
    /// プロジェクトのパス
    pub path: PathBuf,
    /// プロジェクト名（ディレクトリ名）
    pub name: String,
    /// 検出されたマーカー
    pub marker: String,
}

impl Project {
    /// 表示用の文字列を生成
    #[allow(dead_code)]
    pub fn display_string(&self) -> String {
        format!("{} ({})", self.name, self.path.display())
    }

    /// 短い表示用文字列（パスのみ）
    #[allow(dead_code)]
    pub fn short_display(&self) -> String {
        self.path.display().to_string()
    }
}

/// プロジェクトスキャナー
pub struct Scanner {
    /// プロジェクトマーカー
    markers: HashSet<String>,
    /// 除外ディレクトリ
    exclude_dirs: HashSet<String>,
    /// 最大深度
    max_depth: usize,
}

impl Scanner {
    /// 設定からスキャナーを作成
    pub fn from_config(config: &Config) -> Self {
        Self {
            markers: config.project_markers.iter().cloned().collect(),
            exclude_dirs: config.exclude_dirs.iter().cloned().collect(),
            max_depth: config.max_depth,
        }
    }

    /// 指定されたルートパスからプロジェクトをスキャン
    pub fn scan(&self, root_paths: &[PathBuf]) -> Result<Vec<Project>> {
        let projects = Arc::new(Mutex::new(Vec::new()));
        let seen_paths = Arc::new(Mutex::new(HashSet::new()));

        // 各ルートパスを並列処理
        root_paths.par_iter().for_each(|root| {
            if let Ok(found) = self.scan_root(root) {
                let mut projects_lock = projects.lock().unwrap();
                let mut seen_lock = seen_paths.lock().unwrap();

                for project in found {
                    // 重複を排除
                    if seen_lock.insert(project.path.clone()) {
                        projects_lock.push(project);
                    }
                }
            }
        });

        let mut result = Arc::try_unwrap(projects)
            .unwrap()
            .into_inner()
            .unwrap();

        // プロジェクト名でソート
        result.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

        Ok(result)
    }

    /// 単一のルートパスをスキャン
    fn scan_root(&self, root: &Path) -> Result<Vec<Project>> {
        if !root.exists() {
            return Ok(vec![]);
        }

        let mut projects = Vec::new();
        let mut visited = HashSet::new();

        // ignore クレートを使用してウォーク
        let walker = WalkBuilder::new(root)
            .max_depth(Some(self.max_depth))
            .hidden(false)  // 隠しディレクトリも探索（.git検出のため）
            .git_ignore(true)
            .git_global(true)
            .git_exclude(true)
            .follow_links(false)
            .build();

        for entry in walker.flatten() {
            let path = entry.path();

            // ディレクトリのみ対象
            if !path.is_dir() {
                continue;
            }

            // 除外ディレクトリをスキップ
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if self.exclude_dirs.contains(name) {
                    continue;
                }
            }

            // 既にプロジェクトとして検出された親ディレクトリの子はスキップ
            if self.is_under_project(&visited, path) {
                continue;
            }

            // マーカーをチェック
            if let Some(marker) = self.detect_marker(path) {
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                projects.push(Project {
                    path: path.to_path_buf(),
                    name,
                    marker,
                });

                visited.insert(path.to_path_buf());
            }
        }

        Ok(projects)
    }

    /// ディレクトリがプロジェクトかどうかを判定
    fn detect_marker(&self, dir: &Path) -> Option<String> {
        for marker in &self.markers {
            let marker_path = dir.join(marker);
            if marker_path.exists() {
                return Some(marker.clone());
            }
        }
        None
    }

    /// パスが既に検出されたプロジェクトの配下にあるかチェック
    fn is_under_project(&self, visited: &HashSet<PathBuf>, path: &Path) -> bool {
        let mut current = path.parent();
        while let Some(parent) = current {
            if visited.contains(parent) {
                return true;
            }
            current = parent.parent();
        }
        false
    }
}

/// プロジェクト一覧を検索クエリでフィルタリング
#[allow(dead_code)]
pub fn filter_projects<'a>(projects: &'a [Project], query: &str) -> Vec<&'a Project> {
    if query.is_empty() {
        return projects.iter().collect();
    }

    let query_lower = query.to_lowercase();
    let query_parts: Vec<&str> = query_lower.split_whitespace().collect();

    projects
        .iter()
        .filter(|p| {
            let name_lower = p.name.to_lowercase();
            let path_lower = p.path.to_string_lossy().to_lowercase();

            // すべてのクエリパートがマッチする必要がある
            query_parts.iter().all(|part| {
                name_lower.contains(part) || path_lower.contains(part)
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use tempfile::tempdir;

    fn create_test_project(dir: &Path, marker: &str) {
        fs::create_dir_all(dir).unwrap();
        if marker.contains('.') || marker == "Makefile" || marker == "Gemfile" {
            // ファイルマーカー
            File::create(dir.join(marker)).unwrap();
        } else {
            // ディレクトリマーカー
            fs::create_dir_all(dir.join(marker)).unwrap();
        }
    }

    #[test]
    fn test_scan_finds_projects() {
        let root = tempdir().unwrap();

        // プロジェクトを作成
        create_test_project(&root.path().join("project-a"), ".git");
        create_test_project(&root.path().join("project-b"), "Cargo.toml");
        create_test_project(&root.path().join("project-c"), "package.json");

        let config = Config::default();
        let scanner = Scanner::from_config(&config);
        let projects = scanner.scan(&[root.path().to_path_buf()]).unwrap();

        assert_eq!(projects.len(), 3);
    }

    #[test]
    fn test_scan_ignores_nested_projects() {
        let root = tempdir().unwrap();

        // 親プロジェクト
        create_test_project(&root.path().join("parent"), ".git");
        // 子プロジェクト（親の中にある）
        create_test_project(&root.path().join("parent").join("child"), "Cargo.toml");

        let config = Config::default();
        let scanner = Scanner::from_config(&config);
        let projects = scanner.scan(&[root.path().to_path_buf()]).unwrap();

        // 親のみが検出される
        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0].name, "parent");
    }

    #[test]
    fn test_filter_projects() {
        let projects = vec![
            Project {
                path: PathBuf::from("/home/user/rust-project"),
                name: "rust-project".to_string(),
                marker: "Cargo.toml".to_string(),
            },
            Project {
                path: PathBuf::from("/home/user/node-app"),
                name: "node-app".to_string(),
                marker: "package.json".to_string(),
            },
        ];

        // "rust" でフィルタ
        let filtered = filter_projects(&projects, "rust");
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "rust-project");

        // 空クエリは全件
        let all = filter_projects(&projects, "");
        assert_eq!(all.len(), 2);
    }
}
