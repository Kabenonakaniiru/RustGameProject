# GitHub Copilot Instructions — 冒険者ギルド経営シミュレーション

> **正式な設計文書は [`AGENTS.md`](../AGENTS.md) を参照してください。**
> このファイルは Copilot が自動的に読み取るリポジトリレベルの指示書です。

---

## プロジェクト概要

- **ジャンル**: 冒険者ギルド経営シミュレーション（Rust / eframe / egui）
- **ターゲット**: 完全ローカル・オフライン完結のデスクトップアプリ（サーバー依存ゼロ）
- **言語**: Rust 2024 Edition

---

## アーキテクチャ ― 絶対に守るべきルール

### 1. `core/`（ゲームロジック）と `ui/`（描画）の完全分離
- `src/core/` は **純粋な Rust コード**。`egui` や `eframe` への依存を一切持たない。
- `src/ui/` は `eframe::App` のライフサイクルおよび `egui` 描画のみ。`GameState` を受け取って表示し、ユーザー操作を `core` のメソッド呼び出しに変換するだけ。
- **`core` モジュール内で `egui` を `use` したり、UI 型を引数に取ったりしてはならない。**

### 2. 借用チェッカー対策 ― Action パターン
- UI 描画中に `GameState` を `&mut` で借用すると衝突するため、**描画フェーズでは `&self` で読み取り、操作を `Action` enum に蓄積し、描画後にまとめて適用する**パターンを使う。
- 例: `QuestAction`, `SettingsAction`, `HeaderAction`

### 3. セーブ/ロード
- `serde` / `serde_json` で JSON 形式。全状態構造体に `#[derive(Serialize, Deserialize)]` 付与。
- セーブ/ロード処理は抽象化し、将来的にローカル暗号化（AES-256）や HMAC 署名に差し替え可能にする。

### 4. 乱数
- `rand` / `rand_pcg`（`Pcg64`）による決定論的 RNG。シード再現性を保証する。

---

## コーディングスタイル

### Rust イディオム
- `unsafe` コードは **禁止** (`unsafe_code = "deny"`)。
- `clippy::all`, `clippy::pedantic`, `clippy::nursery` を有効化。`cargo clippy --all-targets -- -D warnings` がゼロ警告。
- `#[derive(Debug)]` はすべての `pub` 型に付与。
- Rust 2024 イディオム: `&& let` ガード、`std::io::Error::other`、`uninlined_format_args` 等を積極利用。

### テスト
- `core` ロジックには必ず Unit テストを書く。`#[cfg(test)]` モジュール内に配置。
- `cargo test` で画面なし環境（CI / DevContainer）でも高速動作すること。
- 現在 19 件の Unit テストが全パス。

### コメント・ドキュメント
- 既存のコメント・docstring は変更対象でない限り保持する。
- 日本語コメント OK。

---

## ディレクトリ構成

```text
src/
├── main.rs              # エントリポイント
├── app.rs               # GuildApp（eframe::App実装、タブ遷移）
├── core/                # 【GUI非依存】ゲームドメイン・ロジック
│   ├── mod.rs           # GameState 統括、save/load
│   ├── state.rs         # ゲーム基本状態（Day, Gold, Rank, 等）
│   ├── adventurer.rs    # 冒険者データ・ステータス・スキル
│   ├── quest.rs         # クエストデータ・派遣・完了判定・QuestReport
│   ├── party.rs         # パーティ編成・シナジー計算
│   ├── inventory.rs     # 倉庫アイテム・売却計算
│   ├── finance.rs       # 固定費・人件費・収支台帳
│   ├── audit.rs         # 月次監査システム
│   ├── guild_rank.rs    # ギルドランク昇格（F〜S）
│   ├── training.rs      # 訓練所（道場）・教官システム
│   └── meta.rs          # 周回メタ状態・殿堂
└── ui/                  # 【egui依存】画面描画
    ├── mod.rs
    ├── config.rs         # ウィンドウ設定・UserSettings
    ├── font.rs           # フォント管理
    ├── components/       # ヘッダー、フッター、設定モーダル
    └── views/            # 各タブ画面
```

---

## ドメイン知識（生成時の参考）

- **冒険者**: Active/Injured/Retired/Fallen の4状態。7属性ステータス。11種スキル。40歳〜減衰、70〜80代で老衰。
- **クエスト**: 難易度 E〜S。発注者3勢力（Kingdom/MerchantGuild/Civilian）。結果は大成功/成功/失敗/大失敗。
- **パーティ**: 最大10名。スキル互換性によるシナジーボーナス。
- **月次監査**: 30日周期。上納金＋クエスト実績。連続3回未達でゲームオーバー。
- **ギルドランク**: F→E→D→C→B→A→S。Sランク到達でゲームクリア。
- **収入源**: クエスト仲介手数料 ＋ 戦利品売却マージン（冒険者に固定給なし）。
- **職員**: 引退教官・受付係等は給与制（採用時契約金 + 日給/月給）。

---

## 禁止事項

1. **外部サーバー・クラウドDB への通信コードを生成しない**。完全オフライン・ローカル完結。
2. **`core/` 内で `egui` / `eframe` を import しない**。
3. **`unsafe` ブロックを書かない**。
4. **既存の Unit テストを削除・無効化しない**（追加は歓迎）。
5. **セッションを跨ぐ大きなリファクタリングを一度に行わない**（小さく区切る）。
