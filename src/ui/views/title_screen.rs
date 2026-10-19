use eframe::egui;

/// タイトル画面から返されるアクション
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TitleAction {
    None,
    NewGame,
    LoadGame,
    OpenSettings,
}

/// タイトル画面を描画し、選択されたアクションを返す
pub fn render(ui: &mut egui::Ui, has_save_data: bool) -> TitleAction {
    let mut action = TitleAction::None;

    ui.vertical_centered(|ui| {
        ui.add_space(80.0);

        // ゲームタイトル
        ui.label(
            egui::RichText::new("🏰 冒険者ギルド経営シミュレーション")
                .size(34.0)
                .strong()
                .color(egui::Color32::from_rgb(255, 215, 0)),
        );

        ui.add_space(10.0);
        ui.label(
            egui::RichText::new("— 栄光のギルドマスターへの道 —")
                .size(16.0)
                .color(egui::Color32::from_rgb(180, 180, 200)),
        );

        ui.add_space(60.0);

        // メニューボタン群
        let button_width = 280.0;
        let button_height = 46.0;

        // 新規開業ボタン
        let new_game_btn = egui::Button::new(
            egui::RichText::new("🆕 新規開業").size(18.0).strong(),
        )
        .min_size(egui::vec2(button_width, button_height));
        if ui.add(new_game_btn).clicked() {
            action = TitleAction::NewGame;
        }

        ui.add_space(12.0);

        // 続きからボタン（セーブデータがある場合のみ有効）
        let load_btn = egui::Button::new(
            egui::RichText::new("📂 続きから（ロード）").size(18.0),
        )
        .min_size(egui::vec2(button_width, button_height));
        let load_response = ui.add_enabled(has_save_data, load_btn);
        if load_response.clicked() {
            action = TitleAction::LoadGame;
        }
        if !has_save_data {
            load_response.on_disabled_hover_text("セーブデータ (save_data.json) が見つかりません");
        }

        ui.add_space(12.0);

        // 環境設定ボタン
        let settings_btn = egui::Button::new(
            egui::RichText::new("⚙️ 環境設定").size(18.0),
        )
        .min_size(egui::vec2(button_width, button_height));
        if ui.add(settings_btn).clicked() {
            action = TitleAction::OpenSettings;
        }

        ui.add_space(40.0);

        // フッターコピーライト等
        ui.label(
            egui::RichText::new("© Adventurer's Guild Management Sim")
                .size(11.0)
                .color(egui::Color32::from_rgb(110, 110, 130)),
        );
    });

    action
}
