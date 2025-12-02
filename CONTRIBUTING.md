# Contributing to quick-proj

quick-proj への貢献をありがとうございます！

## 開発環境のセットアップ

```bash
# リポジトリをクローン
git clone https://github.com/taro33333/quick-proj.git
cd quick-proj

# ビルド
cargo build

# テスト
cargo test

# リント
cargo clippy

# フォーマット
cargo fmt
```

## コミットメッセージ規約

このプロジェクトは **Conventional Commits** を採用しています。
リリースノートはコミットメッセージから自動生成されます。

### フォーマット

```
<type>(<scope>): <description>
```

### タイプ一覧

| タイプ | 説明 | リリースノートでの表示 |
|-------|------|----------------------|
| `feat` | 新機能 | ✨ Features |
| `fix` | バグ修正 | 🐛 Bug Fixes |
| `docs` | ドキュメント変更 | 📚 Documentation |
| `refactor` | リファクタリング | ♻️ Refactor |
| `perf` | パフォーマンス改善 | ⚡ Performance |
| `test` | テスト追加・修正 | 🧪 Testing |
| `chore` | その他の変更 | ⚙️ Miscellaneous Tasks |

### スコープ（オプション）

| スコープ | 説明 |
|---------|------|
| `cli` | CLI関連 |
| `config` | 設定関連 |
| `scanner` | スキャン関連 |
| `launcher` | エディタ起動関連 |
| `ui` | UI関連 |
| `ci` | CI/CD関連 |

### 例

```bash
# 新機能
git commit -m "feat(cli): add --filter option for project filtering"

# バグ修正
git commit -m "fix(scanner): handle symlinks correctly"

# ドキュメント
git commit -m "docs: update installation instructions"

# パフォーマンス改善
git commit -m "perf(scanner): optimize parallel scanning"
```

## プルリクエスト

1. フォークしてブランチを作成
2. 変更を加える
3. テストを通す (`cargo test`)
4. リントを通す (`cargo clippy`)
5. フォーマットする (`cargo fmt`)
6. プルリクエストを作成

## 質問・問題報告

- バグ報告: [Issues](https://github.com/taro33333/quick-proj/issues)
- 質問: [Discussions](https://github.com/taro33333/quick-proj/discussions)
