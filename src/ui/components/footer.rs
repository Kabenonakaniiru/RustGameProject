use crate::core::GameState;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, state: &GameState) {
    egui::Frame::side_top_panel(ui.style().as_ref())
        .fill(egui::Color32::from_rgb(20, 22, 28))
        .inner_margin(egui::Margin::symmetric(16, 8))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.strong("📜 ギルド業務日誌 / イベント速報");
                ui.label(format!("（全 {} 件）", state.logs.len()));
            });

            ui.add_space(4.0);

            egui::ScrollArea::vertical()
                .max_height(90.0)
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    for log in state.logs.iter().rev() {
                        ui.label(log);
                    }
                });
        });
}
