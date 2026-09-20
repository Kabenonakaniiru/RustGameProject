use crate::core::GameState;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, state: &mut GameState) {
    ui.heading("🏛 ギルド基本情報・執務室");
    ui.separator();

    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.label("ギルド名称: 王都冒険者ギルド第3支部");
            ui.label(format!("ギルドランク: {} ランク", state.rank));
            ui.label(format!("ギルド名声: {} pt", state.reputation));
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
