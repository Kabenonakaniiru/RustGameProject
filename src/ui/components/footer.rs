use crate::core::GameState;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, state: &GameState) {
    let screen_size = ui.input(|i| i.raw.screen_rect).map(|r| r.size());

    egui::Frame::side_top_panel(ui.style().as_ref())
        .fill(egui::Color32::from_rgb(20, 22, 28))
        .inner_margin(egui::Margin::symmetric(14, 6))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.strong("📜 ギルド業務日誌 / イベント速報");
                ui.label(format!("（全 {} 件）", state.logs.len()));

                if let Some(size) = screen_size {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.small(format!("🖥️ ウィンドウサイズ: {:.0} x {:.0}", size.x, size.y));
                    });
                }
            });

            ui.add_space(2.0);

            egui::ScrollArea::vertical()
                .max_height(85.0)
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    for log in state.logs.iter().rev() {
                        ui.label(log);
                    }
                });
        });
}
