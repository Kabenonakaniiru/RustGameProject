use eframe::egui;

pub struct MyApp {
    label_text: String,
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_fonts(&cc.egui_ctx);
        Self {
            label_text: "こんにちは、世界！".to_string(),
        }
    }
}

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // FontData から Arc<FontData> への変換のため .into() を追加
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

impl eframe::App for MyApp {
    // 最新の eframe では update(&mut self, ctx, frame) ではなく ui(&mut self, ui, frame) を実装
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("経営シミュレーション");
        ui.label(&self.label_text);
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("経営シミュレーション")
            .with_decorations(true)
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "経営シミュレーション",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}
