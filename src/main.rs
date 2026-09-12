use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("経営シミュレーション"),
        ..Default::default()
    };

    eframe::run_native(
        "経営シミュレーション",
        options,
        Box::new(|cc| {
            // 日本語フォントを適用
            setup_fonts(&cc.egui_ctx);
            Ok(Box::new(MyApp::default()))
        }),
    )
}

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // インストールした日本語フォントを読み込む
    if let Ok(font_data) = std::fs::read("/usr/share/fonts/truetype/fonts-japanese-gothic.ttf") {
        fonts.font_data.insert(
            "JapaneseFont".to_owned(),
            egui::FontData::from_owned(font_data),
        );

        // プロポーショナルフォントの優先順位の先頭に追加
        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "JapaneseFont".to_owned());

        // 等幅フォントにも追加
        fonts
            .families
            .get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .push("JapaneseFont".to_owned());

        ctx.set_fonts(fonts);
    }
}

#[derive(Default)]
struct MyApp;

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("経営シミュレーション");
        ui.label("ウィンドウの表示と日本語の描画に成功しました！");
    }
}
