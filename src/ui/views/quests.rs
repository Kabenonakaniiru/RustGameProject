use crate::core::GameState;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, state: &mut GameState) {
    ui.heading("📋 受付・クエスト管理＆派遣");
    ui.separator();
    ui.label("冒険者にクエストを斡旋・派遣します。帰還時に仲介手数料と戦利品の買取機会が得られます。");
    ui.add_space(8.0);

    let available_adventurers: Vec<String> = state
        .adventurers
        .iter()
        .filter(|a| a.status == "待機中")
        .map(|a| a.name.clone())
        .collect();

    let mut dispatch_target = None;

    for (i, quest) in state.quests.iter().enumerate() {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.strong(&quest.title);
                ui.label(format!("【危険度: {}】", quest.difficulty));
                ui.label(format!("場所: {}", quest.location));
                ui.label(format!("所要日数: {}日", quest.days_required));
                ui.label(format!("報酬目安: {} G", quest.reward_estimate));
            });

            ui.horizontal(|ui| {
                if quest.is_dispatched {
                    ui.colored_label(
                        egui::Color32::from_rgb(255, 180, 50),
                        format!(
                            "⏳ 派遣中 (担当: {}, 残り: {}日)",
                            quest.assigned_party.as_deref().unwrap_or("不明"),
                            quest.days_remaining
                        ),
                    );
                } else {
                    ui.label("状態: 待機中（未派遣）");
                    if !available_adventurers.is_empty() {
                        let candidate = &available_adventurers[0];
                        if ui.button(format!("{} を派遣する", candidate)).clicked() {
                            dispatch_target = Some((i, candidate.clone()));
                        }
                    } else {
                        ui.label("（派遣可能な待機中冒険者がいません）");
                    }
                }
            });
        });
        ui.add_space(4.0);
    }

    if let Some((idx, candidate)) = dispatch_target {
        state.dispatch_quest(idx, &candidate);
    }
}
