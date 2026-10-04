use crate::ui::config::{UserSettings, WindowSizePreset, SETTINGS_FILE_PATH};
use eframe::egui;

/// 設定メニューの描画結果（app.rs 側で処理するアクション）
pub enum SettingsAction {
    /// 何もしない
    None,
    /// ウィンドウを閉じる
    Close,
}

/// 設定メニュー（モーダルウィンドウ）の描画
///
/// Escキーまたはヘッダーボタンから開閉。
/// `egui::Window::open()` に `&mut bool` を渡すと借用チェッカーと衝突するため、
/// 閉じる制御は `SettingsAction::Close` を返す方式で回避している。
pub fn render(
    ctx: &egui::Context,
    is_open: bool,
    settings: &mut UserSettings,
    status_msg: &mut Option<String>,
) -> SettingsAction {
    if !is_open {
        return SettingsAction::None;
    }

    let mut action = SettingsAction::None;
    let mut should_apply_viewport = false;

    egui::Window::new("⚙️ 環境設定・システム")
        .collapsible(false)
        .resizable(false)
        .default_size([540.0, 520.0])
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add_space(4.0);

                // ─── 1. 画面・ウィンドウ設定 ───────────────────────
                ui.group(|ui| {
                    ui.heading("🖥️ 画面・ウィンドウ設定");
                    ui.separator();

                    // 解像度プリセット
                    ui.label("解像度プリセットを選択:");
                    ui.horizontal_wrapped(|ui| {
                        for preset in WindowSizePreset::ALL {
                            let is_current = settings.window_preset == preset;
                            if ui.selectable_label(is_current, preset.label()).clicked() {
                                settings.window_preset = preset;
                                should_apply_viewport = true;
                                *status_msg = Some(format!(
                                    "📐 解像度を {} に変更しました",
                                    preset.label()
                                ));
                            }
                        }
                    });

                    ui.add_space(6.0);

                    // フルスクリーン切替
                    let mut fs = settings.fullscreen;
                    if ui.checkbox(&mut fs, "フルスクリーン表示").changed() {
                        settings.fullscreen = fs;
                        should_apply_viewport = true;
                    }

                    ui.add_space(6.0);

                    // UIズーム倍率
                    ui.horizontal(|ui| {
                        ui.label("UI拡大倍率 (ズーム):");
                        let prev_zoom = settings.zoom_factor;
                        ui.add(
                            egui::Slider::new(&mut settings.zoom_factor, 0.7..=2.0).text("倍率"),
                        );
                        if (settings.zoom_factor - prev_zoom).abs() > 0.001 {
                            ctx.set_zoom_factor(settings.zoom_factor);
                        }
                        if ui.button("100%").clicked() {
                            settings.zoom_factor = 1.0;
                            ctx.set_zoom_factor(1.0);
                        }
                    });
                    ui.small(
                        "※ Ctrl + プラス/マイナス/ゼロ キーでも手元で素早く拡大縮小可能です。",
                    );
                });

                ui.add_space(8.0);

                // ─── 2. 表示テーマ設定 ──────────────────────────
                ui.group(|ui| {
                    ui.heading("🎨 表示テーマ");
                    ui.separator();

                    ui.horizontal(|ui| {
                        let label = if settings.dark_mode {
                            "🌙 ダークモード（現在有効）"
                        } else {
                            "☀️ ライトモード（現在有効）"
                        };
                        if ui.checkbox(&mut settings.dark_mode, label).changed() {
                            if settings.dark_mode {
                                ctx.set_visuals(egui::Visuals::dark());
                            } else {
                                ctx.set_visuals(egui::Visuals::light());
                            }
                        }
                    });
                    ui.small(
                        "テーマの変更は即時反映されます。設定を保存すると次回起動時にも適用されます。",
                    );
                });

                ui.add_space(8.0);

                // ─── 3. 音響・サウンド設定（将来拡張用スロット）───────────
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.heading("🔊 音響設定");
                        ui.colored_label(
                            egui::Color32::from_rgb(150, 150, 150),
                            "（将来のBGM/SE実装時に有効化）",
                        );
                    });
                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.label("BGM 音量: ");
                        let muted = settings.is_muted;
                        ui.add_enabled(
                            !muted,
                            egui::Slider::new(&mut settings.bgm_volume, 0..=100).suffix("%"),
                        );
                    });

                    ui.horizontal(|ui| {
                        ui.label("SE 音量:   ");
                        let muted = settings.is_muted;
                        ui.add_enabled(
                            !muted,
                            egui::Slider::new(&mut settings.se_volume, 0..=100).suffix("%"),
                        );
                    });

                    ui.checkbox(&mut settings.is_muted, "🔇 サウンドをミュート（消音）");
                });

                ui.add_space(8.0);

                // ─── 4. ゲームプレイ設定 ────────────────────────
                ui.group(|ui| {
                    ui.heading("🕹️ ゲームプレイ設定");
                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.label("オートセーブ間隔:");
                        ui.add(
                            egui::Slider::new(&mut settings.auto_save_interval_days, 0..=30)
                                .suffix(" 日ごと"),
                        );
                    });
                    if settings.auto_save_interval_days == 0 {
                        ui.small("⚠️ オートセーブが無効です。手動セーブを忘れずに！");
                    } else {
                        ui.small(&format!(
                            "✅ {} 日ごとに自動保存されます。",
                            settings.auto_save_interval_days
                        ));
                    }
                });

                ui.add_space(12.0);

                // ─── ボタン行（保存・リセット・閉じる）─────────────────
                ui.horizontal(|ui| {
                    // 設定保存ボタン
                    if ui
                        .button(egui::RichText::new("💾 設定を保存する").strong())
                        .on_hover_text("現在の設定をファイルに保存します")
                        .clicked()
                    {
                        match settings.save_to_file(SETTINGS_FILE_PATH) {
                            Ok(()) => {
                                *status_msg = Some(format!(
                                    "💾 設定を保存しました ({})",
                                    SETTINGS_FILE_PATH
                                ));
                            }
                            Err(e) => {
                                *status_msg = Some(format!("❌ 設定保存失敗: {}", e));
                            }
                        }
                    }

                    ui.separator();

                    // デフォルト復元ボタン
                    if ui
                        .button("🔄 初期設定に戻す")
                        .on_hover_text("すべての設定をデフォルト値にリセットします")
                        .clicked()
                    {
                        *settings = UserSettings::default();
                        ctx.set_zoom_factor(settings.zoom_factor);
                        ctx.set_visuals(egui::Visuals::dark());
                        should_apply_viewport = true;
                        *status_msg = Some("🔄 設定を初期値にリセットしました".to_string());
                    }

                    ui.separator();

                    // 閉じるボタン
                    if ui.button("閉じる (Esc)").clicked() {
                        action = SettingsAction::Close;
                    }
                });

                ui.add_space(4.0);
            });
        });

    // ウィンドウサイズ変更の即時適用
    if should_apply_viewport {
        let [w, h] = settings.window_preset.size();
        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(w, h)));
        ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(settings.fullscreen));
    }

    action
}
