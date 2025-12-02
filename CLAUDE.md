# CLAUDE.md - quick-proj プロジェクトガイド

このファイルはClaude Code（claude.ai/code）がこのリポジトリを理解するためのガイドです。

## プロジェクト概要

**quick-proj** は、開発者のための高速プロジェクトランチャーCLIツールです。

### 主な機能

- 高速スキャン: 登録ディレクトリからプロジェクトを並列探索
- あいまい検索: Fuzzy Selectで素早く絞り込み
- エディタ起動: 選択したプロジェクトを即座に開く
- 設定管理: 検索対象パスの追加・削除

## ビルドとテスト

```bash
# ビルド
cargo build

# リリースビルド
cargo build --release

# テスト実行
cargo test

# フォーマット
cargo fmt

# リント
cargo clippy

# 実行
cargo run -- add ~/src
cargo run
```

## アーキテクチャ

```
src/
├── main.rs       # エントリーポイント、コマンドディスパッチ
├── cli.rs        # clap deriveによるCLI引数定義
├── config.rs     # 設定の読み書き（TOML）
├── scanner.rs    # ディレクトリ探索（ignore + rayon）
├── launcher.rs   # エディタ起動
└── ui.rs         # dialoguerによる選択UI
```

### モジュール責務

| モジュール | 責務 |
|-----------|------|
| `cli.rs` | CLIコマンド・オプションの定義 |
| `config.rs` | 設定ファイルの読み書き、パス管理 |
| `scanner.rs` | プロジェクトのスキャン、マーカー検出 |
| `launcher.rs` | エディタプロセスの起動 |
| `ui.rs` | ユーザー対話UI（選択、表示） |

## 主要な型

```rust
// 設定
pub struct Config {
    pub root_paths: Vec<PathBuf>,
    pub editor: Option<String>,
    pub max_depth: usize,
    pub project_markers: Vec<String>,
    pub exclude_dirs: Vec<String>,
}

// プロジェクト
pub struct Project {
    pub path: PathBuf,
    pub name: String,
    pub marker: String,
}

// CLIコマンド
pub enum Command {
    Add { path: PathBuf },
    Remove { path: PathBuf },
    List,
    Config,
    Scan,
    SetEditor { editor: String },
}
```

## 依存クレート

| クレート | 用途 |
|---------|------|
| `clap` | CLI引数解析 |
| `anyhow` | エラーハンドリング |
| `serde` + `toml` | 設定ファイル |
| `directories` | OS標準パス |
| `ignore` | 高速ディレクトリ走査 |
| `rayon` | 並列処理 |
| `dialoguer` | 選択UI |
| `colored` | 色付き出力 |
| `shellexpand` | パス展開 |

## コーディング規約

- **エラー処理**: `anyhow::Result` を使用、`.with_context()` でコンテキスト付与
- **パス操作**: `std::path::PathBuf` を使用
- **並列処理**: `rayon` の `par_iter()` を活用
- **テスト**: 各モジュールに `#[cfg(test)]` でユニットテストを配置

## CI/CD

- **CI**: `.github/workflows/ci.yml` - fmt, clippy, test, build
- **Release**: `.github/workflows/release.yml` - マルチプラットフォームバイナリ + Homebrew

## 設定ファイルの場所

| OS | パス |
|----|------|
| macOS | `~/Library/Application Support/quick-proj/config.toml` |
| Linux | `~/.config/quick-proj/config.toml` |
| Windows | `%APPDATA%\quick-proj\config.toml` |
