use serde::{Deserialize, Serialize};
use super::adventurer::Adventurer;
use super::finance::Staff;
use super::inventory::InventoryItem;
use super::quest::Quest;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameState {
    // 基本ステータス
    pub gold: i64,
    pub day: u32,
    pub rank: String,
    pub reputation: u32,

    // 支出設定（日額）
    pub daily_tax: i64,     // 固定資産税（日換算）
    pub daily_utility: i64, // 光熱費

    // 各種データ
    pub adventurers: Vec<Adventurer>,
    pub inventory: Vec<InventoryItem>,
    pub quests: Vec<Quest>,
    pub staff_list: Vec<Staff>,

    // 累積収支記録（累計）
    pub total_sales_revenue: i64, // 売却売上
    pub total_commission: i64,    // 手数料収入
    pub total_expenses: i64,      // 支出合計

    // イベントログ
    pub logs: Vec<String>,
}

impl Default for GameState {
    fn default() -> Self {
        Self::new_default()
    }
}

impl GameState {
    /// 新規ゲームの初期状態を生成
    pub fn new_default() -> Self {
        let adventurers = vec![
            Adventurer::new("アレン", "C", "剣士", 14),
            Adventurer::new("エレナ", "C", "魔導士", 13),
            Adventurer::new("ボリス", "D", "重戦士", 9),
            Adventurer::new("リナ", "D", "盗賊", 10),
            Adventurer::new("クルト", "E", "見習い狩人", 3),
        ];

        let inventory = vec![
            InventoryItem::new("ゴブリンの爪", "魔物部位", 12, 30, 65),
            InventoryItem::new("低級魔導石", "魔導素材", 5, 150, 320),
            InventoryItem::new("薬草（上質）", "薬草・植物", 20, 20, 50),
            InventoryItem::new("オオカミの毛皮", "毛皮・皮革", 4, 80, 180),
        ];

        let quests = vec![
            Quest::new("ゴブリンの森の掃討", "南の森林", "D", 800, 1),
            Quest::new("コボルトの鉱山調査", "東部廃鉱山", "D", 1200, 2),
            Quest::new("ワイバーン警戒任務", "北の岩山街道", "C", 2500, 3),
            Quest::new("上級薬草の緊急採取依頼", "精霊の泉周辺", "E", 400, 1),
            Quest::new("古代遺跡地下の探索", "旧王都地下遺跡", "B", 6000, 5),
        ];

        let staff_list = vec![
            Staff::new("マルタ", "受付主任", 40),
            Staff::new("トマス", "倉庫番・鑑定士", 35),
            Staff::new("セリア", "事務・会計係", 30),
        ];

        let logs = vec![
            "【ギルド業務日誌】本日の業務を開始しました。".to_string(),
            "【通達】今月の固定資産税および光熱費は毎日自動引き落としされます。".to_string(),
            "【市場情報】魔導石の相場がわずかに上昇傾向にあります。".to_string(),
        ];

        Self {
            gold: 3500,
            day: 1,
            rank: "ブロンズ (Rank 2)".to_string(),
            reputation: 45,
            daily_tax: 25,
            daily_utility: 15,
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

        self.add_log(format!(
            "【{}日目】固定経費（人件費:{}G, 税金:{}G, 光熱費:{}G）計 {}G を支払いました。",
            self.day,
            self.daily_staff_salary(),
            self.daily_tax,
            self.daily_utility,
            expense
        ));

        // 派遣中クエストの進行処理
        struct CompletedQuest {
            party_name: String,
            title: String,
            commission: i64,
        }
        let mut completed = Vec::new();

        for quest in &mut self.quests {
            if quest.is_dispatched {
                if quest.days_remaining > 1 {
                    quest.days_remaining -= 1;
                } else {
                    quest.is_dispatched = false;
                    quest.days_remaining = 0;
                    let party_name = quest.assigned_party.take().unwrap_or_else(|| "冒険者".to_string());
                    let commission = quest.calculate_commission();
                    completed.push(CompletedQuest {
                        party_name,
                        title: quest.title.clone(),
                        commission,
                    });
                }
            }
        }

        for c in completed {
            self.gold += c.commission;
            self.total_commission += c.commission;
            self.add_log(format!(
                "【帰還報告】{} が『{}』より無事帰還！ 仲介手数料 {}G を受領しました。",
                c.party_name, c.title, c.commission
            ));
            self.inventory.push(InventoryItem::new(
                "採取された魔導鉱石",
                "鉱石",
                2,
                120,
                280,
            ));
        }

        // 冒険者のステータス更新（クエスト連動）
        for adv in &mut self.adventurers {
            if adv.status == "派遣中" {
                let still_on_quest = self.quests.iter().any(|q| {
                    q.is_dispatched && q.assigned_party.as_deref() == Some(&adv.name)
                });
                if !still_on_quest {
                    adv.status = "待機中".to_string();
                }
            }
        }
    }

    /// クエストに冒険者を派遣する
    pub fn dispatch_quest(&mut self, quest_idx: usize, adventurer_name: &str) -> bool {
        if quest_idx >= self.quests.len() {
            return false;
        }

        // 冒険者が待機中かチェック
        let adv_found = self
            .adventurers
            .iter_mut()
            .find(|a| a.name == adventurer_name && a.status == "待機中");

        if let Some(adv) = adv_found {
            adv.status = "派遣中".to_string();
            let (title, days_required) = {
                let quest = &mut self.quests[quest_idx];
                quest.dispatch(adventurer_name);
                (quest.title.clone(), quest.days_required)
            };
            self.add_log(format!(
                "【派遣】{} を『{}』へ派遣しました（所要: {}日）。",
                adventurer_name, title, days_required
            ));
            true
        } else {
            false
        }
    }

    /// 指定インデックスのアイテムを1個売却
    pub fn sell_item(&mut self, idx: usize) -> Option<i64> {
        if idx >= self.inventory.len() {
            return None;
        }

        let revenue = self.inventory[idx].market_price;
        let item_name = self.inventory[idx].name.clone();
        self.gold += revenue;
        self.total_sales_revenue += revenue;

        self.inventory[idx].count -= 1;
        if self.inventory[idx].count == 0 {
            self.inventory.remove(idx);
        }

        self.add_log(format!(
            "【売却】『{}』を1個市場へ売却し、{} G を得ました。",
            item_name, revenue
        ));

        Some(revenue)
    }

    /// 全在庫を一括売却
    pub fn sell_all_items(&mut self) -> i64 {
        let mut total_earn = 0;
        for item in &self.inventory {
            total_earn += item.total_market_value();
        }

        if total_earn > 0 {
            self.gold += total_earn;
            self.total_sales_revenue += total_earn;
            self.inventory.clear();
            self.add_log(format!(
                "【一括売却】全在庫を市場へ売却し、合計 {} G を得ました！",
                total_earn
            ));
        }

        total_earn
    }

    /// ログを追加（最大50件保持）
    pub fn add_log(&mut self, message: impl Into<String>) {
        self.logs.push(message.into());
        if self.logs.len() > 50 {
            self.logs.remove(0);
        }
    }
}
