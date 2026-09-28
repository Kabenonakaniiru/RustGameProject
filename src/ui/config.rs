use eframe::egui;

/// アプリケーション全体のタイトル
pub const APP_TITLE: &str = "冒険者ギルド経営シミュレーション";

/// ウィンドウの標準サイズ
pub const DEFAULT_WINDOW_WIDTH: f32 = 1400.0;
pub const DEFAULT_WINDOW_HEIGHT: f32 = 800.0;

/// ウィンドウの最小サイズ
pub const MIN_WINDOW_WIDTH: f32 = 1000.0;
pub const MIN_WINDOW_HEIGHT: f32 = 650.0;

/// ウィンドウおよびGUI起動設定
#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub title: &'static str,
    pub default_size: [f32; 2],
    pub min_size: [f32; 2],
    pub decorations: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: APP_TITLE,
            default_size: [DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT],
            min_size: [MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT],
            decorations: true,
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
        }
    }

    /// eframe 起動用の NativeOptions を生成
    pub fn to_native_options(&self) -> eframe::NativeOptions {
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_title(self.title)
                .with_decorations(self.decorations)
                .with_inner_size(self.default_size)
                .with_min_inner_size(self.min_size),
            ..Default::default()
        }
    }
}
