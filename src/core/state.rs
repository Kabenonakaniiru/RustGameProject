use std::collections::HashMap;
use rand::SeedableRng;
use rand_pcg::Pcg64;
use serde::{Deserialize, Serialize};

use super::adventurer::{Adventurer, AdventurerStatus, DeathCause, Skill, SkillType, Stats};
use super::finance::Staff;
use super::inventory::InventoryItem;
use super::meta::HallOfFameAdventurer;
use super::party::Party;
use super::quest::{ClientFaction, Quest, QuestDifficulty};
use super::training::TrainingDojo;

pub const DAYS_PER_YEAR: u32 = 365;

fn default_rng() -> Pcg64 {
    Pcg64::seed_from_u64(42)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameState {
    // 基本ステータス
    pub gold: i64,
    pub day: u32,
    pub rank: String,
    pub reputation: u32, // 総合名声（Guild Fame）

    // 発注勢力ごとの信頼度（-100 〜 +100）
    pub faction_trust: HashMap<ClientFaction, i32>,

    // 支出設定（日額）
    pub daily_tax: i64,     // 固定資産税
    pub daily_utility: i64, // 光熱費

    // 各種エンティティ
    pub adventurers: Vec<Adventurer>,
    pub parties: Vec<Party>,
    pub inventory: Vec<InventoryItem>,
    pub quests: Vec<Quest>,
    pub staff_list: Vec<Staff>,
    pub training_dojo: TrainingDojo,
    pub hall_of_fame: Vec<HallOfFameAdventurer>,

    // 累積収支記録
    pub total_sales_revenue: i64,
    pub total_commission: i64,
    pub total_expenses: i64,

    // 決定論的乱数シード & RNG
    pub rng_seed: u64,
    #[serde(skip, default = "default_rng")]
    pub rng: Pcg64,

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
        let rng = Pcg64::seed_from_u64(42);

        // 初期冒険者（スキル制・能力値・年齢）
        let adventurers = vec![
            Adventurer::new(
                1,
                "アレン",
                "C",
                24,
                78,
                Stats::new(120, 22, 18, 8, 16, 14, 15),
                vec![
                    Skill::new(SkillType::Swordsmanship, 4),
                    Skill::new(SkillType::Tactics, 2),
                    Skill::new(SkillType::Cooperation, 2),
                ],
            ),
            Adventurer::new(
                2,
                "エレナ",
                "C",
                22,
                82,
                Stats::new(85, 8, 10, 28, 14, 16, 10),
                vec![
                    Skill::new(SkillType::AttackMagic, 4),
                    Skill::new(SkillType::HealingMagic, 3),
                    Skill::new(SkillType::SupportMagic, 2),
                ],
            ),
            Adventurer::new(
                3,
                "ボリス",
                "D",
                28,
                75,
                Stats::new(160, 26, 25, 6, 8, 10, 8),
                vec![
                    Skill::new(SkillType::Shieldwork, 3),
                    Skill::new(SkillType::MartialArts, 2),
                ],
            ),
            Adventurer::new(
                4,
                "リナ",
                "D",
                19,
                80,
                Stats::new(95, 12, 11, 14, 24, 22, 9),
                vec![
                    Skill::new(SkillType::Scouting, 3),
                    Skill::new(SkillType::DisarmTrap, 3),
                    Skill::new(SkillType::Archery, 2),
                ],
            ),
            Adventurer::new(
                5,
                "クルト",
                "E",
                17,
                85,
                Stats::new(90, 14, 12, 10, 16, 18, 7),
                vec![
                    Skill::new(SkillType::Herbalism, 2),
                    Skill::new(SkillType::Archery, 1),
                ],
            ),
        ];

        // 初期部隊（例: アレン隊）
        let mut first_party = Party::new(1, "第一遊撃小隊");
        let _ = first_party.add_member(1); // アレン
        let _ = first_party.add_member(2); // エレナ
        let _ = first_party.add_member(3); // ボリス
        let _ = first_party.add_member(4); // リナ

        let mut faction_trust = HashMap::new();
        faction_trust.insert(ClientFaction::Kingdom, 10);
        faction_trust.insert(ClientFaction::Commerce, 15);
        faction_trust.insert(ClientFaction::Civilian, 20);

        let inventory = vec![
            InventoryItem::new("ゴブリンの爪", "魔物部位", 12, 30, 65),
            InventoryItem::new("低級魔導石", "魔導素材", 5, 150, 320),
            InventoryItem::new("薬草（上質）", "薬草・植物", 20, 20, 50),
            InventoryItem::new("オオカミの毛皮", "毛皮・皮革", 4, 80, 180),
        ];

        let quests = vec![
            Quest::new(
                1,
                "ゴブリンの森の掃討",
                ClientFaction::Civilian,
                "南の森林",
                QuestDifficulty::D,
                SkillType::Swordsmanship,
                800,
                1,
            ),
            Quest::new(
                2,
                "コボルトの鉱山調査",
                ClientFaction::Commerce,
                "東部廃鉱山",
                QuestDifficulty::D,
                SkillType::Scouting,
                1200,
                2,
            ),
            Quest::new(
                3,
                "ワイバーン警戒任務",
                ClientFaction::Kingdom,
                "北の岩山街道",
                QuestDifficulty::C,
                SkillType::Archery,
                2500,
                3,
            ),
            Quest::new(
                4,
                "上級薬草の緊急採取依頼",
                ClientFaction::Civilian,
                "精霊の泉周辺",
                QuestDifficulty::E,
                SkillType::Herbalism,
                400,
                1,
            ),
            Quest::new(
                5,
                "古代遺跡地下の探索",
                ClientFaction::Kingdom,
                "旧王都地下遺跡",
                QuestDifficulty::B,
                SkillType::DisarmTrap,
                6000,
                5,
            ),
        ];

        let staff_list = vec![
            Staff::new("マルタ", "受付主任", 40),
            Staff::new("トマス", "倉庫番・鑑定士", 35),
            Staff::new("セリア", "事務・会計係", 30),
        ];

        let logs = vec![
            "【ギルド業務日誌】本日の業務を開始しました。".to_string(),
            "【通達】固定資産税・光熱費・職員給与は毎日自動引き落としされます。".to_string(),
            "【新体制】スキル制および最大10名編成の連携システムが始動しました。".to_string(),
        ];

        Self {
            gold: 3500,
            day: 1,
            rank: "ブロンズ (Rank 2)".to_string(),
            reputation: 45,
            faction_trust,
            daily_tax: 25,
            daily_utility: 15,
            adventurers,
            parties: vec![first_party],
            inventory,
            quests,
            staff_list,
            training_dojo: TrainingDojo::new(),
            hall_of_fame: Vec::new(),
            total_sales_revenue: 0,
            total_commission: 0,
            total_expenses: 0,
            rng_seed: 42,
            rng,
            logs,
        }
    }

    /// 現在の経過年（1年 = 365日）
    pub fn current_year(&self) -> u32 {
        ((self.day.saturating_sub(1)) / DAYS_PER_YEAR) + 1
    }

    /// 年内の経過日数（1〜365日）
    pub fn day_of_year(&self) -> u32 {
        ((self.day.saturating_sub(1)) % DAYS_PER_YEAR) + 1
    }

    /// 職員人件費の日額合計
    pub fn daily_staff_salary(&self) -> i64 {
        self.staff_list.iter().map(|s| s.salary).sum()
    }

    /// 1日あたりの固定支出合計（税＋光熱費＋職員給＋道場教官給）
    pub fn daily_total_expenses(&self) -> i64 {
        self.daily_tax
            + self.daily_utility
            + self.daily_staff_salary()
            + self.training_dojo.daily_salary_total()
    }

    /// 1日進める処理（経費、クエスト進行、訓練、加齢・老衰）
    pub fn advance_day(&mut self) {
        self.day += 1;

        // 1. 固定費用の支払い
        let expense = self.daily_total_expenses();
        self.gold -= expense;
        self.total_expenses += expense;

        self.add_log(format!(
            "【{}日目 ({}年目 {}日)】固定経費計 {}G を支払いました。",
            self.day,
            self.current_year(),
            self.day_of_year(),
            expense
        ));

        // 2. 負傷療養者の回復処理
        let mut recovered_names = Vec::new();
        for adv in &mut self.adventurers {
            if let AdventurerStatus::Injured { days_remaining } = adv.status {
                if days_remaining > 1 {
                    adv.status = AdventurerStatus::Injured {
                        days_remaining: days_remaining - 1,
                    };
                } else {
                    adv.status = AdventurerStatus::Standby;
                    recovered_names.push(adv.name.clone());
                }
            }
        }
        for name in recovered_names {
            self.add_log(format!(
                "【療養完了】{} の負傷が完治し、前線へ復帰しました！",
                name
            ));
        }

        // 3. 訓練所での日次修練処理
        let dojo_msgs = self.training_dojo.process_daily_training(&mut self.adventurers);
        for msg in dojo_msgs {
            self.add_log(msg);
        }

        // 4. クエストの進行と成否判定
        let mut completed_quest_indices = Vec::new();

        for (idx, quest) in self.quests.iter_mut().enumerate() {
            if quest.is_dispatched {
                if quest.days_remaining > 1 {
                    quest.days_remaining -= 1;
                } else {
                    completed_quest_indices.push(idx);
                }
            }
        }

        for idx in completed_quest_indices {
            self.resolve_completed_quest(idx);
        }

        // 5. 1年経過（新年に突入したタイミング）の加齢・老衰処理
        if (self.day - 1) % DAYS_PER_YEAR == 0 && self.day > 1 {
            self.process_yearly_aging();
        }
    }

    /// クエスト完了の判定・報酬精算・負傷戦死処理
    fn resolve_completed_quest(&mut self, quest_idx: usize) {
        let (party_id, quest_title, quest_faction) = {
            let quest = &mut self.quests[quest_idx];
            quest.is_dispatched = false;
            quest.days_remaining = 0;
            let p_id = quest.assigned_party_id.take();
            let _ = quest.assigned_party_name.take();
            (p_id, quest.title.clone(), quest.faction)
        };

        let party_idx = party_id.and_then(|id| self.parties.iter().position(|p| p.id == id));

        if let Some(p_idx) = party_idx {
            let resolution = {
                let party = &mut self.parties[p_idx];
                party.completed_quests += 1;
                let quest = &self.quests[quest_idx];
                quest.resolve(party, &self.adventurers, &mut self.rng)
            };

            // 手数料・名声・信頼度の反映
            self.gold += resolution.guild_commission;
            self.total_commission += resolution.guild_commission;
            self.reputation += resolution.fame_gain;

            let current_trust = self.faction_trust.entry(quest_faction).or_insert(0);
            *current_trust = (*current_trust + resolution.trust_change).clamp(-100, 100);

            // 戦利品の入庫
            for item in resolution.loot_items {
                self.inventory.push(item);
            }

            // ログ記録
            self.add_log(&resolution.log_summary);

            // 負傷者の反映
            let mut injured_logs = Vec::new();
            for (adv_id, days) in resolution.injured_member_ids {
                if let Some(adv) = self.adventurers.iter_mut().find(|a| a.id == adv_id) {
                    adv.status = AdventurerStatus::Injured { days_remaining: days };
                    injured_logs.push(format!(
                        "【療養入院】{} はクエストでの負傷により {} 日間の療養に入ります。",
                        adv.name, days
                    ));
                }
            }
            for log in injured_logs {
                self.add_log(log);
            }

            // 戦死者の反映
            let mut fallen_logs = Vec::new();
            for adv_id in resolution.fallen_member_ids {
                if let Some(pos) = self.adventurers.iter().position(|a| a.id == adv_id) {
                    let adv = &mut self.adventurers[pos];
                    adv.status = AdventurerStatus::Fallen {
                        cause: DeathCause::KilledInAction(quest_title.clone()),
                        day: self.day,
                    };
                    fallen_logs.push(format!(
                        "【哀悼】冒険者 {} は任務『{}』にて名誉の戦死を遂げました…",
                        adv.name, quest_title
                    ));

                    // 殿堂入り
                    let hall_entry = HallOfFameAdventurer::from_adventurer(
                        adv,
                        format!("戦死（{}）", quest_title),
                        self.day,
                    );
                    self.hall_of_fame.push(hall_entry);

                    // パーティから除外
                    self.parties[p_idx].remove_member(adv_id);
                }
            }
            for log in fallen_logs {
                self.add_log(log);
            }

            // 無事なメンバーは待機中に復帰
            for &m_id in &self.parties[p_idx].member_ids {
                if let Some(adv) = self.adventurers.iter_mut().find(|a| a.id == m_id) {
                    if matches!(adv.status, AdventurerStatus::OnQuest { .. }) {
                        adv.status = AdventurerStatus::Standby;
                        adv.completed_quests += 1;
                    }
                }
            }
        }
    }

    /// 年末の加齢・衰退・老衰処理
    fn process_yearly_aging(&mut self) {
        let current_yr = self.current_year();
        self.add_log(format!(
            "【年末】第 {} 年が経過しました。ギルド員全員が1歳加齢します。",
            current_yr
        ));

        let mut deceased_info = Vec::new();

        for adv in &mut self.adventurers {
            adv.age_one_year();

            // 寿命チェック
            if adv.age >= adv.natural_lifespan && !matches!(adv.status, AdventurerStatus::Fallen { .. }) {
                deceased_info.push((adv.id, adv.name.clone(), adv.age));
            }
        }

        for (id, name, age) in deceased_info {
            if let Some(adv) = self.adventurers.iter_mut().find(|a| a.id == id) {
                adv.status = AdventurerStatus::Fallen {
                    cause: DeathCause::NaturalDeath,
                    day: self.day,
                };
                let hall_entry = HallOfFameAdventurer::from_adventurer(adv, "老衰永眠", self.day);
                self.hall_of_fame.push(hall_entry);
            }
            self.add_log(format!(
                "【老衰永眠】長年ギルドに貢献した {} が天寿を全うし、永眠しました（享年{}）。",
                name, age
            ));

            // パーティや訓練所からの除外
            for party in &mut self.parties {
                party.remove_member(id);
            }
            self.training_dojo.remove_trainee(id);
        }
    }

    /// パーティをクエストに派遣する（新設計）
    pub fn dispatch_quest_party(&mut self, quest_idx: usize, party_id: u64) -> Result<(), &'static str> {
        if quest_idx >= self.quests.len() {
            return Err("クエストが存在しません");
        }

        let party = self.parties.iter().find(|p| p.id == party_id).ok_or("パーティが存在しません")?;
        if party.member_ids.is_empty() {
            return Err("パーティにメンバーがいません");
        }

        // 全メンバーが待機可能か確認
        for &m_id in &party.member_ids {
            let adv = self.adventurers.iter().find(|a| a.id == m_id).ok_or("冒険者が見つかりません")?;
            if !adv.status.is_available() {
                return Err("パーティ内に派遣不可能な冒険者が含まれています");
            }
        }

        // 冒険者を派遣中状態に
        for &m_id in &party.member_ids {
            if let Some(adv) = self.adventurers.iter_mut().find(|a| a.id == m_id) {
                adv.status = AdventurerStatus::OnQuest { quest_id: quest_idx };
            }
        }

        let party_name = party.name.clone();
        let (quest_title, days_required) = {
            let quest = &mut self.quests[quest_idx];
            quest.dispatch(party);
            (quest.title.clone(), quest.days_required)
        };

        self.add_log(format!(
            "【部隊派遣】部隊『{}』が『{}』へ出撃しました（所要: {}日）。",
            party_name, quest_title, days_required
        ));

        Ok(())
    }

    /// 旧UI・テスト互換ヘルパー: 単独冒険者を派遣する
    pub fn dispatch_quest(&mut self, quest_idx: usize, adventurer_name: &str) -> bool {
        let adv_id = match self.adventurers.iter().find(|a| a.name == adventurer_name && a.status.is_available()) {
            Some(a) => a.id,
            None => return false,
        };

        // この冒険者が所属するパーティを探すか、単独臨時パーティを即席編成
        let party_id = if let Some(existing) = self.parties.iter().find(|p| p.member_ids.contains(&adv_id)) {
            existing.id
        } else {
            let new_id = (self.parties.len() as u64) + 100;
            let mut p = Party::new(new_id, format!("{}隊", adventurer_name));
            let _ = p.add_member(adv_id);
            self.parties.push(p);
            new_id
        };

        self.dispatch_quest_party(quest_idx, party_id).is_ok()
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
