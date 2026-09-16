use crate::core::GameState;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, state: &mut GameState) {
    ui.heading("📊 ギルド収支台帳・経営レポート");
    ui.separator();
    ui.label("ギルドの総収入と総支出の内訳です。");
    ui.add_space(12.0);

    let total_income = state.total_sales_revenue + state.total_commission;
    let net_profit = total_income - state.total_expenses;

    egui::Grid::new("finance_grid")
        .striped(true)
        .min_col_width(160.0)
        .show(ui, |ui| {
            ui.strong("項目");
            ui.strong("金額");
            ui.end_row();

            ui.label("💰 素材・部位売却売上（累計）");
            ui.colored_label(
                egui::Color32::from_rgb(100, 220, 100),
                format!("+{} G", state.total_sales_revenue),
            );
            ui.end_row();

            ui.label("📝 クエスト仲介手数料（累計）");
            ui.colored_label(
                egui::Color32::from_rgb(100, 220, 100),
                format!("+{} G", state.total_commission),
            );
            ui.end_row();

            ui.strong("【総収入合計】");
            ui.strong(format!("+{} G", total_income));
            ui.end_row();

            ui.separator();
            ui.separator();
            ui.end_row();

            ui.label("🔻 ギルド固定資産税・維持費（累計）");
            ui.colored_label(
                egui::Color32::from_rgb(255, 120, 120),
                format!("-{} G", state.total_expenses),
            );
            ui.end_row();

            ui.separator();
            ui.separator();
            ui.end_row();

            ui.strong("【純利益（損益）】");
            if net_profit >= 0 {
                ui.colored_label(
                    egui::Color32::from_rgb(100, 220, 100),
                    format!("+{} G (黒字)", net_profit),
                );
            } else {
                ui.colored_label(
                    egui::Color32::from_rgb(255, 80, 80),
                    format!("{} G (赤字)", net_profit),
                );
            }
            ui.end_row();
        });
}
