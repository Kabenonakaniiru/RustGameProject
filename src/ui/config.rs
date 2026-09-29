use eframe::egui;

/// アプリケーション全体のタイトル
pub const APP_TITLE: &str = "冒険者ギルド経営シミュレーション";

/// ウィンドウの標準サイズ
pub const DEFAULT_WINDOW_WIDTH: f32 = 1400.0;
pub const DEFAULT_WINDOW_HEIGHT: f32 = 800.0;

/// ウィンドウの最小サイズ（レスポンシブ化により小型ディスプレイにも対応）
pub const MIN_WINDOW_WIDTH: f32 = 800.0;
pub const MIN_WINDOW_HEIGHT: f32 = 520.0;

/// ウィンドウ解像度プリセット
#[derive(Debug, Clone, Copy, PartialEq)]
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

    /// 環境変数からウィンドウ設定を読み込み（未指定時はデフォルト）
    /// - `GUILD_WINDOW_WIDTH`, `GUILD_WINDOW_HEIGHT`: ピクセルサイズ指定
    /// - `GUILD_WINDOW_PRESET`: compact, standard, large, fullhd
    /// - `GUILD_FULLSCREEN`: 1 または true でフルスクリーン
    pub fn from_env() -> Self {
        Self::from_vars(|key| std::env::var(key).ok())
    }

    /// 変数取得関数からウィンドウ設定を構築（テストおよび外部設定用）
    pub fn from_vars<F>(get_var: F) -> Self
    where
        F: Fn(&str) -> Option<String>,
    {
        let mut config = Self::default();

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
}
