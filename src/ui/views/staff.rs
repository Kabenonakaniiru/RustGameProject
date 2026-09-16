use crate::core::GameState;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, state: &mut GameState) {
    ui.heading("👥 職員・施設・維持費管理（支出項目）");
    ui.separator();
    ui.label("ギルドの運営には日々の人件費、固定資産税、光熱費がかかります。");
    ui.add_space(8.0);

    ui.group(|ui| {
        ui.heading("🏢 固定資産・施設維持費（日額）");
        ui.horizontal(|ui| {
            ui.label("ギルド本部施設 固定資産税（日換算）:");
            ui.strong(format!("{} G / 日", state.daily_tax));
        });
        ui.horizontal(|ui| {
            ui.label("ギルド本部 光熱費・水道代:");
            ui.strong(format!("{} G / 日", state.daily_utility));
        });
    });

    ui.add_space(8.0);

    ui.group(|ui| {
        ui.heading("👤 雇用職員一覧（人件費）");
        egui::Grid::new("staff_grid")
            .striped(true)
            .min_col_width(120.0)
            .show(ui, |ui| {
                ui.strong("職員名");
                ui.strong("役職");
                ui.strong("日給");
                ui.end_row();

                for staff in &state.staff_list {
                    ui.label(&staff.name);
                    ui.label(&staff.role);
                    ui.label(format!("{} G / 日", staff.salary));
                    ui.end_row();
                }
            });

        ui.add_space(4.0);
        ui.label(format!(
            "職員人件費 合計: {} G / 日",
            state.daily_staff_salary()
        ));
    });

    ui.add_space(8.0);
    ui.colored_label(
        egui::Color32::from_rgb(255, 120, 120),
        format!(
            "1日あたりの固定経費総計: {} G（毎日の日付変更時に自動引き落とし）",
            state.daily_total_expenses()
        ),
    );
}
