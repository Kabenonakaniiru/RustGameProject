use crate::core::GameState;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, state: &mut GameState) {
    ui.heading("📦 ギルド倉庫・戦利品売買（主収益源）");
    ui.separator();
    ui.label("冒険者から買い取った魔物部位や素材を管理し、商人に売却して利益を得ます。");
    ui.add_space(8.0);

    let mut sold_index = None;
    let mut sell_all = false;

    if ui.button("💰 在庫をすべて市場に一括売却する").clicked() {
        sell_all = true;
    }
    ui.add_space(8.0);

    egui::Grid::new("inventory_grid")
        .striped(true)
        .min_col_width(90.0)
        .show(ui, |ui| {
            ui.strong("品名");
            ui.strong("分類");
            ui.strong("在庫数");
            ui.strong("買取単価");
            ui.strong("市場売却単価");
            ui.strong("見込利益/個");
            ui.strong("アクション");
            ui.end_row();

            for (idx, item) in state.inventory.iter().enumerate() {
                let profit_per_unit = item.unit_profit();
                ui.label(&item.name);
                ui.label(&item.category);
                ui.label(format!("{} 個", item.count));
                ui.label(format!("{} G", item.buy_price));
                ui.label(format!("{} G", item.market_price));
                ui.colored_label(
                    egui::Color32::from_rgb(100, 220, 100),
                    format!("+{} G", profit_per_unit),
                );

                if ui.button("1個売却").clicked() {
                    sold_index = Some(idx);
                }
                ui.end_row();
            }
        });

    if let Some(idx) = sold_index {
        state.sell_item(idx);
    }

    if sell_all {
        state.sell_all_items();
    }
}
