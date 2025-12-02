# リリース手順

このドキュメントでは、quick-proj の新しいバージョンをリリースする手順を説明します。

## コミットメッセージ規約

リリースノートは **Conventional Commits** から自動生成されます。
詳細は [CONTRIBUTING.md](../CONTRIBUTING.md) を参照してください。

```bash
feat(cli): add new option
fix(scanner): fix bug in directory traversal
docs: update README
```

## 前提条件

### GitHub Secret の設定（初回のみ）

Homebrew Tap への自動プッシュには Personal Access Token (PAT) が必要です。

1. **PAT を作成**
   - <https://github.com/settings/tokens?type=beta> へアクセス
   - 「Generate new token」をクリック
   - **Repository access**: `taro33333/homebrew-tap` を選択
   - **Permissions**: `Contents` → `Read and write`

2. **Secret を追加**
   - <https://github.com/taro33333/quick-proj/settings/secrets/actions>
   - **Name**: `HOMEBREW_TAP_TOKEN`
   - **Value**: 作成したPAT

---

## リリースフロー

### 1. バージョンを更新

`Cargo.toml` のバージョンを更新：

```toml
[package]
version = "0.2.0"  # ← 新しいバージョン
```

### 2. 変更をコミット

```bash
git add Cargo.toml Cargo.lock
git commit -m "chore(release): bump version to 0.2.0"
git push origin main
```

### 3. タグを作成してプッシュ

```bash
git tag v0.2.0
git push origin v0.2.0
```

### 4. 自動実行される処理

1. **ビルド** - Linux, macOS, Windows 向け
2. **GitHub Release 作成** - バイナリ + チェックサム
3. **Homebrew Formula 更新** - homebrew-tap に自動プッシュ

---

## バージョニング規則

[Semantic Versioning](https://semver.org/) に従います：

| 変更内容 | バージョン例 |
|---------|-------------|
| バグ修正 | 0.1.0 → 0.1.1 |
| 機能追加 | 0.1.0 → 0.2.0 |
| 破壊的変更 | 0.1.0 → 1.0.0 |

---

## トラブルシューティング

### タグを削除して再作成

```bash
git tag -d v0.2.0
git push origin :refs/tags/v0.2.0
git tag v0.2.0
git push origin v0.2.0
```

---

## ユーザー向けインストール方法

```bash
brew tap taro33333/tap
brew install quick-proj
```
