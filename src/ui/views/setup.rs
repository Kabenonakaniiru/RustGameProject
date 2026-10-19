use eframe::egui;

/// 初期設定フォームのデータ
#[derive(Debug, Clone)]
pub struct SetupForm {
    pub guild_name: String,
    pub master_name: String,
    pub location: String,
    pub seed_input: String,
    pub error_message: Option<String>,
}

impl Default for SetupForm {
    fn default() -> Self {
        Self {
            guild_name: "冒険者ギルド".to_string(),
            master_name: String::new(),
            location: "王都".to_string(),
            seed_input: String::new(),
            error_message: None,
        }
    }
}

impl SetupForm {
    /// 入力を検証し、エラーがあればエラーメッセージを返す
    #[must_use]
    pub fn validate(&self) -> Option<String> {
        let name = self.guild_name.trim();
        if name.is_empty() {
            return Some("ギルド名称を入力してください。".to_string());
        }
        if name.chars().count() > 30 {
            return Some("ギルド名称は30文字以内で入力してください。".to_string());
        }

        let master = self.master_name.trim();
        if master.is_empty() {
            return Some("ギルドマスター名を入力してください。".to_string());
        }
        if master.chars().count() > 20 {
            return Some("ギルドマスター名は20文字以内で入力してください。".to_string());
        }

        let loc = self.location.trim();
        if loc.is_empty() {
            return Some("開業地域を入力してください。".to_string());
        }
        if loc.chars().count() > 20 {
            return Some("開業地域は20文字以内で入力してください。".to_string());
        }

        // シード値の検証（空欄はOK=ランダム自動決定）
        let seed_str = self.seed_input.trim();
        if !seed_str.is_empty() && seed_str.parse::<u64>().is_err() {
            return Some("乱数シードは正の整数で入力してください。".to_string());
        }

        None
    }

    /// シード値を取得（空欄の場合はシステム時刻ベースの乱数値）
    #[must_use]
    pub fn seed(&self) -> u64 {
        self.seed_input
            .trim()
            .parse::<u64>()
            .unwrap_or_else(|_| {
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map_or(42, |d| {
                        let ms = d.as_millis();
                        u64::try_from(ms & 0xFFFF_FFFF_FFFF_FFFF).unwrap_or(42)
                    })
            })
    }
}

/// 初期設定画面から返されるアクション
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SetupAction {
    None,
    Start,
    Back,
}

/// 初期設定画面を描画し、選択されたアクションを返す
pub fn render(ui: &mut egui::Ui, form: &mut SetupForm) -> SetupAction {
    let mut action = SetupAction::None;

    ui.vertical_centered(|ui| {
        ui.add_space(40.0);

        ui.label(
            egui::RichText::new("📝 ギルド新規開業・初期設定")
                .size(28.0)
                .strong()
                .color(egui::Color32::from_rgb(255, 215, 0)),
        );

        ui.add_space(8.0);
        ui.label(
            egui::RichText::new("新たに設立する冒険者ギルドの基本情報を決定してください")
                .size(14.0)
                .color(egui::Color32::from_rgb(180, 180, 200)),
        );

        ui.add_space(24.0);
    });

    // フォーム本体（中央寄せ）
    let max_width = 520.0;
    let available_w = ui.available_width();
    let indent = ((available_w - max_width) / 2.0).max(16.0);

    ui.horizontal(|ui| {
        ui.add_space(indent);

        ui.vertical(|ui| {
            ui.set_max_width(max_width);

            // 1. ギルド名称
            ui.group(|ui| {
                ui.label(egui::RichText::new("🏛 ギルド名称").strong().size(15.0));
                ui.add_space(4.0);
                ui.add(
                    egui::TextEdit::singleline(&mut form.guild_name)
                        .desired_width(max_width - 30.0)
                        .hint_text("例: 蒼穹の冒険者ギルド")
                        .font(egui::TextStyle::Body),
                );
                ui.add_space(2.0);
                ui.small("冒険者連盟に正式登録するギルドの名前です（30文字以内）");
            });

            ui.add_space(10.0);

            // 2. ギルドマスター名
            ui.group(|ui| {
                ui.label(egui::RichText::new("👤 ギルドマスター名").strong().size(15.0));
                ui.add_space(4.0);
                ui.add(
                    egui::TextEdit::singleline(&mut form.master_name)
                        .desired_width(max_width - 30.0)
                        .hint_text("あなたの名前を入力してください")
                        .font(egui::TextStyle::Body),
                );
                ui.add_space(2.0);
                ui.small("ギルドの最高責任者・あなたの名前です（20文字以内）");
            });

            ui.add_space(10.0);

            // 3. 開業地域
            ui.group(|ui| {
                ui.label(egui::RichText::new("📍 開業地域").strong().size(15.0));
                ui.add_space(4.0);

                let preset_locations = ["王都", "港町", "辺境の砦", "学術都市", "交易都市"];
                ui.horizontal_wrapped(|ui| {
                    for loc in &preset_locations {
                        if ui.selectable_label(form.location == *loc, *loc).clicked() {
                            form.location = (*loc).to_string();
                        }
                    }
                });
                ui.add_space(4.0);
                ui.small("自由入力:");
                ui.add(
                    egui::TextEdit::singleline(&mut form.location)
                        .desired_width(max_width - 30.0)
                        .hint_text("任意の地域名を入力可能")
                        .font(egui::TextStyle::Body),
                );
            });

            ui.add_space(10.0);

            // 4. 乱数シード（任意）
            ui.collapsing("🎲 乱数シード（任意設定）", |ui| {
                ui.add_space(4.0);
                ui.add(
                    egui::TextEdit::singleline(&mut form.seed_input)
                        .desired_width(220.0)
                        .hint_text("空欄 = 毎回ランダム")
                        .font(egui::TextStyle::Body),
                );
                ui.small("特定のシード値を指定すると同じ乱数展開を再現できます。空欄の場合は現在時刻から自動決定されます。");
            });

            ui.add_space(16.0);

            // エラーメッセージ表示
            if let Some(err) = &form.error_message {
                ui.colored_label(egui::Color32::from_rgb(255, 100, 100), format!("⚠️ {err}"));
                ui.add_space(8.0);
            }

            // 操作ボタン
            ui.horizontal(|ui| {
                let start_btn = egui::Button::new(
                    egui::RichText::new("✅ この内容でギルドを開業する").size(16.0).strong(),
                )
                .min_size(egui::vec2(260.0, 42.0));

                if ui.add(start_btn).clicked() {
                    if let Some(err) = form.validate() {
                        form.error_message = Some(err);
                    } else {
                        form.error_message = None;
                        action = SetupAction::Start;
                    }
                }

                ui.add_space(12.0);

                let back_btn = egui::Button::new(
                    egui::RichText::new("◀ タイトルに戻る").size(14.0),
                )
                .min_size(egui::vec2(140.0, 42.0));

                if ui.add(back_btn).clicked() {
                    action = SetupAction::Back;
                }
            });

            ui.add_space(20.0);
        });
    });

    action
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup_form_validation() {
        let mut form = SetupForm::default();
        // 初期状態（master_name が空）ではバリデーションエラー
        assert!(form.validate().is_some());

        // 正常な入力
        form.master_name = "アシュレイ".to_string();
        assert!(form.validate().is_none());

        // ギルド名が空
        form.guild_name = "   ".to_string();
        assert!(form.validate().is_some());
        form.guild_name = "新生ギルド".to_string();

        // ギルド名が長すぎる（31文字）
        form.guild_name = "あ".repeat(31);
        assert!(form.validate().is_some());
        form.guild_name = "新生ギルド".to_string();

        // マスター名が長すぎる（21文字）
        form.master_name = "い".repeat(21);
        assert!(form.validate().is_some());
        form.master_name = "マスター".to_string();

        // シード値が不正な文字列
        form.seed_input = "invalid_seed".to_string();
        assert!(form.validate().is_some());

        // シード値が正の整数
        form.seed_input = "12345".to_string();
        assert!(form.validate().is_none());
        assert_eq!(form.seed(), 12345);
    }
}
