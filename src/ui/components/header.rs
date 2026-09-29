use crate::core::GameState;
use eframe::egui;

pub enum HeaderAction {
    None,
    AdvanceDay,
    Save,
    Load,
    ZoomIn,
    ZoomOut,
    ResetZoom,
}

pub fn render(
    ui: &mut egui::Ui,
    state: &GameState,
    status_msg: Option<&str>,
    zoom_factor: f32,
) -> HeaderAction {
    let mut action = HeaderAction::None;
    let available_width = ui.available_width();

    egui::Frame::side_top_panel(ui.style().as_ref())
        .fill(egui::Color32::from_rgb(25, 27, 34))
        .inner_margin(egui::Margin::symmetric(14, 8))
        .show(ui, |ui| {
            // 幅が狭い（720px未満）ときは縦並び、広いときは左右分割
            if available_width >= 720.0 {
                ui.horizontal(|ui| {
                    ui.heading("🏰 冒険者ギルド経営日誌");

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        render_action_buttons(ui, &mut action, status_msg, zoom_factor);
                    });
                });
            } else {
                ui.vertical(|ui| {
                    ui.heading("🏰 冒険者ギルド経営日誌");
                    ui.add_space(4.0);
                    ui.horizontal_wrapped(|ui| {
                        render_action_buttons(ui, &mut action, status_msg, zoom_factor);
                    });
                });
            }

            ui.add_space(4.0);
            ui.separator();
            ui.add_space(4.0);

            // ステータスサマリーバー（折り返し対応）
            ui.horizontal_wrapped(|ui| {
                ui.label(format!(
                    "📅 カレンダー: 第 {} 年目 {} 日目 (通算 {} 日)",
                    state.current_year(),
                    state.day_of_year(),
                    state.day
                ));
                ui.separator();

                let gold_color = if state.gold >= 0 {
                    egui::Color32::from_rgb(255, 215, 0)
                } else {
                    egui::Color32::from_rgb(255, 80, 80)
                };
                ui.colored_label(gold_color, format!("💰 ギルド資金: {} G", state.gold));
                ui.separator();

                ui.label(format!("🎖️ ギルドランク: {}", state.guild_rank.display_name()));
                ui.separator();

                ui.label(format!("⭐ 名声値: {} pt", state.reputation));
                ui.separator();

                ui.colored_label(
                    egui::Color32::from_rgb(255, 120, 120),
                    format!("📉 日次固定支出: -{} G/日", state.daily_total_expenses()),
                );
                ui.separator();

                let days_to_audit = state.audit.days_until_next_audit(state.day);
                let audit_color = if days_to_audit <= 5 {
                    egui::Color32::from_rgb(255, 100, 100)
                } else {
                    egui::Color32::from_rgb(200, 200, 200)
                };
                ui.colored_label(audit_color, format!("⚖️ 次回監査まで: 残り {} 日", days_to_audit));

                if state.game_over {
                    ui.separator();
                    ui.colored_label(egui::Color32::RED, "☠️ 【認可取消・GAME OVER】");
                } else if state.game_cleared {
                    ui.separator();
                    ui.colored_label(egui::Color32::GREEN, "🏆 【Sランク到達・GAME CLEARED】");
                }
            });
        });

    action
}

fn render_action_buttons(
    ui: &mut egui::Ui,
    action: &mut HeaderAction,
    status_msg: Option<&str>,
    zoom_factor: f32,
) {
    if ui
        .button(egui::RichText::new("⏩ 1日進める").strong())
        .clicked()
    {
        *action = HeaderAction::AdvanceDay;
    }

    if ui.button("💾 セーブ").clicked() {
        *action = HeaderAction::Save;
    }

    if ui.button("📂 ロード").clicked() {
        *action = HeaderAction::Load;
    }

    ui.separator();

    // ズーム調整コントロール
    ui.label(format!("🔍 {:.0}%", zoom_factor * 100.0));
    if ui.small_button("➖").on_hover_text("表示を縮小 (Ctrl + -)").clicked() {
        *action = HeaderAction::ZoomOut;
    }
    if ui.small_button("➕").on_hover_text("表示を拡大 (Ctrl + +)").clicked() {
        *action = HeaderAction::ZoomIn;
    }
    if (zoom_factor - 1.0).abs() > 0.01 {
        if ui.small_button("↺").on_hover_text("100%にリセット (Ctrl + 0)").clicked() {
            *action = HeaderAction::ResetZoom;
        }
    }

    if let Some(msg) = status_msg {
        ui.separator();
        ui.colored_label(egui::Color32::from_rgb(120, 220, 120), msg);
    }
}
