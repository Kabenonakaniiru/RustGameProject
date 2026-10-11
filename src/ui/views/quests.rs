use crate::core::adventurer::Adventurer;
use crate::core::quest::QuestOutcomeType;
use crate::core::GameState;
use eframe::egui;

enum QuestAction {
    Dispatch { quest_idx: usize, party_id: u64 },
    CreateParty(String),
    DisbandParty(u64),
    AddPartyMember { party_id: u64, adv_id: u64 },
    RemovePartyMember { party_id: u64, adv_id: u64 },
    ClearReports,
}

pub fn render(ui: &mut egui::Ui, state: &mut GameState) {
    let mut actions = Vec::new();

    ui.heading("📋 受付・クエスト管理＆部隊編成");
    ui.separator();
    ui.label("部隊（最大10名）を自由に編成・指揮し、各勢力からの依頼へ派遣します。");
    ui.add_space(6.0);

    // =========================================================================
    // 1. 直近のクエスト成果報告（新着リザルト）
    // =========================================================================
    if !state.latest_quest_reports.is_empty() {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.heading(format!(
                    "🎉 クエスト帰還・成果報告（新着 {} 件）",
                    state.latest_quest_reports.len()
                ));
                if ui.button("✔ すべて確認して閉じる").clicked() {
                    actions.push(QuestAction::ClearReports);
                }
            });
            ui.separator();

            for report in &state.latest_quest_reports {
                ui.group(|ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.strong(format!("『{}』", report.quest_title));
                        ui.label(format!("（担当: {}）", report.party_name));

                        let (outcome_color, outcome_text) = match report.outcome {
                            QuestOutcomeType::GreatSuccess => {
                                (egui::Color32::from_rgb(100, 255, 100), "⭐ 大成功")
                            }
                            QuestOutcomeType::Success => {
                                (egui::Color32::from_rgb(120, 200, 255), "⭕ 成功")
                            }
                            QuestOutcomeType::Failure => {
                                (egui::Color32::from_rgb(255, 160, 50), "🔺 失敗")
                            }
                            QuestOutcomeType::Catastrophe => {
                                (egui::Color32::from_rgb(255, 70, 70), "💀 壊滅・大失敗")
                            }
                        };
                        ui.colored_label(outcome_color, outcome_text);
                        ui.label(format!("報酬: +{} G", report.commission));
                        ui.label(format!("名声: +{}", report.fame_gain));
                        if report.trust_change != 0 {
                            let sign = if report.trust_change > 0 { "+" } else { "" };
                            ui.label(format!("信頼度: {}{}", sign, report.trust_change));
                        }
                    });

                    if !report.loot_items.is_empty() {
                        ui.horizontal_wrapped(|ui| {
                            ui.label("📦 獲得戦利品:");
                            for item in &report.loot_items {
                                ui.colored_label(
                                    egui::Color32::from_rgb(220, 200, 140),
                                    format!("［{}］", item),
                                );
                            }
                        });
                    }

                    if !report.injured_names.is_empty() || !report.fallen_names.is_empty() {
                        ui.horizontal_wrapped(|ui| {
                            for (name, days) in &report.injured_names {
                                ui.colored_label(
                                    egui::Color32::from_rgb(255, 180, 50),
                                    format!("🏥 {} (療養{}日)", name, days),
                                );
                            }
                            for name in &report.fallen_names {
                                ui.colored_label(
                                    egui::Color32::from_rgb(255, 80, 80),
                                    format!("🪦 {} (名誉の戦死)", name),
                                );
                            }
                        });
                    }

                    ui.label(format!("💬 {}", report.log_summary));
                });
                ui.add_space(2.0);
            }
        });
        ui.add_space(10.0);
    }

    // =========================================================================
    // 2. 部隊（パーティ）編成・管理セクション
    // =========================================================================
    ui.horizontal(|ui| {
        ui.heading("⚔️ 部隊（パーティ）編成・指揮");
        ui.label(format!("（登録部隊数: {}/8）", state.parties.len()));
    });
    ui.separator();

    // 未配属かつ待機中の冒険者リスト
    let unassigned_adventurers: Vec<&Adventurer> = state
        .adventurers
        .iter()
        .filter(|a| {
            a.status.is_available() && !state.parties.iter().any(|p| p.member_ids.contains(&a.id))
        })
        .collect();

    // 部隊一覧の表示
    for party in &state.parties {
        let is_on_quest = state.is_party_on_quest(party.id);
        let synergy = party.calculate_effective_synergy(&state.adventurers);

        ui.group(|ui| {
            // 部隊ヘッダー行
            ui.horizontal_wrapped(|ui| {
                ui.strong(format!("『{}』", party.name));
                if is_on_quest {
                    ui.colored_label(egui::Color32::from_rgb(255, 180, 50), "⏳ 出撃中");
                } else if party.member_ids.is_empty() {
                    ui.colored_label(egui::Color32::from_rgb(180, 180, 180), "⚪ 待機中（人員なし）");
                } else {
                    ui.colored_label(egui::Color32::from_rgb(100, 220, 100), "🟢 待機中（即時出撃可）");
                }

                ui.label(format!("人数: {}/10名", party.member_ids.len()));
                ui.label(format!("基礎連携: {:.1}", party.cooperation));
                ui.colored_label(
                    egui::Color32::from_rgb(140, 220, 255),
                    format!("実効連携倍率: x{:.2}", synergy),
                );
                ui.label(format!("完了任務数: {}件", party.completed_quests));

                // 解散ボタン（出撃中は無効）
                if !is_on_quest {
                    if ui.button("🗑️ 部隊解散").clicked() {
                        actions.push(QuestAction::DisbandParty(party.id));
                    }
                }
            });

            ui.add_space(4.0);

            // 部隊メンバーリスト
            if party.member_ids.is_empty() {
                ui.label("（所属している冒険者はいません。下の候補から配属してください）");
            } else {
                egui::Grid::new(format!("party_grid_{}", party.id))
                    .striped(true)
                    .min_col_width(70.0)
                    .show(ui, |ui| {
                        ui.strong("冒険者名");
                        ui.strong("ランク");
                        ui.strong("役割");
                        ui.strong("戦闘力");
                        ui.strong("状態");
                        if !is_on_quest {
                            ui.strong("操作");
                        }
                        ui.end_row();

                        for &m_id in &party.member_ids {
                            if let Some(adv) = state.adventurers.iter().find(|a| a.id == m_id) {
                                ui.label(&adv.name);
                                ui.label(&adv.rank);
                                ui.label(adv.primary_role());
                                ui.label(format!("{}", adv.total_combat_power()));
                                ui.label(adv.status.display_name());

                                if !is_on_quest {
                                    if ui.button("❌ 外す").clicked() {
                                        actions.push(QuestAction::RemovePartyMember {
                                            party_id: party.id,
                                            adv_id: adv.id,
                                        });
                                    }
                                }
                                ui.end_row();
                            }
                        }
                    });
            }

            // メンバー追加エリア（出撃中でなく、枠に余裕がある場合）
            if !is_on_quest && party.member_ids.len() < 10 {
                ui.add_space(4.0);
                ui.horizontal_wrapped(|ui| {
                    ui.label("➕ 冒険者を配属:");
                    if unassigned_adventurers.is_empty() {
                        ui.label("（配属可能な未所属冒険者はいません）");
                    } else {
                        for adv in &unassigned_adventurers {
                            let btn_text = format!(
                                "{} ({}・戦力{})",
                                adv.name,
                                adv.primary_role(),
                                adv.total_combat_power()
                            );
                            if ui.button(btn_text).clicked() {
                                actions.push(QuestAction::AddPartyMember {
                                    party_id: party.id,
                                    adv_id: adv.id,
                                });
                            }
                        }
                    }
                });
            }
        });
        ui.add_space(6.0);
    }

    // 新規部隊作成フォーム
    ui.horizontal(|ui| {
        let name_temp_id = ui.id().with("new_party_input");
        let default_name = format!("第{}遊撃部隊", state.parties.len() + 1);
        let mut input_name: String = ui
            .data_mut(|d| d.get_temp(name_temp_id))
            .unwrap_or(default_name);

        ui.label("部隊名:");
        let response = ui.text_edit_singleline(&mut input_name);
        if response.changed() {
            ui.data_mut(|d| d.insert_temp(name_temp_id, input_name.clone()));
        }

        let can_create = state.parties.len() < 8 && !input_name.trim().is_empty();
        if ui
            .add_enabled(can_create, egui::Button::new("➕ 新規部隊を結成"))
            .clicked()
        {
            actions.push(QuestAction::CreateParty(input_name.clone()));
            // 次回用にリセット
            let next_default = format!("第{}遊撃部隊", state.parties.len() + 2);
            ui.data_mut(|d| d.insert_temp(name_temp_id, next_default));
        }
    });

    ui.add_space(14.0);

    // =========================================================================
    // 3. 受付・クエスト一覧＆派遣セクション
    // =========================================================================
    ui.heading("📜 受付・クエスト一覧＆出撃命令");
    ui.separator();

    // 派遣可能なパーティの一覧（メンバーが1人以上いて、全メンバーが待機可能）
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
        .map(|p| {
            let synergy = p.calculate_effective_synergy(&state.adventurers);
            (p.id, p.name.clone(), p.member_ids.len(), synergy)
        })
        .collect();

    for (i, quest) in state.quests.iter().enumerate() {
        ui.group(|ui| {
            // クエスト情報ヘッダー
            ui.horizontal_wrapped(|ui| {
                ui.strong(&quest.title);
                ui.label(format!("【危険度: {}】", quest.difficulty));
                ui.colored_label(
                    egui::Color32::from_rgb(140, 200, 255),
                    format!("［{}］", quest.faction.name()),
                );
                ui.label(format!("場所: {}", quest.location));
                ui.label(format!("推奨技能: {}", quest.recommended_skill.name()));
                ui.label(format!("所要: {}日", quest.days_required));
                ui.colored_label(
                    egui::Color32::from_rgb(255, 215, 0),
                    format!("報酬目安: {} G", quest.reward_estimate),
                );
            });

            ui.add_space(4.0);

            // 派遣操作・ステータス
            ui.horizontal_wrapped(|ui| {
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
                        for (p_id, p_name, count, synergy) in &available_parties {
                            let btn_label = format!(
                                "『{}』({}名/倍率{:.2}) を派遣",
                                p_name, count, synergy
                            );
                            if ui.button(btn_label).clicked() {
                                actions.push(QuestAction::Dispatch {
                                    quest_idx: i,
                                    party_id: *p_id,
                                });
                            }
                        }
                    } else {
                        ui.colored_label(
                            egui::Color32::from_rgb(180, 180, 180),
                            "（出撃可能な待機部隊がありません。部隊に冒険者を配属してください）",
                        );
                    }
                }
            });
        });
        ui.add_space(4.0);
    }

    // =========================================================================
    // 4. アクションの反映（借用チェッカーを回避して順次適用）
    // =========================================================================
    for action in actions {
        match action {
            QuestAction::Dispatch { quest_idx, party_id } => {
                let _ = state.dispatch_quest_party(quest_idx, party_id);
            }
            QuestAction::CreateParty(name) => {
                let _ = state.create_party(name);
            }
            QuestAction::DisbandParty(party_id) => {
                let _ = state.disband_party(party_id);
            }
            QuestAction::AddPartyMember { party_id, adv_id } => {
                let _ = state.add_party_member(party_id, adv_id);
            }
            QuestAction::RemovePartyMember { party_id, adv_id } => {
                let _ = state.remove_party_member(party_id, adv_id);
            }
            QuestAction::ClearReports => {
                state.clear_quest_reports();
            }
        }
    }
}

