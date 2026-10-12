# CLAUDE.md — 冒険者ギルド経営シミュレーション

> **正式な設計文書は [`AGENTS.md`](AGENTS.md) を参照してください。**
> このファイルは Claude Code (claude-code CLI) が自動的に読み取るリポジトリレベルの指示書です。

---

## プロジェクト概要

冒険者ギルド経営シミュレーション。Rust (eframe/egui) デスクトップアプリ。
完全ローカル・オフライン完結（サーバー依存ゼロ、インフラ費用ゼロ）。

---

## 絶対ルール

### アーキテクチャ
- `src/core/` は **純粋 Rust**。`egui` / `eframe` への依存を一切持たない。
- `src/ui/` は `egui` 描画のみ。`GameState` を受け取って表示し、ユーザー操作を `core` メソッド呼び出しに変換する。
- **Action パターン**: UI 描画中は `&self` で読み取り、操作を enum (`QuestAction`, `SettingsAction` 等) に蓄積、描画後に `&mut self` で適用。借用チェッカー衝突を回避。

### コーディング
- Rust 2024 Edition。`unsafe` コード禁止 (`unsafe_code = "deny"`)。
- `clippy::all`, `clippy::pedantic`, `clippy::nursery` 有効。ゼロ警告必須。
- 全 pub 型に `#[derive(Debug)]`。全状態構造体に `#[derive(Serialize, Deserialize)]`。
- 決定論的 RNG: `rand_pcg::Pcg64`。

### テスト
- `core` ロジックには Unit テスト必須。`cargo test` で CI/headless 動作。
- 既存テスト（19件）の削除・無効化禁止。

### セーブ/ロード
- `serde_json` で JSON 形式。将来的にローカル暗号化に差し替え可能な抽象化。

### 運用
- 外部サーバー・クラウド DB への通信コード禁止。
- 既存コメント・docstring は変更対象でない限り保持。
- 作業は小さく区切り、`cargo clippy` と `cargo test` を実行してから終了。

---

## ドメインモデル要約

| 概念 | 詳細 |
|------|------|
| 冒険者 | Active/Injured/Retired/Fallen。7属性、11種スキル。40歳〜減衰、70-80代老衰 |
| クエスト | 難易度 E〜S。Kingdom/MerchantGuild/Civilian 3勢力。大成功/成功/失敗/大失敗 |
| パーティ | 最大10名。スキル互換性シナジー |
| 月次監査 | 30日周期。上納金＋実績。3連続未達でゲームオーバー |
| ギルドランク | F→E→D→C→B→A→S。S到達でクリア |
| 収入 | クエスト仲介手数料 + 戦利品売却マージン（冒険者無給） |
| 職員 | 引退教官・受付係等は給与制（契約金 + 日給/月給） |
| 周回 | `MetaState` でアンロック情報を全周回蓄積 |

---

## ファイル構成

```
src/core/ — ゲームロジック（GUI非依存）
  mod.rs, state.rs, adventurer.rs, quest.rs, party.rs,
  inventory.rs, finance.rs, audit.rs, guild_rank.rs, training.rs, meta.rs

src/ui/ — 画面描画（egui依存）
  mod.rs, config.rs, font.rs, components/, views/

src/app.rs — GuildApp (eframe::App)
src/main.rs — エントリポイント
```
