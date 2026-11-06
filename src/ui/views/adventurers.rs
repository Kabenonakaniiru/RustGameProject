use crate::core::adventurer::AdventurerStatus;
use crate::core::GameState;
use eframe::egui;

/// 借用安全のためのアクション列挙
enum AdventurerAction {
    Retire(u64),
    HireAsInstructor(u64),
    DismissInstructor(u64),
    AcceptRecruit(usize),
    DeclineRecruit(usize),
}

/// 志願者スナップショット型エイリアス（index, id, name, rank, age, role, power, skills）
type RecruitSnapshot = (usize, u64, String, String, u32, String, u32, String);

pub fn render(ui: &mut egui::Ui, state: &mut GameState) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        render_inner(ui, state);
    });
}

fn render_inner(ui: &mut egui::Ui, state: &mut GameState) {
    ui.heading("⚔ 冒険者名簿（スキル制・酒場・宿舎）");
    ui.separator();
    ui.label("ギルドに登録されている冒険者の一覧です。職業制ではなくスキル制で成長し、年齢を重ねます。");
    ui.add_space(8.0);

    // ──────────────────────────────
    // アクティブ冒険者一覧
    // ──────────────────────────────
    let mut actions: Vec<AdventurerAction> = Vec::new();

    // 現役冒険者（Fallen以外）
    let active_ids: Vec<u64> = state
        .adventurers
        .iter()
        .filter(|a| !matches!(a.status, AdventurerStatus::Fallen { .. }))
        .map(|a| a.id)
        .collect();

    egui::Grid::new("adventurers_grid")
        .striped(true)
        .min_col_width(70.0)
        .show(ui, |ui| {
            ui.strong("冒険者名");
            ui.strong("ランク");
            ui.strong("年齢");
            ui.strong("所属部隊");
            ui.strong("得意領域");
            ui.strong("戦闘力");
            ui.strong("習得スキル");
            ui.strong("状態");
            ui.strong("操作");
            ui.end_row();

            for &adv_id in &active_ids {
                let Some(adv) = state.adventurers.iter().find(|a| a.id == adv_id) else {
                    continue;
                };
                let adv_id = adv.id;
                let adv_name = adv.name.clone();
                let adv_rank = adv.rank.clone();
                let adv_age = adv.age;
                let adv_role = adv.primary_role();
                let adv_power = adv.total_combat_power();
                let skills_str = adv
                    .skills
                    .iter()
                    .map(|s| format!("{}:Lv{}", s.skill_type.name(), s.level))
                    .collect::<Vec<_>>()
                    .join(", ");
                let status_text = adv.status.display_name();
                let status_clone = adv.status.clone();

                ui.label(&adv_name);
                ui.label(&adv_rank);
                ui.label(format!("{adv_age}歳"));

                // 所属部隊の表示
                let party_name = state
                    .get_adventurer_party(adv_id)
                    .map_or_else(|| "（無所属）".to_string(), |p| format!("『{}』", p.name));
                if party_name == "（無所属）" {
                    ui.colored_label(egui::Color32::from_rgb(160, 160, 160), &party_name);
                } else {
                    ui.colored_label(egui::Color32::from_rgb(140, 200, 255), &party_name);
                }

                ui.label(&adv_role);
                ui.label(format!("{adv_power}"));
                ui.label(&skills_str);

                // 状態表示と色分け
                match &status_clone {
                    AdventurerStatus::Standby => {
                        ui.colored_label(egui::Color32::from_rgb(100, 220, 100), &status_text);
                    }
                    AdventurerStatus::OnQuest { .. } => {
                        ui.colored_label(egui::Color32::from_rgb(255, 180, 50), &status_text);
                    }
                    AdventurerStatus::Injured { .. } => {
                        ui.colored_label(egui::Color32::from_rgb(230, 80, 80), &status_text);
                    }
                    AdventurerStatus::Retired => {
                        ui.colored_label(egui::Color32::from_rgb(180, 180, 180), &status_text);
                    }
                    AdventurerStatus::Fallen { .. } => {
                        ui.colored_label(egui::Color32::from_rgb(150, 50, 50), &status_text);
                    }
                    AdventurerStatus::Training => {
                        ui.colored_label(egui::Color32::from_rgb(100, 180, 255), &status_text);
                    }
                }

                // 操作ボタン
                ui.horizontal(|ui| {
                    match &status_clone {
                        AdventurerStatus::Standby => {
                            if ui
                                .add(egui::Button::new("🚪 引退").small())
                                .on_hover_text("この冒険者を現役引退させます（教官採用が可能になります）")
                                .clicked()
                            {
                                actions.push(AdventurerAction::Retire(adv_id));
                            }
                        }
                        AdventurerStatus::Retired => {
                            // 既に教官かどうか確認
                            let already_instructor = state
                                .training_dojo
                                .instructors
                                .iter()
                                .any(|i| i.adventurer_id == Some(adv_id));
                            if already_instructor {
                                if ui
                                    .add(egui::Button::new("🏫 解雇").small())
                                    .on_hover_text("訓練所の教官職を解雇します")
                                    .clicked()
                                {
                                    actions.push(AdventurerAction::DismissInstructor(adv_id));
                                }
                            } else if ui
                                .add(
                                    egui::Button::new("🎓 教官採用")
                                        .small()
                                        .fill(egui::Color32::from_rgb(50, 100, 180)),
                                )
                                .on_hover_text(format!(
                                    "{}を訓練所の教官として雇用します（日給 {}G）",
                                    adv_name,
                                    suggested_salary(adv_age)
                                ))
                                .clicked()
                            {
                                actions.push(AdventurerAction::HireAsInstructor(adv_id));
                            }
                        }
                        _ => {
                            ui.label("—");
                        }
                    }
                });

                ui.end_row();
            }
        });

    // アクション実行（借用解放後）
    for action in actions {
        match action {
            AdventurerAction::Retire(id) => {
                if let Err(e) = state.retire_adventurer(id) {
                    state.add_log(format!("【エラー】引退処理失敗: {e}"));
                }
            }
            AdventurerAction::HireAsInstructor(id) => {
                let age = state.adventurers.iter().find(|a| a.id == id).map_or(40, |a| a.age);
                let salary = suggested_salary(age);
                if let Err(e) = state.hire_as_instructor(id, salary) {
                    state.add_log(format!("【エラー】教官採用失敗: {e}"));
                }
            }
            AdventurerAction::DismissInstructor(id) => {
                if let Err(e) = state.dismiss_instructor(id) {
                    state.add_log(format!("【エラー】教官解雇失敗: {e}"));
                }
            }
            AdventurerAction::AcceptRecruit(idx) => {
                if let Err(e) = state.accept_recruit(idx) {
                    state.add_log(format!("【エラー】採用失敗: {e}"));
                }
            }
            AdventurerAction::DeclineRecruit(idx) => {
                if let Err(e) = state.decline_recruit(idx) {
                    state.add_log(format!("【エラー】見送り失敗: {e}"));
                }
            }
        }
    }

    // ──────────────────────────────
    // 訓練所 教官一覧
    // ──────────────────────────────
    if !state.training_dojo.instructors.is_empty() {
        ui.add_space(16.0);
        ui.separator();
        ui.heading("🏫 訓練所 教官一覧");
        ui.label("引退冒険者が後輩に経験を伝えます。教官の得意スキルが訓練生の成長率を高めます。");
        ui.add_space(4.0);

        egui::Grid::new("instructor_grid")
            .striped(true)
            .min_col_width(100.0)
            .show(ui, |ui| {
                ui.strong("教官名");
                ui.strong("得意スキル");
                ui.strong("指導Lv");
                ui.strong("指導術Lv");
                ui.strong("日給");
                ui.end_row();

                for inst in &state.training_dojo.instructors {
                    ui.label(&inst.name);
                    ui.label(inst.specialty_skill.name());
                    ui.label(format!("Lv{}", inst.skill_bonus_level));
                    ui.label(format!("Lv{}", inst.mentorship_level));
                    ui.label(format!("{} G / 日", inst.daily_salary));
                    ui.end_row();
                }
            });

        let trainee_count = state.training_dojo.trainee_ids.len();
        ui.add_space(4.0);
        ui.label(format!("訓練生: {trainee_count} 名 / {} 名", state.training_dojo.max_trainees));
    }

    // ──────────────────────────────
    // 志願者待機キュー
    // ──────────────────────────────
    ui.add_space(16.0);
    ui.separator();
    ui.heading("📋 志願者一覧（採用候補）");

    if state.pending_recruits.is_empty() {
        ui.colored_label(
            egui::Color32::from_rgb(160, 160, 160),
            "現在、ギルドへの入団志願者はいません。名声を高めると優秀な志願者が集まりやすくなります。",
        );
    } else {
        ui.label(format!(
            "{}名の冒険者希望者がいます。採用か見送りを選択してください。",
            state.pending_recruits.len()
        ));
        ui.add_space(4.0);

        // 志願者情報をコピーしてからUIを描画（借用衝突を避ける）
        let recruits_snapshot: Vec<RecruitSnapshot> =
            state
                .pending_recruits
                .iter()
                .enumerate()
                .map(|(i, r)| {
                    let skills_str = r
                        .skills
                        .iter()
                        .map(|s| format!("{}:Lv{}", s.skill_type.name(), s.level))
                        .collect::<Vec<_>>()
                        .join(", ");
                    (
                        i,
                        r.id,
                        r.name.clone(),
                        r.rank.clone(),
                        r.age,
                        r.primary_role(),
                        r.total_combat_power(),
                        skills_str,
                    )
                })
                .collect();

        let mut recruit_actions: Vec<AdventurerAction> = Vec::new();

        egui::Grid::new("recruit_grid")
            .striped(true)
            .min_col_width(70.0)
            .show(ui, |ui| {
                ui.strong("名前");
                ui.strong("ランク");
                ui.strong("年齢");
                ui.strong("得意領域");
                ui.strong("戦闘力");
                ui.strong("習得スキル");
                ui.strong("操作");
                ui.end_row();

                for (i, _id, name, rank, age, role, power, skills_str) in &recruits_snapshot {
                    ui.label(name);
                    ui.colored_label(egui::Color32::from_rgb(255, 220, 100), rank);
                    ui.label(format!("{age}歳"));
                    ui.label(role);
                    ui.label(format!("{power}"));
                    ui.label(skills_str);

                    ui.horizontal(|ui| {
                        if ui
                            .add(
                                egui::Button::new("✅ 採用")
                                    .small()
                                    .fill(egui::Color32::from_rgb(50, 160, 80)),
                            )
                            .clicked()
                        {
                            recruit_actions.push(AdventurerAction::AcceptRecruit(*i));
                        }
                        if ui
                            .add(egui::Button::new("❌ 見送り").small())
                            .clicked()
                        {
                            recruit_actions.push(AdventurerAction::DeclineRecruit(*i));
                        }
                    });
                    ui.end_row();
                }
            });

        // 志願者アクション実行（インデックスは末尾から処理して崩れを防ぐ）
        let mut recruit_actions_sorted = recruit_actions;
        recruit_actions_sorted.sort_by(|a, b| {
            let idx_a = match a {
                AdventurerAction::AcceptRecruit(i) | AdventurerAction::DeclineRecruit(i) => *i,
                _ => 0,
            };
            let idx_b = match b {
                AdventurerAction::AcceptRecruit(i) | AdventurerAction::DeclineRecruit(i) => *i,
                _ => 0,
            };
            idx_b.cmp(&idx_a) // 末尾から処理
        });
        for action in recruit_actions_sorted {
            match action {
                AdventurerAction::AcceptRecruit(idx) => {
                    if let Err(e) = state.accept_recruit(idx) {
                        state.add_log(format!("【エラー】採用失敗: {e}"));
                    }
                }
                AdventurerAction::DeclineRecruit(idx) => {
                    if let Err(e) = state.decline_recruit(idx) {
                        state.add_log(format!("【エラー】見送り失敗: {e}"));
                    }
                }
                _ => {}
            }
        }
    }

    // ──────────────────────────────
    // 殿堂入り・追悼録
    // ──────────────────────────────
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

/// 年齢に応じた推奨日給を計算
fn suggested_salary(age: u32) -> i64 {
    // 若い引退者は低め、ベテランは高め（最低25G、最高80G）
    let base = 25i64 + ((age as i64).saturating_sub(25)) * 2;
    base.clamp(25, 80)
}
