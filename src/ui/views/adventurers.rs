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
            ui.strong("所属部隊");
            ui.strong("得意領域");
            ui.strong("総合戦闘力");
            ui.strong("習得スキル");
            ui.strong("状態");
            ui.end_row();

            for adv in &state.adventurers {
                ui.label(&adv.name);
                ui.label(&adv.rank);
                ui.label(format!("{}歳", adv.age));

                // 所属部隊の表示
                let party_name = state
                    .get_adventurer_party(adv.id).map_or_else(|| "（無所属）".to_string(), |p| format!("『{}』", p.name));
                if party_name == "（無所属）" {
                    ui.colored_label(egui::Color32::from_rgb(160, 160, 160), party_name);
                } else {
                    ui.colored_label(egui::Color32::from_rgb(140, 200, 255), party_name);
                }

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

    // 殿堂入り・追悼録（戦死・老衰で永眠した冒険者たち）
    if !state.hall_of_fame.is_empty() {
        ui.add_space(16.0);
        ui.separator();
        ui.heading("🏛 栄誉の殿堂・追悼録（ギルドに貢献した英傑たち）");
        ui.label("任務にて散った冒険者や、天寿を全うした元教官たちの永久記録です。");
        ui.add_space(4.0);

        egui::Grid::new("hall_of_fame_grid")
            .striped(true)
            .min_col_width(80.0)
            .show(ui, |ui| {
                ui.strong("氏名");
                ui.strong("最終ランク");
                ui.strong("享年");
                ui.strong("事由・功績");
                ui.strong("記録日");
                ui.end_row();

                for hof in &state.hall_of_fame {
                    ui.label(&hof.name);
                    ui.label(&hof.final_rank);
                    ui.label(format!("{}歳", hof.age));
                    ui.colored_label(egui::Color32::from_rgb(200, 180, 140), &hof.reason);
                    ui.label(format!("第{}日", hof.recorded_day));
                    ui.end_row();
                }
            });
    }
}
