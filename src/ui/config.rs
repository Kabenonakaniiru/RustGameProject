use eframe::egui;
use serde::{Deserialize, Serialize};

/// アプリケーション全体のタイトル
pub const APP_TITLE: &str = "冒険者ギルド経営シミュレーション";

/// 設定ファイルのデフォルトパス
pub const SETTINGS_FILE_PATH: &str = "user_settings.json";

/// ウィンドウの標準サイズ
pub const DEFAULT_WINDOW_WIDTH: f32 = 1400.0;
pub const DEFAULT_WINDOW_HEIGHT: f32 = 800.0;

/// ウィンドウの最小サイズ（レスポンシブ化により小型ディスプレイにも対応）
pub const MIN_WINDOW_WIDTH: f32 = 800.0;
pub const MIN_WINDOW_HEIGHT: f32 = 520.0;

/// ウィンドウ解像度プリセット
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum WindowSizePreset {
    /// 小型ディスプレイ・ノートPC向け (1024 x 640, 16:10)
    Compact,
    /// 標準推奨サイズ (1400 x 800, 7:4)
    Standard,
    /// ワイド高解像度 (1600 x 900, 16:9)
    Large,
    /// フルHD (1920 x 1080, 16:9)
    FullHd,
    /// 任意のカスタム解像度
    Custom(f32, f32),
}

impl WindowSizePreset {
    pub const ALL: [WindowSizePreset; 4] = [
        WindowSizePreset::Compact,
        WindowSizePreset::Standard,
        WindowSizePreset::Large,
        WindowSizePreset::FullHd,
    ];

    /// プリセットの幅と高さを取得
    pub fn size(&self) -> [f32; 2] {
        match self {
            WindowSizePreset::Compact => [1024.0, 640.0],
            WindowSizePreset::Standard => [DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT],
            WindowSizePreset::Large => [1600.0, 900.0],
            WindowSizePreset::FullHd => [1920.0, 1080.0],
            WindowSizePreset::Custom(w, h) => [*w, *h],
        }
    }

    /// 表示用ラベル
    pub fn label(&self) -> &'static str {
        match self {
            WindowSizePreset::Compact => "コンパクト (1024x640)",
            WindowSizePreset::Standard => "標準 (1400x800)",
            WindowSizePreset::Large => "ラージ (1600x900)",
            WindowSizePreset::FullHd => "フルHD (1920x1080)",
            WindowSizePreset::Custom(_, _) => "カスタム解像度",
        }
    }

    /// アスペクト比 (幅 / 高さ)
    pub fn aspect_ratio(&self) -> f32 {
        let [w, h] = self.size();
        if h > 0.0 {
            w / h
        } else {
            1.0
        }
    }

    /// 文字列からのパース（環境変数等で使用）
    pub fn from_str_name(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "compact" | "small" => Some(WindowSizePreset::Compact),
            "standard" | "default" => Some(WindowSizePreset::Standard),
            "large" => Some(WindowSizePreset::Large),
            "fullhd" | "1080p" => Some(WindowSizePreset::FullHd),
            _ => None,
        }
    }
}

/// ユーザー環境設定（ウィンドウサイズ・表示・音量・ゲーム設定など、将来の拡張に対応）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserSettings {
    /// 選択中のウィンドウサイズプリセット
    pub window_preset: WindowSizePreset,
    /// フルスクリーン表示フラグ
    pub fullscreen: bool,
    /// UIズーム倍率 (0.7 ~ 2.0)
    pub zoom_factor: f32,

    // === 将来拡張用設定スロット ===
    /// BGM音量 (0 ~ 100)
    pub bgm_volume: u8,
    /// SE（効果音）音量 (0 ~ 100)
    pub se_volume: u8,
    /// ミュートフラグ
    pub is_muted: bool,
    /// オートセーブ間隔（日数、0で無効）
    pub auto_save_interval_days: u32,
    /// ダーク/ライト表示テーマ
    pub dark_mode: bool,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            window_preset: WindowSizePreset::Standard,
            fullscreen: false,
            zoom_factor: 1.0,
            bgm_volume: 80,
            se_volume: 80,
            is_muted: false,
            auto_save_interval_days: 10,
            dark_mode: true,
        }
    }
}

impl UserSettings {
    /// 設定内容を egui コンテキストおよび viewport に即時適用
    pub fn apply_to_context(&self, ctx: &egui::Context) {
        ctx.set_zoom_factor(self.zoom_factor);
        let [w, h] = self.window_preset.size();
        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(w, h)));
        ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(self.fullscreen));
    }

    /// 設定をローカルJSONファイルに保存
    pub fn save_to_file(&self, path: &str) -> Result<(), String> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("設定のシリアライズに失敗: {}", e))?;
        std::fs::write(path, json)
            .map_err(|e| format!("設定ファイルの書き込みに失敗 ({}): {}", path, e))?;
        Ok(())
    }

    /// ローカルJSONファイルから設定を読み込み（存在しない場合や破損時はデフォルト）
    pub fn load_from_file_or_default(path: &str) -> Self {
        if let Ok(content) = std::fs::read_to_string(path) {
            if let Ok(settings) = serde_json::from_str::<UserSettings>(&content) {
                return settings;
            }
        }
        Self::default()
    }
}

/// ウィンドウおよびGUI起動設定
#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub title: &'static str,
    pub default_size: [f32; 2],
    pub min_size: [f32; 2],
    pub decorations: bool,
    pub resizable: bool,
    pub fullscreen: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: APP_TITLE,
            default_size: WindowSizePreset::Standard.size(),
            min_size: [MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT],
            decorations: true,
            resizable: true,
            fullscreen: false,
        }
    }
}

impl WindowConfig {
    /// 新しいウィンドウ設定を生成
    pub fn new(title: &'static str, width: f32, height: f32) -> Self {
        Self {
            title,
            default_size: [width, height],
            min_size: [MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT],
            decorations: true,
            resizable: true,
            fullscreen: false,
        }
    }

    /// プリセットからウィンドウ設定を生成
    pub fn from_preset(preset: WindowSizePreset) -> Self {
        Self {
            default_size: preset.size(),
            ..Default::default()
        }
    }

    /// ユーザー環境設定からウィンドウ設定を生成
    pub fn from_user_settings(settings: &UserSettings) -> Self {
        Self {
            title: APP_TITLE,
            default_size: settings.window_preset.size(),
            min_size: [MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT],
            decorations: true,
            resizable: true,
            fullscreen: settings.fullscreen,
        }
    }

    /// 環境変数からウィンドウ設定を読み込み（未指定時はデフォルト）
    pub fn from_env() -> Self {
        Self::from_settings_and_env(&UserSettings::default())
    }

    /// 保存済みユーザー設定を基本値とし、環境変数指定があれば上書きしてウィンドウ設定を生成
    pub fn from_settings_and_env(settings: &UserSettings) -> Self {
        Self::from_settings_and_vars(settings, |key| std::env::var(key).ok())
    }

    /// 変数取得関数からウィンドウ設定を構築（テストおよび外部設定用）
    pub fn from_vars<F>(get_var: F) -> Self
    where
        F: Fn(&str) -> Option<String>,
    {
        Self::from_settings_and_vars(&UserSettings::default(), get_var)
    }

    /// 設定と変数取得関数から構築（テスト可能）
    pub fn from_settings_and_vars<F>(settings: &UserSettings, get_var: F) -> Self
    where
        F: Fn(&str) -> Option<String>,
    {
        let mut config = Self::from_user_settings(settings);

        // プリセット指定の確認
        if let Some(preset_str) = get_var("GUILD_WINDOW_PRESET") {
            if let Some(preset) = WindowSizePreset::from_str_name(&preset_str) {
                config.default_size = preset.size();
            }
        }

        // 個別幅・高さ指定の確認（プリセット指定より優先）
        let w_res = get_var("GUILD_WINDOW_WIDTH").and_then(|v| v.parse::<f32>().ok());
        let h_res = get_var("GUILD_WINDOW_HEIGHT").and_then(|v| v.parse::<f32>().ok());
        if let (Some(w), Some(h)) = (w_res, h_res) {
            config.default_size = [w.max(MIN_WINDOW_WIDTH), h.max(MIN_WINDOW_HEIGHT)];
        }

        // フルスクリーン指定
        if let Some(fs) = get_var("GUILD_FULLSCREEN") {
            let fs_lower = fs.to_ascii_lowercase();
            if fs_lower == "1" || fs_lower == "true" {
                config.fullscreen = true;
            } else if fs_lower == "0" || fs_lower == "false" {
                config.fullscreen = false;
            }
        }

        config
    }

    /// サイズ変更可否を設定
    pub fn with_resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// フルスクリーン可否を設定
    pub fn with_fullscreen(mut self, fullscreen: bool) -> Self {
        self.fullscreen = fullscreen;
        self
    }

    /// eframe 起動用の NativeOptions を生成
    pub fn to_native_options(&self) -> eframe::NativeOptions {
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_title(self.title)
                .with_decorations(self.decorations)
                .with_resizable(self.resizable)
                .with_fullscreen(self.fullscreen)
                .with_inner_size(self.default_size)
                .with_min_inner_size(self.min_size),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_window_size_presets() {
        let compact = WindowSizePreset::Compact;
        assert_eq!(compact.size(), [1024.0, 640.0]);
        assert!((compact.aspect_ratio() - 1.6).abs() < 0.01);

        let standard = WindowSizePreset::Standard;
        assert_eq!(standard.size(), [1400.0, 800.0]);
        assert!((standard.aspect_ratio() - 1.75).abs() < 0.01);

        let large = WindowSizePreset::Large;
        assert_eq!(large.size(), [1600.0, 900.0]);
        assert!((large.aspect_ratio() - (16.0 / 9.0)).abs() < 0.01);

        let fullhd = WindowSizePreset::FullHd;
        assert_eq!(fullhd.size(), [1920.0, 1080.0]);

        assert_eq!(WindowSizePreset::from_str_name("compact"), Some(WindowSizePreset::Compact));
        assert_eq!(WindowSizePreset::from_str_name("fullhd"), Some(WindowSizePreset::FullHd));
        assert_eq!(WindowSizePreset::from_str_name("unknown"), None);
    }

    #[test]
    fn test_window_config_preset_and_options() {
        let config = WindowConfig::from_preset(WindowSizePreset::Large)
            .with_resizable(true)
            .with_fullscreen(false);

        assert_eq!(config.default_size, [1600.0, 900.0]);
        assert!(config.resizable);
        assert!(!config.fullscreen);

        let options = config.to_native_options();
        assert_eq!(options.viewport.title, Some(APP_TITLE.to_string()));
    }

    #[test]
    fn test_window_config_from_vars() {
        // 個別指定テスト
        let mut vars = HashMap::new();
        vars.insert("GUILD_WINDOW_WIDTH", "1200".to_string());
        vars.insert("GUILD_WINDOW_HEIGHT", "700".to_string());
        vars.insert("GUILD_FULLSCREEN", "true".to_string());
        let config = WindowConfig::from_vars(|k| vars.get(k).cloned());
        assert_eq!(config.default_size, [1200.0, 700.0]);
        assert!(config.fullscreen);

        // プリセット指定テスト
        let mut vars2 = HashMap::new();
        vars2.insert("GUILD_WINDOW_PRESET", "compact".to_string());
        let config_preset = WindowConfig::from_vars(|k| vars2.get(k).cloned());
        assert_eq!(config_preset.default_size, [1024.0, 640.0]);
        assert!(!config_preset.fullscreen);
    }

    #[test]
    fn test_user_settings_roundtrip() {
        let mut settings = UserSettings::default();
        settings.window_preset = WindowSizePreset::Large;
        settings.zoom_factor = 1.2;
        settings.bgm_volume = 65;
        settings.fullscreen = true;

        let json = serde_json::to_string(&settings).expect("serialize failed");
        let restored: UserSettings = serde_json::from_str(&json).expect("deserialize failed");

        assert_eq!(restored.window_preset, WindowSizePreset::Large);
        assert!((restored.zoom_factor - 1.2).abs() < 0.001);
        assert_eq!(restored.bgm_volume, 65);
        assert!(restored.fullscreen);
    }

    #[test]
    fn test_user_settings_default_values() {
        let defaults = UserSettings::default();

        assert_eq!(defaults.window_preset, WindowSizePreset::Standard);
        assert!(!defaults.fullscreen);
        assert!((defaults.zoom_factor - 1.0).abs() < 0.001);
        assert_eq!(defaults.bgm_volume, 80);
        assert_eq!(defaults.se_volume, 80);
        assert!(!defaults.is_muted);
        assert_eq!(defaults.auto_save_interval_days, 10);
        assert!(defaults.dark_mode);
    }

    #[test]
    fn test_user_settings_file_save_and_load() {
        let test_path = "test_user_settings_tmp.json";

        // テスト用にカスタム設定を作成
        let mut settings = UserSettings::default();
        settings.window_preset = WindowSizePreset::Compact;
        settings.zoom_factor = 1.5;
        settings.dark_mode = false;
        settings.auto_save_interval_days = 5;
        settings.se_volume = 40;
        settings.is_muted = true;

        // 保存
        settings.save_to_file(test_path).expect("save should succeed");

        // 読み込み
        let loaded = UserSettings::load_from_file_or_default(test_path);
        assert_eq!(loaded.window_preset, WindowSizePreset::Compact);
        assert!((loaded.zoom_factor - 1.5).abs() < 0.001);
        assert!(!loaded.dark_mode);
        assert_eq!(loaded.auto_save_interval_days, 5);
        assert_eq!(loaded.se_volume, 40);
        assert!(loaded.is_muted);

        // テストファイル削除
        let _ = std::fs::remove_file(test_path);
    }

    #[test]
    fn test_user_settings_load_missing_file_returns_default() {
        let loaded = UserSettings::load_from_file_or_default("nonexistent_settings_file.json");
        let defaults = UserSettings::default();
        assert_eq!(loaded, defaults);
    }

    #[test]
    fn test_user_settings_load_corrupted_file_returns_default() {
        let test_path = "test_corrupted_settings_tmp.json";
        std::fs::write(test_path, "{ not valid json }}}").expect("write failed");

        let loaded = UserSettings::load_from_file_or_default(test_path);
        let defaults = UserSettings::default();
        assert_eq!(loaded, defaults);

        let _ = std::fs::remove_file(test_path);
    }

    #[test]
    fn test_window_config_from_user_settings() {
        let mut settings = UserSettings::default();
        settings.window_preset = WindowSizePreset::FullHd;
        settings.fullscreen = true;

        let config = WindowConfig::from_user_settings(&settings);
        assert_eq!(config.default_size, [1920.0, 1080.0]);
        assert!(config.fullscreen);
        assert_eq!(config.title, APP_TITLE);
    }
}
