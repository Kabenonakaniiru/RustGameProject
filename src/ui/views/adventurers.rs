use crate::core::GameState;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, state: &mut GameState) {
    ui.heading("⚔ 冒険者管理（酒場・宿舎）");
    ui.separator();
    ui.label("ギルドに登録されている冒険者の一覧です。");
    ui.add_space(8.0);

    egui::Grid::new("adventurers_grid")
        .striped(true)
        .min_col_width(100.0)
        .show(ui, |ui| {
            ui.strong("冒険者名");
            ui.strong("ランク");
            ui.strong("職業");
            ui.strong("Lv");
            ui.strong("状態");
            ui.end_row();

            for adv in &state.adventurers {
                ui.label(&adv.name);
                ui.label(&adv.rank);
                ui.label(&adv.class_name);
                ui.label(format!("{}", adv.level));
                if adv.status == "待機中" {
                    ui.colored_label(egui::Color32::from_rgb(100, 220, 100), &adv.status);
                } else {
                    ui.colored_label(egui::Color32::from_rgb(255, 180, 50), &adv.status);
                }
                ui.end_row();
            }
        });
}
