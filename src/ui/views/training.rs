use crate::core::adventurer::AdventurerStatus;
use crate::core::training::suggested_instructor_salary;
use crate::core::GameState;
use eframe::egui;

#[derive(Debug)]
enum TrainingAction {
    Enroll(u64),
    Graduate(u64),
    DismissInstructor(u64),
    HireInstructor(u64, i64),
}

pub fn render(ui: &mut egui::Ui, state: &mut GameState) {
    let mut actions = Vec::new();

    ui.heading("🥋 ギルド付属訓練所（道場・指導修練）");
    ui.separator();
    ui.label(
        "引退したベテラン冒険者を教官として迎え、現役の若手・後輩冒険者を鍛え上げる施設です。\n\
         教官の得意スキルに応じて、入門した訓練生は毎日自動的にスキルの指導・経験値向上を受けます。",
    );
    ui.add_space(8.0);

    // ──────────────────────────────
    // 1. 訓練所ステータスサマリーカード
    // ──────────────────────────────
    let instructor_count = state.training_dojo.instructors.len();
    let trainee_count = state.training_dojo.trainee_ids.len();
    let max_trainees = state.training_dojo.max_trainees;
    let daily_salary = state.training_dojo.daily_salary_total();

    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.label("🏛 道場ステータス:");
            ui.strong(format!("教官数: {instructor_count} 名"));
            ui.separator();
            ui.strong(format!("受入枠: {trainee_count} / {max_trainees} 名"));
            ui.separator();
            ui.colored_label(
                egui::Color32::from_rgb(255, 140, 140),
                format!("教官日給総計: -{daily_salary} G / 日"),
            );
        });
    });

    ui.add_space(8.0);

    // ──────────────────────────────
    // 2. 指導教官セクション
    // ──────────────────────────────
    ui.group(|ui| {
        ui.heading("👨‍🏫 指導教官一覧");
        ui.label("引退冒険者が後輩に経験を伝授します。得意スキルの指導ボーナスと指導術Lvが訓練速度を左右します。");
        ui.add_space(4.0);

        if state.training_dojo.instructors.is_empty() {
            ui.colored_label(
                egui::Color32::from_rgb(200, 200, 100),
                "現在、指導教官はいません。引退した冒険者を教官として雇用してください。",
            );
        } else {
            egui::Grid::new("training_instructors_grid")
                .striped(true)
                .min_col_width(90.0)
                .show(ui, |ui| {
                    ui.strong("教官名");
                    ui.strong("得意指導スキル");
                    ui.strong("指導Lv");
                    ui.strong("指導術Lv");
                    ui.strong("日給");
                    ui.strong("操作");
                    ui.end_row();

                    for inst in &state.training_dojo.instructors {
                        ui.label(&inst.name);
                        ui.label(inst.specialty_skill.name());
                        ui.label(format!("Lv{}", inst.skill_bonus_level));
                        ui.label(format!("Lv{}", inst.mentorship_level));
                        ui.label(format!("{} G / 日", inst.daily_salary));

                        if let Some(adv_id) = inst.adventurer_id {
                            if ui
                                .button("解雇")
                                .on_hover_text("この教官を解雇します（日給の支払いが停止します）")
                                .clicked()
                            {
                                actions.push(TrainingAction::DismissInstructor(adv_id));
                            }
                        } else {
                            ui.label("専任");
                        }
                        ui.end_row();
                    }
                });
        }

        // 雇用可能な引退冒険者
        let retired_candidates: Vec<(u64, String, u32, i64, &'static str)> = state
            .adventurers
            .iter()
            .filter(|a| matches!(a.status, AdventurerStatus::Retired))
            .filter(|a| !state.training_dojo.is_instructor(a.id))
            .map(|a| {
                let best_skill = a
                    .skills
                    .iter()
                    .max_by_key(|s| s.level)
                    .map_or("未習得", |s| s.skill_type.name());
                let salary = suggested_instructor_salary(a.age);
                (a.id, a.name.clone(), a.age, salary, best_skill)
            })
            .collect();

        if !retired_candidates.is_empty() {
            ui.add_space(6.0);
            ui.separator();
            ui.strong("📋 教官就任が可能な引退冒険者:");
            for (id, name, age, salary, skill_name) in retired_candidates {
                ui.horizontal(|ui| {
                    ui.label(format!("・{name}（{age}歳 / 最得意: {skill_name}）"));
                    ui.label(format!("想定日給: {salary} G"));
                    if ui.button("教官として雇用").clicked() {
                        actions.push(TrainingAction::HireInstructor(id, salary));
                    }
                });
            }
        }
    });

    ui.add_space(8.0);

    // ──────────────────────────────
    // 3. 訓練生一覧（現在入門中）
    // ──────────────────────────────
    ui.group(|ui| {
        ui.heading("🎯 現在の訓練生（特訓中）");
        ui.label("道場で修練中の冒険者です。毎日の日付変更時に経験値が自動獲得されます（派遣・部隊参加不可）。");
        ui.add_space(4.0);

        if state.training_dojo.trainee_ids.is_empty() {
            ui.label("現在、道場に入門している訓練生はいません。");
        } else {
            let trainee_ids = state.training_dojo.trainee_ids.clone();
            egui::Grid::new("training_trainees_grid")
                .striped(true)
                .min_col_width(80.0)
                .show(ui, |ui| {
                    ui.strong("冒険者名");
                    ui.strong("ランク");
                    ui.strong("年齢");
                    ui.strong("得意領域");
                    ui.strong("戦闘力");
                    ui.strong("習得スキル");
                    ui.strong("操作");
                    ui.end_row();

                    for id in trainee_ids {
                        let Some(adv) = state.adventurers.iter().find(|a| a.id == id) else {
                            continue;
                        };
                        ui.label(&adv.name);
                        ui.label(&adv.rank);
                        ui.label(format!("{}歳", adv.age));
                        ui.label(adv.primary_role());
                        ui.label(format!("⚔ {}", adv.total_combat_power()));

                        let skills_str = adv
                            .skills
                            .iter()
                            .map(|s| format!("{}:Lv{}", s.skill_type.name(), s.level))
                            .collect::<Vec<_>>()
                            .join(", ");
                        ui.label(skills_str);

                        if ui
                            .button("🎓 卒業・退所")
                            .on_hover_text("修練を終えて待機状態に戻します（部隊配属・クエスト派遣が可能になります）")
                            .clicked()
                        {
                            actions.push(TrainingAction::Graduate(id));
                        }
                        ui.end_row();
                    }
                });
        }
    });

    ui.add_space(8.0);

    // ──────────────────────────────
    // 4. 入門候補生（待機中の冒険者）
    // ──────────────────────────────
    ui.group(|ui| {
        ui.heading("📥 訓練生 入門登録（待機中の冒険者）");
        ui.label("待機中の冒険者を道場に入門させます（入門中はクエスト派遣ができなくなります）。");
        ui.add_space(4.0);

        let standby_advs: Vec<(u64, String, String, u32, String, u32)> = state
            .adventurers
            .iter()
            .filter(|a| matches!(a.status, AdventurerStatus::Standby))
            .map(|a| (a.id, a.name.clone(), a.rank.clone(), a.age, a.primary_role(), a.total_combat_power()))
            .collect();

        if standby_advs.is_empty() {
            ui.label("現在、待機中の冒険者はいません（全員が部隊派遣中、療養中、または既に道場に入門しています）。");
        } else {
            let is_full = state.training_dojo.trainee_ids.len() >= state.training_dojo.max_trainees;

            egui::Grid::new("standby_candidates_grid")
                .striped(true)
                .min_col_width(80.0)
                .show(ui, |ui| {
                    ui.strong("冒険者名");
                    ui.strong("ランク");
                    ui.strong("年齢");
                    ui.strong("得意領域");
                    ui.strong("戦闘力");
                    ui.strong("操作");
                    ui.end_row();

                    for (id, name, rank, age, role, power) in standby_advs {
                        ui.label(name);
                        ui.label(rank);
                        ui.label(format!("{age}歳"));
                        ui.label(role);
                        ui.label(format!("⚔ {power}"));

                        if is_full {
                            ui.add_enabled(false, egui::Button::new("枠上限"))
                                .on_disabled_hover_text("訓練所の受け入れ枠がいっぱいです");
                        } else if ui
                            .button("🥋 入門させる")
                            .on_hover_text("この冒険者を訓練所に入門させ、毎日の指導修練を受けさせます")
                            .clicked()
                        {
                            actions.push(TrainingAction::Enroll(id));
                        }
                        ui.end_row();
                    }
                });
        }
    });

    ui.add_space(8.0);

    // ──────────────────────────────
    // 5. 道場修練・成長ログ
    // ──────────────────────────────
    ui.group(|ui| {
        ui.heading("📜 最近の道場修練・成長ログ");
        ui.label("教官の指導によるスキルの開眼やレベルアップの履歴です。");
        ui.add_space(4.0);

        if state.training_dojo.recent_training_logs.is_empty() {
            ui.label("まだ道場での修練ログはありません。");
        } else {
            egui::ScrollArea::vertical()
                .max_height(140.0)
                .show(ui, |ui| {
                    for log in state.training_dojo.recent_training_logs.iter().rev() {
                        ui.label(log);
                    }
                });
        }
    });

    // ──────────────────────────────
    // アクションの適用（Action パターン）
    // ──────────────────────────────
    for action in actions {
        match action {
            TrainingAction::Enroll(id) => {
                if let Err(e) = state.enroll_trainee(id) {
                    state.add_log(format!("【エラー】道場入門失敗: {e}"));
                }
            }
            TrainingAction::Graduate(id) => {
                if let Err(e) = state.remove_trainee(id) {
                    state.add_log(format!("【エラー】道場退所失敗: {e}"));
                }
            }
            TrainingAction::DismissInstructor(id) => {
                if let Err(e) = state.dismiss_instructor(id) {
                    state.add_log(format!("【エラー】教官解雇失敗: {e}"));
                }
            }
            TrainingAction::HireInstructor(id, salary) => {
                if let Err(e) = state.hire_as_instructor(id, salary) {
                    state.add_log(format!("【エラー】教官採用失敗: {e}"));
                }
            }
        }
    }
}
