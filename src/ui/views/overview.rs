use crate::core::guild_rank::PromotionRequirement;
use crate::core::GameState;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, state: &mut GameState) {
    ui.heading("🏛 ギルド基本情報・執務室");
    ui.separator();

    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.label("ギルド名称: 王都冒険者ギルド第3支部");
            ui.label(format!("ギルドランク: {}", state.guild_rank.display_name()));
            ui.label(format!("ギルド名声: {} pt", state.reputation));
            ui.label(format!("累計クエスト達成: {} 件", state.total_quests_completed));
            ui.label(format!("所属冒険者数: {} 名 (編成部隊: {}隊)", state.adventurers.len(), state.parties.len()));
            ui.label(format!("雇用職員・教官数: {} 名", state.staff_list.len() + state.training_dojo.instructors.len()));
            ui.label(format!("ギルド殿堂・追悼録: {} 名", state.hall_of_fame.len()));
        });

        ui.add_space(30.0);

        ui.group(|ui| {
            ui.heading("🤝 発注勢力との信頼度");
            for (faction, trust) in &state.faction_trust {
                let color = if *trust >= 20 {
                    egui::Color32::from_rgb(100, 220, 100)
                } else if *trust >= 0 {
                    egui::Color32::from_rgb(200, 200, 100)
                } else {
                    egui::Color32::from_rgb(255, 100, 100)
                };
                ui.colored_label(color, format!("{}: {} pt", faction.name(), trust));
            }
        });
    });

    ui.add_space(12.0);

    ui.horizontal(|ui| {
        // 月次監査情報
        ui.group(|ui| {
            ui.heading("⚖️ 冒険者連盟 月次監査");
            let days_left = state.audit.days_until_next_audit(state.day);
            ui.label(format!("次回監査まで: 残り {} 日 ({} 日周期)", days_left, crate::core::audit::AUDIT_PERIOD_DAYS));
            ui.label(format!(
                "今期の依頼達成: {} / {} 件",
                state.audit.quests_completed_this_period,
                state.guild_rank.monthly_quest_requirement()
            ));
            ui.label(format!("予定納付上納金: {} G", state.guild_rank.monthly_tribute()));

            if state.audit.consecutive_failures > 0 {
                ui.colored_label(
                    egui::Color32::from_rgb(255, 100, 100),
                    format!("⚠️ 連続未達回数: {} / 3（3回で認可取消）", state.audit.consecutive_failures),
                );
            } else {
                ui.colored_label(egui::Color32::from_rgb(100, 220, 100), "連盟認可ステータス: 良好 (問題なし)");
            }
        });

        ui.add_space(15.0);

        // 次期昇格要件
        ui.group(|ui| {
            ui.heading("🎖️ ギルドランク昇格状況");
            if let Some(next) = state.guild_rank.next_rank() {
                ui.label(format!("目標ランク: {}", next.display_name()));
                if let Some(req) = PromotionRequirement::for_rank(next) {
                    let rep_color = if state.reputation >= req.required_reputation {
                        egui::Color32::from_rgb(100, 220, 100)
                    } else {
                        egui::Color32::from_rgb(220, 180, 100)
                    };
                    ui.colored_label(rep_color, format!("・総合名声: {} / {} pt", state.reputation, req.required_reputation));

                    let quest_color = if state.total_quests_completed >= req.required_total_quests {
                        egui::Color32::from_rgb(100, 220, 100)
                    } else {
                        egui::Color32::from_rgb(220, 180, 100)
                    };
                    ui.colored_label(
                        quest_color,
                        format!("・累計クエスト達成: {} / {} 件", state.total_quests_completed, req.required_total_quests),
                    );

                    let days_color = if state.day >= req.required_min_days {
                        egui::Color32::from_rgb(100, 220, 100)
                    } else {
                        egui::Color32::from_rgb(220, 180, 100)
                    };
                    ui.colored_label(
                        days_color,
                        format!("・運営実績日数: {} / {} 日", state.day, req.required_min_days),
                    );
                }
            } else {
                ui.colored_label(egui::Color32::from_rgb(255, 215, 0), "🏆 最高峰の Sランク（伝説級）ギルドです！");
            }
        });
    });

    ui.add_space(12.0);
    ui.group(|ui| {
        ui.heading("💰 財務ハイライト");
        ui.label(format!("現在の手元資金: {} G", state.gold));
        ui.label(format!("累計素材売却益: +{} G", state.total_sales_revenue));
        ui.label(format!("累計仲介手数料: +{} G", state.total_commission));
        ui.label(format!("累計固定支出費: -{} G", state.total_expenses));

        let net_balance = state.total_sales_revenue + state.total_commission - state.total_expenses;
        let color = if net_balance >= 0 {
            egui::Color32::from_rgb(100, 220, 100)
        } else {
            egui::Color32::from_rgb(255, 100, 100)
        };
        ui.colored_label(color, format!("通算純収益: {} G", net_balance));
    });
}
