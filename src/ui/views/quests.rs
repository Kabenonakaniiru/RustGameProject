use crate::core::GameState;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, state: &mut GameState) {
    ui.heading("📋 受付・クエスト管理＆部隊派遣");
    ui.separator();
    ui.label("クライアント勢力（王宮・商業連盟・民間）からの依頼を管理し、編成した部隊（最大10名）を派遣します。");
    ui.add_space(8.0);

    // 派遣可能なパーティの一覧
    let available_parties: Vec<(u64, String, usize, f32)> = state
        .parties
        .iter()
        .filter(|p| {
            !p.member_ids.is_empty()
                && p.member_ids.iter().all(|&id| {
                    state
                        .adventurers
                        .iter()
                        .find(|a| a.id == id)
                        .map(|a| a.status.is_available())
                        .unwrap_or(false)
                })
        })
        .map(|p| (p.id, p.name.clone(), p.member_ids.len(), p.cooperation))
        .collect();

    let mut dispatch_target = None;

    for (i, quest) in state.quests.iter().enumerate() {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.strong(&quest.title);
                ui.label(format!("【危険度: {}】", quest.difficulty));
                ui.colored_label(
                    egui::Color32::from_rgb(140, 200, 255),
                    format!("［{}］", quest.faction.name()),
                );
                ui.label(format!("推奨技能: {}", quest.recommended_skill.name()));
                ui.label(format!("所要日数: {}日", quest.days_required));
                ui.label(format!("報酬目安: {} G", quest.reward_estimate));
            });

            ui.horizontal(|ui| {
                if quest.is_dispatched {
                    ui.colored_label(
                        egui::Color32::from_rgb(255, 180, 50),
                        format!(
                            "⏳ 派遣中 (担当部隊: {}, 残り: {}日)",
                            quest.assigned_party_name.as_deref().unwrap_or("不明"),
                            quest.days_remaining
                        ),
                    );
                } else {
                    ui.label("状態: 受注可能");
                    if !available_parties.is_empty() {
                        for (p_id, p_name, member_count, coop) in &available_parties {
                            if ui
                                .button(format!(
                                    "『{}』({}名/連携{:.0}) を派遣",
                                    p_name, member_count, coop
                                ))
                                .clicked()
                            {
                                dispatch_target = Some((i, *p_id));
                            }
                        }
                    } else {
                        ui.label("（出撃可能な待機中編成部隊がありません）");
                    }
                }
            });
        });
        ui.add_space(4.0);
    }

    if let Some((quest_idx, party_id)) = dispatch_target {
        let _ = state.dispatch_quest_party(quest_idx, party_id);
    }
}
