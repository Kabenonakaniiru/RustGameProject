# 実装履歴アーカイブ (CHANGELOG.md)

本ファイルは `AGENTS.md` から切り出された過去の完了フェーズおよび実装詳細の記録です。

---

## Phase 1（2026-09-17 完了）
* [x] **依存関係の追加**: `serde`, `serde_json` を導入。
* [x] **`core` モジュールの切り出し**:
  * `src/core/` 配下に `adventurer.rs`, `inventory.rs`, `quest.rs`, `finance.rs`, `state.rs`, `mod.rs` を抽出。
  * 完全 GUI 非依存のゲームロジックおよび JSON セーブ/ロード（`save_game`, `load_game`）を実装。
* [x] **Unit テストの整備**:
  * 日付進行・固定費引き落とし、アイテム売却、クエスト派遣/帰還、JSONセーブ/ロード往復の5件のテストを実装、すべてパス。
* [x] **`ui` モジュールおよび `app.rs` の整理**:
  * ヘッダー、フッター、6つの各タブ画面を `src/ui/` に完全分離。
  * `src/main.rs` はエントリポイント（50行程度）にスリム化。

---

## Phase 2（2026-09-20 完了）
* [x] **ドメインモデル大幅拡張**:
  * `src/core/party.rs`: `Party` 構造体、最大4人（後に10人へ拡張）編成、スキル互換性によるシナジーボーナス計算。
  * `src/core/training.rs`: `TrainingDojo`、教官スロット（`InstructorSlot`）、成長率バフ計算。
  * `src/core/meta.rs`: `MetaState`（周回用）、`HallOfFameAdventurer`（殿堂記録）、`UnlockedFacility`/`UnlockedSecret`。
  * `src/core/adventurer.rs`: `AdventurerStatus`（Active/Injured/Retired/Fallen）、`Stats`（7属性）、`SkillType`（11種）、年齢減衰（40歳〜）、`DeathCause`。
  * `src/core/quest.rs`: `ClientFaction`（Kingdom/MerchantGuild/Civilian）、`QuestDifficulty`（E〜S）、`QuestResult`（大成功〜大失敗）、派遣/判定ロジック。
  * `src/core/state.rs`: `faction_trust`（勢力信頼度マップ）、`Pcg64` RNG、`advance_day` 拡張（加齢・老衰・殿堂登録）、`apply_quest_result`、カレンダー表示。
* [x] **決定論的乱数（RNG）基盤の導入**: `rand` / `rand_pcg`（`Pcg64`）によるシード再現性。
* [x] **UI の新型対応**: ヘッダー（カレンダー表示）、冒険者画面（ステータス・年齢・スキル）、概要画面（勢力信頼度）、クエスト画面（難易度・勢力・状態）。

---

## Phase 3（2026-09-27〜 完了項目）
* [x] **月次監査システムの実装（2026-09-27）**:
  * `src/core/audit.rs`: 30日周期での上納金・クエスト実績判定、連続3回未達による認可取消ゲームオーバー処理。
  * `src/core/state.rs`: `advance_day` への月次監査イベント統合、上納金引き落とし・経費計上。
* [x] **ギルドランク昇格システムの実装（2026-09-27）**:
  * `src/core/guild_rank.rs`: F〜Sランク定義、名声・勢力信頼度・クエスト実績・日数による自動昇格判定、Sランク到達時のゲームクリア処理。
* [x] **UI表示の対応（2026-09-27）**:
  * `src/ui/components/header.rs`: ギルドランク表示、次回監査までの残り日数カウントダウン、ゲームオーバー/クリア状態表示。
  * `src/ui/views/overview.rs`: 月次監査進捗（今期達成数・上納金・連続未達回数）、次期ランク昇格要件と進捗の可視化。
* [x] **UI基盤・エントリポイントのリファクタリング（2026-09-29）**:
  * `src/ui/config.rs`: ウィンドウ設定（`WindowConfig`）およびタイトル定数（`APP_TITLE`）、初期・最小画面サイズ定数の定義。
  * `src/ui/font.rs`: フォント登録（`FontManager` / `setup_fonts`）のモジュール化・カプセル化。
  * `src/ui/mod.rs` & `src/app.rs`: `Tab` enum へのラベル定義（`label()`、`Tab::ALL`）追加とメニュー描画のループ化。
  * `src/main.rs`: 設定とフォント処理の委譲によりエントリポイントを最小構成にスリム化。
* [x] **画面サイズ指定の自由化・調整機能およびレスポンシブ対応（2026-09-30）**:
  * `src/ui/config.rs`: `WindowSizePreset`（Compact/Standard/Large/FullHd/Custom）、アスペクト比計算、最小サイズ（800x520）緩和、環境変数からの設定読み込み。
  * `src/app.rs`: 動的ズームスケーリング（0.7x〜2.0x）、境界ドラッグによるリサイズ可能ナビゲーションパネル（150〜300px）＆フッター（55〜250px）。
  * `src/ui/components/header.rs` & `footer.rs`: 狭幅時の自動縦並びレイアウト、ズーム調整ボタン、解像度インジケーター表示。
  * `src/ui/views/overview.rs` & `quests.rs`: 画面幅（700px）に応じた1列/2列自動切り替え、クエスト情報の折り返し対応。
* [x] **環境設定メニューの実装（2026-10-05）**:
  * `src/ui/config.rs`: `UserSettings` 構造体（`serde` 対応）、解像度・フルスクリーン・ズーム・音量・オートセーブ設定の永続化（`user_settings.json`）。
  * `src/ui/components/settings.rs`: モーダル設定画面。`SettingsAction` enum による借用安全性。
  * `src/app.rs`: Escキートグル、左ナビ設定ボタン。
* [x] **部隊（パーティ）編成・指揮およびクエスト結果通知の実装（2026-10-12）**:
  * `src/core/state.rs`: 部隊の結成・解散・配属・除外・出撃保護ロジック。
  * `src/core/quest.rs`: クエスト完了報告書 `QuestReport` 構造体。
  * `src/ui/views/quests.rs`: 成果報告バナー、部隊編成カード、出撃命令、`QuestAction` パターン。
  * `src/ui/views/adventurers.rs`: 部隊列・栄誉の殿堂・追悼録表示。
* [x] **ギルドプロファイル・初期設定およびタイトル画面システムの実装（2026-10-20）**:
  * `src/core/state.rs`: `guild_name`, `master_name`, `location` フィールドおよび `new_with_profile()` コンストラクタ。
  * `src/ui/views/title_screen.rs`: タイトルメニュー（新規開業 / ロード / 設定）。
  * `src/ui/views/setup.rs`: 新規開業フォーム（バリデーション付き）。
  * `src/app.rs`: `AppPhase` enum（TitleScreen / InitialSetup / InGame）による画面遷移管理。
* [x] **DevContainer 日本語入力（IME）環境の完全整備・恒久化（2026-10-25）**:
  * `.devcontainer/Dockerfile`, `devcontainer.json`, `init-ime.sh`: fcitx5-mozc + WSLg Waylandバインド回避（`fcitx5 -d --disable wayland,waylandim`）による標準的・恒久的なIME環境構築。
* [x] **クエスト供給・自動生成 / 冒険者リクルート / 引退・教官雇用フローの実装（2026-11-07）**:
  * `src/core/adventurer.rs`: `generate_recruit()`（名声連動品質ティア判定）。
  * `src/core/state.rs`: `refill_quests`, `maybe_generate_recruit`, `accept_recruit`, `retire_adventurer`, `hire_as_instructor`, `dismiss_instructor`。
  * `src/ui/views/adventurers.rs`: 引退・教官採用・解雇ボタン、教官一覧、志願者一覧。
* [x] **部隊一覧の代表者表示およびアコーディオン展開による一覧性向上（2026-11-09）**:
  * `src/core/party.rs`: `leader_id(&self)`。
  * `src/core/state.rs`: `get_party_leader(&self)`。
  * `src/ui/views/quests.rs`: 👑代表者サマリー行、CollapsingHeader による折りたたみ、派遣ボタンの代表者名表示＆ホバーツールチップ。
* [x] **画面サイズ自動最適化および高解像度（4K/WQHD）対応（2026-11-13）**:
  * `src/ui/config.rs`: `WindowSizePreset` に `Auto`, `Wqhd`, `FourK` 追加、`closest_preset`, `recommended_zoom` 実装。
  * `src/app.rs`: 初回起動時のモニターサイズ検知と自動最適化適用。

---

## Phase 3 継続（2026-11-17）

### 完了
* [x] **訓練所タブ（🥋 訓練所・道場）の実装**:
  * `src/core/training.rs`: `TrainingDojo` に `recent_training_logs` フィールド追加（最大50件）、`is_trainee()` / `is_instructor()` ヘルパー追加、`suggested_instructor_salary()` 追加、`process_daily_training()` を `&mut self` に変更してログ自動蓄積。
  * `src/core/state.rs`: `enroll_trainee()` / `remove_trainee()` メソッド追加（ステータス `Training` ⇔ `Standby` 連動）。老衰永眠時に教官リストからも自動退任する処理を `process_yearly_aging` に追加。
  * `src/ui/views/training.rs`: 新規ビュー。教官一覧・就任候補・訓練生一覧・入門候補・成長ログを含む。
  * `src/ui/mod.rs` / `src/app.rs` / `src/ui/views/mod.rs`: `Tab::TrainingDojo` 追加・routing 接続。
  * Unit テスト 2件追加（計29件全パス）: `test_enroll_and_graduate_trainee`, `test_deceased_instructor_auto_removed`。

### 次フェーズ以降への課題メモ
* [ ] **セーブデータ複数スロット対応（要望あり）**:
  * 現状: `save_data.json` 1ファイル固定（`const SAVE_FILE_PATH` / `has_save_data()` / `save()` / `load()`）。
  * 方針案:
    * セーブスロットを複数（例: スロット1〜5）持てる構造に変更。
    * `save_data_slot1.json` ～ `save_data_slot5.json` のようなファイル名規則、またはサブディレクトリ管理（`saves/slot01/data.json`）。
    * タイトル画面の「ロード」でスロット選択 UI を表示（ギルド名・マスター名・日付・ギルドランクをサマリー表示）。
    * `save()` 時もスロット選択ダイアログか、「上書き確認付き現在スロット保存」とする。
    * `has_save_data()` → `list_save_slots() -> Vec<SaveSlotInfo>` に刷新。
