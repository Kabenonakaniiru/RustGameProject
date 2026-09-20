use crate::core::adventurer::AdventurerStatus;
use crate::core::GameState;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, state: &mut GameState) {
    ui.heading("⚔ 冒険者名簿（スキル制・酒場・宿舎）");
    ui.separator();
    ui.label("ギルドに登録されている冒険者の一覧です。職業制ではなくスキル制で成長し、年齢を重ねます。");
    ui.add_space(8.0);

    egui::Grid::new("adventurers_grid")
        .striped(true)
        .min_col_width(80.0)
        .show(ui, |ui| {
            ui.strong("冒険者名");
            ui.strong("ランク");
            ui.strong("年齢");
            ui.strong("得意領域");
            ui.strong("総合戦闘力");
            ui.strong("習得スキル");
            ui.strong("状態");
            ui.end_row();

            for adv in &state.adventurers {
                ui.label(&adv.name);
                ui.label(&adv.rank);
                ui.label(format!("{}歳", adv.age));
                ui.label(adv.primary_role());
                ui.label(format!("{}", adv.total_combat_power()));

                // スキル一覧（カンマ区切り）
                let skills_str = adv
                    .skills
                    .iter()
                    .map(|s| format!("{}:Lv{}", s.skill_type.name(), s.level))
                    .collect::<Vec<_>>()
                    .join(", ");
                ui.label(skills_str);

                // 状態表示と色分け
                let status_text = adv.status.display_name();
                match &adv.status {
                    AdventurerStatus::Standby => {
                        ui.colored_label(egui::Color32::from_rgb(100, 220, 100), status_text);
                    }
                    AdventurerStatus::OnQuest { .. } => {
                        ui.colored_label(egui::Color32::from_rgb(255, 180, 50), status_text);
                    }
                    AdventurerStatus::Injured { .. } => {
                        ui.colored_label(egui::Color32::from_rgb(230, 80, 80), status_text);
                    }
                    AdventurerStatus::Retired => {
                        ui.colored_label(egui::Color32::from_rgb(180, 180, 180), status_text);
                    }
                    AdventurerStatus::Fallen { .. } => {
                        ui.colored_label(egui::Color32::from_rgb(150, 50, 50), status_text);
                    }
                    AdventurerStatus::Training => {
                        ui.colored_label(egui::Color32::from_rgb(100, 180, 255), status_text);
                    }
                }
                ui.end_row();
            }
        });
}
