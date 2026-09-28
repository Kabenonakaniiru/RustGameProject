pub mod app;
pub mod core;
pub mod ui;

use app::GuildApp;
use ui::config::WindowConfig;
use ui::font;

fn main() -> eframe::Result<()> {
    // VS Code DevContainer / WSLg 環境で Wayland ソケットの接続エラーを回避するため X11 バックエンドを優先
    if std::env::var("DISPLAY").is_ok() {
        unsafe {
            std::env::remove_var("WAYLAND_DISPLAY");
            std::env::set_var("WINIT_UNIX_BACKEND", "x11");
        }
    }

    let config = WindowConfig::default();
    let options = config.to_native_options();

    eframe::run_native(
        config.title,
        options,
        Box::new(|cc| {
            font::setup_fonts(&cc.egui_ctx);
            Ok(Box::new(GuildApp::new()))
        }),
    )
}

