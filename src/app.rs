use crate::core::{self, GameState};
use crate::ui::config::{UserSettings, SETTINGS_FILE_PATH};
use crate::ui::{views, Tab};
use eframe::egui;

const SAVE_FILE_PATH: &str = "save_data.json";

#[derive(Debug)]
pub struct GuildApp {
    pub state: GameState,
    pub current_tab: Tab,
    pub status_message: Option<String>,
    pub settings: UserSettings,
    pub is_settings_open: bool,
}

impl Default for GuildApp {
    fn default() -> Self {
        Self::new()
    }
}

impl GuildApp {
    pub fn new() -> Self {
        let settings = UserSettings::load_from_file_or_default(SETTINGS_FILE_PATH);
        Self {
            state: GameState::new_default(),
            current_tab: Tab::GuildOverview,
            status_message: None,
            settings,
            is_settings_open: false,
        }
    }

    pub fn save(&mut self) {
        match core::save_game(&self.state, SAVE_FILE_PATH) {
            Ok(()) => {
                let msg = format!("💾 データを保存しました ({SAVE_FILE_PATH})");
                self.state.add_log(&msg);
                self.status_message = Some(msg);
            }
            Err(e) => {
                let msg = format!("❌ セーブ失敗: {e}");
                self.status_message = Some(msg);
            }
        }
    }

    pub fn load(&mut self) {
        match core::load_game(SAVE_FILE_PATH) {
            Ok(loaded_state) => {
                self.state = loaded_state;
                let msg = format!("📂 データを読み込みました ({SAVE_FILE_PATH})");
                self.state.add_log(&msg);
                self.status_message = Some(msg);
            }
            Err(e) => {
                let msg = format!("❌ ロード失敗: {e}");
                self.status_message = Some(msg);
            }
        }
    }
}

impl eframe::App for GuildApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // キーボードショートカット
        ui.input(|i| {
            // Esc キーで設定メニューの開閉をトグル
            if i.key_pressed(egui::Key::Escape) {
                self.is_settings_open = !self.is_settings_open;
            }

            // ズーム操作ショートカット (Ctrl + '+' / '-' / '0')
            if i.modifiers.command || i.modifiers.ctrl {
                if i.key_pressed(egui::Key::Plus) || i.key_pressed(egui::Key::Equals) {
                    self.settings.zoom_factor = (self.settings.zoom_factor + 0.1).min(2.0);
                } else if i.key_pressed(egui::Key::Minus) {
                    self.settings.zoom_factor = (self.settings.zoom_factor - 0.1).max(0.7);
                } else if i.key_pressed(egui::Key::Num0) {
                    self.settings.zoom_factor = 1.0;
                }
            }
        });

        // UI全体のズーム倍率を反映
        ui.ctx().set_zoom_factor(self.settings.zoom_factor);

        // 1. トップパネル（ヘッダーバー: 状況サマリー＆ズーム操作）
        egui::Panel::top("header_panel")
            .frame(egui::Frame::new())
            .show(ui, |ui| {
                let action = crate::ui::components::header::render(
                    ui,
                    &self.state,
                    self.status_message.as_deref(),
                    self.settings.zoom_factor,
                );
                match action {
                    crate::ui::components::header::HeaderAction::AdvanceDay => {
                        self.state.advance_day();
                        self.status_message = None;
                    }
                    crate::ui::components::header::HeaderAction::Save => {
                        self.save();
                    }
                    crate::ui::components::header::HeaderAction::Load => {
                        self.load();
                    }
                    crate::ui::components::header::HeaderAction::OpenSettings => {
                        self.is_settings_open = true;
                    }
                    crate::ui::components::header::HeaderAction::ZoomIn => {
                        self.settings.zoom_factor = (self.settings.zoom_factor + 0.1).min(2.0);
                    }
                    crate::ui::components::header::HeaderAction::ZoomOut => {
                        self.settings.zoom_factor = (self.settings.zoom_factor - 0.1).max(0.7);
                    }
                    crate::ui::components::header::HeaderAction::ResetZoom => {
                        self.settings.zoom_factor = 1.0;
                    }
                    crate::ui::components::header::HeaderAction::None => {}
                }
            });

        // 2. ボトムパネル（フッター: 直近イベントログ＆解像度表示、リサイズ可）
        egui::Panel::bottom("footer_panel")
            .resizable(true)
            .default_size(100.0)
            .min_size(55.0)
            .max_size(250.0)
            .frame(egui::Frame::new())
            .show(ui, |ui| {
                crate::ui::components::footer::render(ui, &self.state);
            });

        // 3. レフトサイドパネル（業務ナビゲーション、リサイズ可）
        egui::Panel::left("navigation_panel")
            .resizable(true)
            .default_size(200.0)
            .min_size(150.0)
            .max_size(300.0)
            .show(ui, |ui| {
                ui.add_space(8.0);
                ui.label("【 ギルド業務メニュー 】");
                ui.add_space(4.0);

                for &tab in &Tab::ALL {
                    if ui
                        .selectable_label(self.current_tab == tab, tab.label())
                        .clicked()
                    {
                        self.current_tab = tab;
                    }
                }

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(8.0);
                ui.label("【経営アドバイス】");
                ui.small("冒険者を派遣して素材を買い取り、市場へ売却して利益を上げましょう。固定費の滞納に注意！");

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(4.0);

                // いつでも開ける環境設定ボタン
                if ui
                    .button("⚙️ 環境設定メニュー (Esc)")
                    .on_hover_text("ウィンドウサイズや各種設定を変更します")
                    .clicked()
                {
                    self.is_settings_open = true;
                }

                ui.add_space(4.0);
                ui.small("💡 画面調整: パネル境界をドラッグしてサイズ変更可能です。");
            });

        // 4. セントラルパネル（選択中タブの詳細画面、縦横スクロール追従）
        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::both().show(ui, |ui| match self.current_tab {
                Tab::GuildOverview => {
                    views::overview::render(ui, &mut self.state);
                }
                Tab::QuestsAndDispatch => {
                    views::quests::render(ui, &mut self.state);
                }
                Tab::Adventurers => {
                    views::adventurers::render(ui, &mut self.state);
                }
                Tab::MarketAndTrading => {
                    views::market::render(ui, &mut self.state);
                }
                Tab::StaffAndFacility => {
                    views::staff::render(ui, &mut self.state);
                }
                Tab::FinancialReport => {
                    views::finance::render(ui, &mut self.state);
                }
            });
        });

        // 5. 設定メニュー（モーダルウィンドウ: どの画面からでも最前面に展開可能）
        let settings_action = crate::ui::components::settings::render(
            ui.ctx(),
            self.is_settings_open,
            &mut self.settings,
            &mut self.status_message,
        );
        match settings_action {
            crate::ui::components::settings::SettingsAction::Close => {
                self.is_settings_open = false;
            }
            crate::ui::components::settings::SettingsAction::None => {}
        }
    }
}
