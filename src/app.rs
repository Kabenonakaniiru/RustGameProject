use eframe::egui;
use crate::core::{self, GameState};
use crate::ui::{views, Tab};

const SAVE_FILE_PATH: &str = "save_data.json";

pub struct GuildApp {
    pub state: GameState,
    pub current_tab: Tab,
    pub status_message: Option<String>,
}

impl GuildApp {
    pub fn new() -> Self {
        Self {
            state: GameState::new_default(),
            current_tab: Tab::GuildOverview,
            status_message: None,
        }
    }

    pub fn save(&mut self) {
        match core::save_game(&self.state, SAVE_FILE_PATH) {
            Ok(()) => {
                let msg = format!("💾 データを保存しました ({})", SAVE_FILE_PATH);
                self.state.add_log(&msg);
                self.status_message = Some(msg);
            }
            Err(e) => {
                let msg = format!("❌ セーブ失敗: {}", e);
                self.status_message = Some(msg);
            }
        }
    }

    pub fn load(&mut self) {
        match core::load_game(SAVE_FILE_PATH) {
            Ok(loaded_state) => {
                self.state = loaded_state;
                let msg = format!("📂 データを読み込みました ({})", SAVE_FILE_PATH);
                self.state.add_log(&msg);
                self.status_message = Some(msg);
            }
            Err(e) => {
                let msg = format!("❌ ロード失敗: {}", e);
                self.status_message = Some(msg);
            }
        }
    }
}

impl eframe::App for GuildApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // 1. トップパネル（ヘッダーバー）
        // 1. トップパネル（ヘッダーバー）
        egui::Panel::top("header_panel")
            .frame(egui::Frame::new())
            .show(ui, |ui| {
                let action = crate::ui::components::header::render(
                    ui,
                    &self.state,
                    self.status_message.as_deref(),
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
                    crate::ui::components::header::HeaderAction::None => {}
                }
            });

        // 2. ボトムパネル（フッター: 直近イベントログ）
        egui::Panel::bottom("footer_panel")
            .frame(egui::Frame::new())
            .show(ui, |ui| {
                crate::ui::components::footer::render(ui, &self.state);
            });

        // 3. レフトサイドパネル（業務ナビゲーション）
        egui::Panel::left("navigation_panel")
            .resizable(false)
            .default_size(200.0)
            .show(ui, |ui| {
                ui.add_space(8.0);
                ui.label("【 ギルド業務メニュー 】");
                ui.add_space(4.0);

                if ui
                    .selectable_label(self.current_tab == Tab::GuildOverview, "🏛 ギルド基本情報")
                    .clicked()
                {
                    self.current_tab = Tab::GuildOverview;
                }
                if ui
                    .selectable_label(
                        self.current_tab == Tab::QuestsAndDispatch,
                        "📋 受付・クエスト派遣",
                    )
                    .clicked()
                {
                    self.current_tab = Tab::QuestsAndDispatch;
                }
                if ui
                    .selectable_label(self.current_tab == Tab::Adventurers, "⚔ 冒険者管理")
                    .clicked()
                {
                    self.current_tab = Tab::Adventurers;
                }
                if ui
                    .selectable_label(
                        self.current_tab == Tab::MarketAndTrading,
                        "📦 倉庫・戦利品売買",
                    )
                    .clicked()
                {
                    self.current_tab = Tab::MarketAndTrading;
                }
                if ui
                    .selectable_label(
                        self.current_tab == Tab::StaffAndFacility,
                        "👥 職員雇用・施設維持",
                    )
                    .clicked()
                {
                    self.current_tab = Tab::StaffAndFacility;
                }
                if ui
                    .selectable_label(self.current_tab == Tab::FinancialReport, "📊 収支台帳")
                    .clicked()
                {
                    self.current_tab = Tab::FinancialReport;
                }

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(8.0);
                ui.label("【経営アドバイス】");
                ui.small("冒険者を派遣して素材を買い取り、市場へ売却して利益を上げましょう。固定費の滞納に注意！");
            });

        // 4. セントラルパネル（選択中タブの詳細画面）
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
    }
}
