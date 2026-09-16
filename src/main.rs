pub mod app;
pub mod core;
pub mod ui;

use app::GuildApp;
use eframe::egui;

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/ipag.ttf")).into(),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());
    ctx.set_fonts(fonts);
}

fn main() -> eframe::Result<()> {
    // VS Code DevContainer / WSLg 環境で Wayland ソケットの接続エラーを回避するため X11 バックエンドを優先
    if std::env::var("DISPLAY").is_ok() {
        unsafe {
            std::env::remove_var("WAYLAND_DISPLAY");
            std::env::set_var("WINIT_UNIX_BACKEND", "x11");
        }
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("冒険者ギルド経営シミュレーション")
            .with_decorations(true)
            .with_inner_size([1400.0, 800.0])
            .with_min_inner_size([1000.0, 650.0]),
        ..Default::default()
    };

    eframe::run_native(
        "冒険者ギルド経営シミュレーション",
        options,
        Box::new(|cc| {
            setup_fonts(&cc.egui_ctx);
            Ok(Box::new(GuildApp::new()))
        }),
    )
}
