# quick-proj

[![CI](https://github.com/taro33333/quick-proj/actions/workflows/ci.yml/badge.svg)](https://github.com/taro33333/quick-proj/actions/workflows/ci.yml)
[![Release](https://github.com/taro33333/quick-proj/actions/workflows/release.yml/badge.svg)](https://github.com/taro33333/quick-proj/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

🚀 開発者のための高速プロジェクトランチャー

## 概要

`quick-proj` は、登録したディレクトリからプロジェクトを高速スキャンし、あいまい検索で絞り込んでエディタで開くCLIツールです。

```bash
# プロジェクトを選択して開く
quick-proj
```

![Demo](docs/demo.gif)

## 特徴

- 🔍 **高速スキャン**: 並列処理で大量のプロジェクトを瞬時に検索
- ✨ **あいまい検索**: Fuzzy Selectで素早く絞り込み
- 🚀 **エディタ起動**: 選択したプロジェクトを即座に開く
- ⚙️ **柔軟な設定**: 検索パス、エディタ、マーカーをカスタマイズ可能
- 📦 **ゼロ設定**: `.git`, `Cargo.toml`, `package.json` などを自動検出

## インストール

### Homebrew（macOS / Linux）

```bash
brew tap taro33333/tap
brew install quick-proj
```

### GitHub Releases

[Releases ページ](https://github.com/taro33333/quick-proj/releases) からバイナリをダウンロード：

```bash
# macOS Apple Silicon
curl -LO https://github.com/taro33333/quick-proj/releases/latest/download/quick-proj-darwin-arm64
chmod +x quick-proj-darwin-arm64
sudo mv quick-proj-darwin-arm64 /usr/local/bin/quick-proj
```

### ソースからビルド

```bash
git clone https://github.com/taro33333/quick-proj.git
cd quick-proj
cargo install --path .
```

## クイックスタート

```bash
# 1. 検索対象のパスを追加
quick-proj add ~/src
quick-proj add ~/projects

# 2. プロジェクトを選択して開く
quick-proj
```

## 使用方法

### 基本コマンド

```bash
# プロジェクト選択モード（メイン機能）
quick-proj

# 検索パスを追加
quick-proj add ~/src

# 検索パスを削除
quick-proj remove ~/old-projects

# 登録済みパスを一覧表示
quick-proj list

# プロジェクト一覧を表示（デバッグ用）
quick-proj scan

# 設定ファイルの情報を表示
quick-proj config

# デフォルトエディタを設定
quick-proj set-editor cursor
```

### オプション

```bash
# 別のエディタで開く
quick-proj --editor vim

# 検索深度を変更
quick-proj --max-depth 6
```

### 使用例

```bash
# VS Code で開く（デフォルト）
quick-proj

# Cursor で開く
quick-proj -e cursor

# Vim で開く
quick-proj -e vim

# 深い階層まで検索
quick-proj -d 8
```

## 設定

### 設定ファイルの場所

| OS | パス |
|----|------|
| macOS | `~/Library/Application Support/quick-proj/config.toml` |
| Linux | `~/.config/quick-proj/config.toml` |
| Windows | `%APPDATA%\quick-proj\config.toml` |

### 設定例

```toml
# 検索対象のパス
root_paths = [
    "/Users/user/src",
    "/Users/user/projects",
]

# デフォルトのエディタ
editor = "cursor"

# スキャンの最大深度
max_depth = 4

# プロジェクトとみなすマーカー
project_markers = [
    ".git",
    "Cargo.toml",
    "package.json",
    "go.mod",
    "pyproject.toml",
]

# 除外するディレクトリ
exclude_dirs = [
    "node_modules",
    "target",
    ".venv",
]
```

## プロジェクト検出マーカー

以下のファイル/ディレクトリが存在するフォルダをプロジェクトとして検出します：

| マーカー | 言語/ツール |
|---------|------------|
| `.git` | Git リポジトリ |
| `Cargo.toml` | Rust |
| `package.json` | Node.js / JavaScript |
| `go.mod` | Go |
| `pyproject.toml` | Python |
| `pom.xml` | Java (Maven) |
| `build.gradle` | Java (Gradle) |
| `Makefile` | Make プロジェクト |
| `CMakeLists.txt` | CMake |
| `composer.json` | PHP |
| `Gemfile` | Ruby |
| `mix.exs` | Elixir |
| `deno.json` | Deno |

## エディタ対応

以下のエディタがエイリアスとしてサポートされています：

| エイリアス | コマンド |
|-----------|---------|
| `code`, `vscode` | `code` |
| `cursor` | `cursor` |
| `vim`, `nvim`, `neovim` | `vim`, `nvim` |
| `emacs` | `emacs` |
| `sublime`, `subl` | `subl` |
| `idea`, `intellij` | `idea` |
| `zed` | `zed` |

環境変数 `EDITOR` も使用できます。

## 開発

```bash
# ビルド
cargo build

# テスト
cargo test

# フォーマット
cargo fmt

# リント
cargo clippy
```

## ライセンス

MIT License

## リンク

- [GitHub Repository](https://github.com/taro33333/quick-proj)
- [Releases](https://github.com/taro33333/quick-proj/releases)
