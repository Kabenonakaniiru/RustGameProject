use crate::core::GameState;
use eframe::egui;

pub enum HeaderAction {
    None,
    AdvanceDay,
    Save,
    Load,
}

pub fn render(ui: &mut egui::Ui, state: &GameState, status_msg: Option<&str>) -> HeaderAction {
    let mut action = HeaderAction::None;

    egui::Frame::side_top_panel(ui.style().as_ref())
        .fill(egui::Color32::from_rgb(25, 27, 34))
        .inner_margin(egui::Margin::symmetric(16, 10))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("🏰 冒険者ギルド経営日誌");

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .button(egui::RichText::new("⏩ 1日進める").strong())
                        .clicked()
                    {
                        action = HeaderAction::AdvanceDay;
                    }

                    if ui.button("💾 セーブ").clicked() {
                        action = HeaderAction::Save;
                    }

                    if ui.button("📂 ロード").clicked() {
                        action = HeaderAction::Load;
                    }

                    if let Some(msg) = status_msg {
                        ui.colored_label(egui::Color32::from_rgb(120, 220, 120), msg);
                    }
                });
            });

            ui.add_space(6.0);
            ui.separator();
            ui.add_space(6.0);

            // ステータスサマリーバー
            ui.horizontal_wrapped(|ui| {
                ui.label(format!("📅 経過日数: 第 {} 日", state.day));
                ui.separator();

                let gold_color = if state.gold >= 0 {
                    egui::Color32::from_rgb(255, 215, 0)
                } else {
                    egui::Color32::from_rgb(255, 80, 80)
                };
                ui.colored_label(gold_color, format!("💰 ギルド資金: {} G", state.gold));
                ui.separator();

                ui.label(format!("🎖️ ギルドランク: {}", state.rank));
                ui.separator();

                ui.label(format!("⭐ 名声値: {} pt", state.reputation));
                ui.separator();

                ui.colored_label(
                    egui::Color32::from_rgb(255, 120, 120),
                    format!("📉 日次固定支出: -{} G/日", state.daily_total_expenses()),
                );
            });
        });

    action
}
