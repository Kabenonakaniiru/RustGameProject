use eframe::egui;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    GuildOverview,     // ギルド情報
    QuestsAndDispatch, // クエスト・ダンジョン派遣
    Adventurers,       // 冒険者管理
    MarketAndTrading,  // 倉庫・売買（素材・部位の売却）
    StaffAndFacility,  // 職員・施設管理（人件費・税金・光熱費）
    FinancialReport,   // 収支台帳
}

#[derive(Debug, Clone)]
pub struct Adventurer {
    pub name: String,
    pub rank: String,
    pub class_name: String,
    pub level: u32,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct InventoryItem {
    pub name: String,
    pub category: String,
    pub count: u32,
    pub buy_price: i64,   // 冒険者から買い取った単価
    pub market_price: i64, // 市場への売却想定単価
}

#[derive(Debug, Clone)]
pub struct Quest {
    pub title: String,
    pub location: String,
    pub difficulty: String,
    pub reward_estimate: i64,
    pub days_required: u32,
    pub days_remaining: u32,
    pub is_dispatched: bool,
    pub assigned_party: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Staff {
    pub name: String,
    pub role: String,
    pub salary: i64, // 日給
}

pub struct GuildApp {
    // 基本ステータス
    pub gold: i64,
    pub day: u32,
    pub rank: String,
    pub reputation: u32,

    // 支出設定（日額）
    pub daily_tax: i64,      // 固定資産税（日換算）
    pub daily_utility: i64,  // 光熱費
    pub current_tab: Tab,

    // 各種データ
    pub adventurers: Vec<Adventurer>,
    pub inventory: Vec<InventoryItem>,
    pub quests: Vec<Quest>,
    pub staff_list: Vec<Staff>,

    // 累積収支記録（今日／累計）
    pub total_sales_revenue: i64, // 売却売上
    pub total_commission: i64,    // 手数料収入
    pub total_expenses: i64,      // 支出合計

    // イベントログ
    pub logs: Vec<String>,
}

impl GuildApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_fonts(&cc.egui_ctx);

        let adventurers = vec![
            Adventurer {
                name: "アレン".to_string(),
                rank: "C".to_string(),
                class_name: "剣士".to_string(),
                level: 14,
                status: "待機中".to_string(),
            },
            Adventurer {
                name: "エレナ".to_string(),
                rank: "C".to_string(),
                class_name: "魔導士".to_string(),
                level: 13,
                status: "待機中".to_string(),
            },
            Adventurer {
                name: "ボリス".to_string(),
                rank: "D".to_string(),
                class_name: "重戦士".to_string(),
                level: 9,
                status: "待機中".to_string(),
            },
            Adventurer {
                name: "リナ".to_string(),
                rank: "D".to_string(),
                class_name: "盗賊".to_string(),
                level: 10,
                status: "待機中".to_string(),
            },
            Adventurer {
                name: "クルト".to_string(),
                rank: "E".to_string(),
                class_name: "見習い狩人".to_string(),
                level: 3,
                status: "待機中".to_string(),
            },
        ];

        let inventory = vec![
            InventoryItem {
                name: "ゴブリンの爪".to_string(),
                category: "魔物部位".to_string(),
                count: 12,
                buy_price: 30,
                market_price: 65,
            },
            InventoryItem {
                name: "低級魔導石".to_string(),
                category: "魔導素材".to_string(),
                count: 5,
                buy_price: 150,
                market_price: 320,
            },
            InventoryItem {
                name: "薬草（上質）".to_string(),
                category: "薬草・植物".to_string(),
                count: 20,
                buy_price: 20,
                market_price: 50,
            },
            InventoryItem {
                name: "オオカミの毛皮".to_string(),
                category: "毛皮・皮革".to_string(),
                count: 4,
                buy_price: 80,
                market_price: 180,
            },
        ];

        let quests = vec![
            Quest {
                title: "ゴブリンの巣窟掃討".to_string(),
                location: "薄暗い洞窟".to_string(),
                difficulty: "D".to_string(),
                reward_estimate: 800,
                days_required: 2,
                days_remaining: 0,
                is_dispatched: false,
                assigned_party: None,
            },
            Quest {
                title: "上級魔導石の探索発掘".to_string(),
                location: "深淵の鉱山跡".to_string(),
                difficulty: "C".to_string(),
                reward_estimate: 2400,
                days_required: 3,
                days_remaining: 0,
                is_dispatched: false,
                assigned_party: None,
            },
            Quest {
                title: "街道周辺の野獣間引き".to_string(),
                location: "王都街道".to_string(),
                difficulty: "E".to_string(),
                reward_estimate: 400,
                days_required: 1,
                days_remaining: 0,
                is_dispatched: false,
                assigned_party: None,
            },
        ];

        let staff_list = vec![
            Staff {
                name: "マリア".to_string(),
                role: "受付主任".to_string(),
                salary: 120,
            },
            Staff {
                name: "ハンス".to_string(),
                role: "鑑定士".to_string(),
                salary: 180,
            },
            Staff {
                name: "ゴードン".to_string(),
                role: "倉庫番兼警備".to_string(),
                salary: 100,
            },
        ];

        let mut logs = Vec::new();
        logs.push("【1日目】冒険者ギルドの営業を開始しました。".to_string());
        logs.push("【情報】固定資産税・職員人件費・光熱費は毎日計上されます。".to_string());

        Self {
            gold: 10000,
            day: 1,
            rank: "E".to_string(),
            reputation: 100,
            daily_tax: 150,
            daily_utility: 80,
            current_tab: Tab::GuildOverview,
            adventurers,
            inventory,
            quests,
            staff_list,
            total_sales_revenue: 0,
            total_commission: 0,
            total_expenses: 0,
            logs,
        }
    }

    /// 職員人件費の日額合計
    pub fn daily_staff_salary(&self) -> i64 {
        self.staff_list.iter().map(|s| s.salary).sum()
    }

    /// 1日あたりの固定支出合計
    pub fn daily_total_expenses(&self) -> i64 {
        self.daily_tax + self.daily_utility + self.daily_staff_salary()
    }

    /// 1日進める処理
    pub fn advance_day(&mut self) {
        self.day += 1;
        let expense = self.daily_total_expenses();
        self.gold -= expense;
        self.total_expenses += expense;

        self.logs.push(format!(
            "【{}日目】固定経費（人件費:{}G, 税金:{}G, 光熱費:{}G）計 {}G を支払いました。",
            self.day,
            self.daily_staff_salary(),
            self.daily_tax,
            self.daily_utility,
            expense
        ));

        // 派遣中クエストの進行処理
        for quest in &mut self.quests {
            if quest.is_dispatched {
                if quest.days_remaining > 1 {
                    quest.days_remaining -= 1;
                } else {
                    quest.is_dispatched = false;
                    quest.days_remaining = 0;
                    let party_name = quest.assigned_party.take().unwrap_or_else(|| "冒険者".to_string());

                    // 成果の精算（ダミーの仲介手数料と買取アイテム獲得）
                    let commission = (quest.reward_estimate as f64 * 0.15) as i64; // ギルド手数料15%
                    self.gold += commission;
                    self.total_commission += commission;

                    self.logs.push(format!(
                        "【帰還報告】{} が『{}』より無事帰還！ 仲介手数料 {}G を受領しました。",
                        party_name, quest.title, commission
                    ));

                    // 戦利品をギルド倉庫に買取追加（ダミー）
                    self.inventory.push(InventoryItem {
                        name: "採取された魔導鉱石".to_string(),
                        category: "鉱石".to_string(),
                        count: 2,
                        buy_price: 120,
                        market_price: 280,
                    });
                }
            }
        }

        // 冒険者のステータス更新（クエスト連動）
        for adv in &mut self.adventurers {
            if adv.status == "派遣中" {
                // 該当クエストが終わっていれば待機中に戻す（簡易処理）
                let still_on_quest = self.quests.iter().any(|q| {
                    q.is_dispatched && q.assigned_party.as_deref() == Some(&adv.name)
                });
                if !still_on_quest {
                    adv.status = "待機中".to_string();
                }
            }
        }

        // ログの最新保持（最大50件）
        if self.logs.len() > 50 {
            self.logs.remove(0);
        }
    }
}

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/ipag.ttf")).into(),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());
    ctx.set_fonts(fonts);
}

impl eframe::App for GuildApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // 1. トップパネル（ヘッダーバー）
        // 上段: タイトル＋ボタン、下段: ステータス情報 の2段構成で横幅競合を回避
        egui::Panel::top("header_panel")
            .frame(
                egui::Frame::side_top_panel(ui.style())
                    .inner_margin(egui::Margin::symmetric(12, 6)),
            )
            .show(ui, |ui| {
                // 上段: タイトル（左）＋ 1日進めるボタン（右端）
                ui.horizontal(|ui| {
                    ui.heading("⚔ 冒険者ギルド運営 ⚔");

                    // 右端にボタンを配置（上段はタイトルとボタンだけなので絶対に埋もれない）
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("  ▶ 1日進める  ").clicked() {
                            self.advance_day();
                        }
                    });
                });

                ui.add_space(2.0);

                // 下段: ステータス情報（所持金・日付・ランク・日次固定費）
                ui.horizontal(|ui| {
                    ui.strong(format!("💰 所持金: {} G", self.gold));
                    ui.separator();
                    ui.label(format!("📅 日付: {} 日目", self.day));
                    ui.separator();
                    ui.label(format!("🏅 ランク: {} (評判: {})", self.rank, self.reputation));
                    ui.separator();
                    ui.colored_label(
                        egui::Color32::from_rgb(255, 120, 120),
                        format!("🔻 日次固定費: -{} G/日", self.daily_total_expenses()),
                    );
                });
            });

        // 2. ボトムパネル（フッター: 直近イベントログ）
        egui::Panel::bottom("footer_panel")
            .resizable(true)
            .default_size(120.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("📜 【ギルド日誌・業務ログ】");
                });
                egui::ScrollArea::vertical()
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        for log in self.logs.iter().rev().take(8) {
                            ui.label(log);
                        }
                    });
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
                    self.show_guild_overview(ui);
                }
                Tab::QuestsAndDispatch => {
                    self.show_quests_and_dispatch(ui);
                }
                Tab::Adventurers => {
                    self.show_adventurers(ui);
                }
                Tab::MarketAndTrading => {
                    self.show_market_and_trading(ui);
                }
                Tab::StaffAndFacility => {
                    self.show_staff_and_facility(ui);
                }
                Tab::FinancialReport => {
                    self.show_financial_report(ui);
                }
            });
        });
    }
}

impl GuildApp {
    fn show_guild_overview(&mut self, ui: &mut egui::Ui) {
        ui.heading("🏛 ギルド基本情報・執務室");
        ui.separator();

        ui.label(format!("ギルド名称: 王都冒険者ギルド第3支部"));
        ui.label(format!("ギルドランク: {} ランク", self.rank));
        ui.label(format!("ギルド名声: {} pt", self.reputation));
        ui.label(format!("所属冒険者数: {} 名", self.adventurers.len()));
        ui.label(format!("雇用職員数: {} 名", self.staff_list.len()));

        ui.add_space(12.0);
        ui.group(|ui| {
            ui.heading("💰 財務ハイライト");
            ui.label(format!("現在の手元資金: {} G", self.gold));
            ui.label(format!("累計素材売却益: +{} G", self.total_sales_revenue));
            ui.label(format!("累計仲介手数料: +{} G", self.total_commission));
            ui.label(format!("累計固定支出費: -{} G", self.total_expenses));
        });
    }

    fn show_quests_and_dispatch(&mut self, ui: &mut egui::Ui) {
        ui.heading("📋 受付・クエスト管理＆派遣");
        ui.separator();
        ui.label("冒険者にクエストを斡旋・派遣します。帰還時に仲介手数料と戦利品の買取機会が得られます。");
        ui.add_space(8.0);

        let available_adventurers: Vec<String> = self
            .adventurers
            .iter()
            .filter(|a| a.status == "待機中")
            .map(|a| a.name.clone())
            .collect();

        for i in 0..self.quests.len() {
            let quest = &mut self.quests[i];
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.strong(&quest.title);
                    ui.label(format!("【危険度: {}】", quest.difficulty));
                    ui.label(format!("場所: {}", quest.location));
                    ui.label(format!("所要日数: {}日", quest.days_required));
                    ui.label(format!("報酬目安: {} G", quest.reward_estimate));
                });

                ui.horizontal(|ui| {
                    if quest.is_dispatched {
                        ui.colored_label(
                            egui::Color32::from_rgb(255, 180, 50),
                            format!(
                                "⏳ 派遣中 (担当: {}, 残り: {}日)",
                                quest.assigned_party.as_deref().unwrap_or("不明"),
                                quest.days_remaining
                            ),
                        );
                    } else {
                        ui.label("状態: 待機中（未派遣）");
                        if !available_adventurers.is_empty() {
                            let candidate = &available_adventurers[0];
                            if ui.button(format!("{} を派遣する", candidate)).clicked() {
                                quest.is_dispatched = true;
                                quest.days_remaining = quest.days_required;
                                quest.assigned_party = Some(candidate.clone());
                            }
                        } else {
                            ui.label("（派遣可能な待機中冒険者がいません）");
                        }
                    }
                });
            });
            ui.add_space(4.0);
        }

        // 派遣中の冒険者のステータスを更新
        for quest in &self.quests {
            if quest.is_dispatched {
                if let Some(party) = &quest.assigned_party {
                    for adv in &mut self.adventurers {
                        if &adv.name == party {
                            adv.status = "派遣中".to_string();
                        }
                    }
                }
            }
        }
    }

    fn show_adventurers(&mut self, ui: &mut egui::Ui) {
        ui.heading("⚔ 冒険者管理（酒場・宿舎）");
        ui.separator();
        ui.label("ギルドに登録されている冒険者の一覧です。");
        ui.add_space(8.0);

        egui::Grid::new("adventurers_grid")
            .striped(true)
            .min_col_width(100.0)
            .show(ui, |ui| {
                ui.strong("冒険者名");
                ui.strong("ランク");
                ui.strong("職業");
                ui.strong("Lv");
                ui.strong("状態");
                ui.end_row();

                for adv in &self.adventurers {
                    ui.label(&adv.name);
                    ui.label(&adv.rank);
                    ui.label(&adv.class_name);
                    ui.label(format!("{}", adv.level));
                    if adv.status == "待機中" {
                        ui.colored_label(egui::Color32::from_rgb(100, 220, 100), &adv.status);
                    } else {
                        ui.colored_label(egui::Color32::from_rgb(255, 180, 50), &adv.status);
                    }
                    ui.end_row();
                }
            });
    }

    fn show_market_and_trading(&mut self, ui: &mut egui::Ui) {
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

                for (idx, item) in self.inventory.iter().enumerate() {
                    let profit_per_unit = item.market_price - item.buy_price;
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

        // 1個売却処理
        if let Some(idx) = sold_index {
            if idx < self.inventory.len() {
                let item = &mut self.inventory[idx];
                let revenue = item.market_price;
                self.gold += revenue;
                self.total_sales_revenue += revenue;
                self.logs.push(format!(
                    "【売却】『{}』を1個市場へ売却し、{} G を得ました。",
                    item.name, revenue
                ));
                item.count -= 1;
                if item.count == 0 {
                    self.inventory.remove(idx);
                }
            }
        }

        // 一括売却処理
        if sell_all {
            let mut total_earn = 0;
            for item in &self.inventory {
                total_earn += item.market_price * item.count as i64;
            }
            if total_earn > 0 {
                self.gold += total_earn;
                self.total_sales_revenue += total_earn;
                self.logs.push(format!(
                    "【一括売却】全在庫を市場へ売却し、合計 {} G を得ました！",
                    total_earn
                ));
                self.inventory.clear();
            }
        }
    }

    fn show_staff_and_facility(&mut self, ui: &mut egui::Ui) {
        ui.heading("👥 職員・施設・維持費管理（支出項目）");
        ui.separator();
        ui.label("ギルドの運営には日々の人件費、固定資産税、光熱費がかかります。");
        ui.add_space(8.0);

        ui.group(|ui| {
            ui.heading("🏢 固定資産・施設維持費（日額）");
            ui.horizontal(|ui| {
                ui.label("ギルド本部施設 固定資産税（日換算）:");
                ui.strong(format!("{} G / 日", self.daily_tax));
            });
            ui.horizontal(|ui| {
                ui.label("ギルド本部 光熱費・水道代:");
                ui.strong(format!("{} G / 日", self.daily_utility));
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

                    for staff in &self.staff_list {
                        ui.label(&staff.name);
                        ui.label(&staff.role);
                        ui.label(format!("{} G / 日", staff.salary));
                        ui.end_row();
                    }
                });

            ui.add_space(4.0);
            ui.label(format!(
                "職員人件費 合計: {} G / 日",
                self.daily_staff_salary()
            ));
        });

        ui.add_space(8.0);
        ui.colored_label(
            egui::Color32::from_rgb(255, 120, 120),
            format!(
                "1日あたりの固定経費総計: {} G（毎日の日付変更時に自動引き落とし）",
                self.daily_total_expenses()
            ),
        );
    }

    fn show_financial_report(&mut self, ui: &mut egui::Ui) {
        ui.heading("📊 ギルド収支台帳・経営レポート");
        ui.separator();
        ui.label("ギルドの総収入と総支出の内訳です。");
        ui.add_space(12.0);

        let total_income = self.total_sales_revenue + self.total_commission;
        let net_profit = total_income - self.total_expenses;

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
                    format!("+{} G", self.total_sales_revenue),
                );
                ui.end_row();

                ui.label("📝 クエスト仲介手数料（累計）");
                ui.colored_label(
                    egui::Color32::from_rgb(100, 220, 100),
                    format!("+{} G", self.total_commission),
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
                    format!("-{} G", self.total_expenses),
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
}

fn main() -> eframe::Result<()> {
    // VS Code DevContainer / WSLg 環境で Wayland ソケットの接続エラーを回避するため X11 バックエンドを優先
    if std::env::var("DISPLAY").is_ok() {
        unsafe {
            std::env::remove_var("WAYLAND_DISPLAY");
            std::env::set_var("WINIT_UNIX_BACKEND", "x11");
        }
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("冒険者ギルド経営シミュレーション")
            .with_decorations(true)
            .with_inner_size([1400.0, 800.0])
            .with_min_inner_size([1000.0, 650.0]),
        ..Default::default()
    };

    eframe::run_native(
        "冒険者ギルド経営シミュレーション",
        options,
        Box::new(|cc| Ok(Box::new(GuildApp::new(cc)))),
    )
}
