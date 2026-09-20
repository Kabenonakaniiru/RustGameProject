use serde::{Deserialize, Serialize};
use rand::Rng;
use super::adventurer::{Adventurer, SkillType};
use super::inventory::InventoryItem;
use super::party::Party;

/// クライアント（発注者）勢力
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ClientFaction {
    Kingdom,  // 王国・公的機関（治安維持・大型討伐、名声獲得大）
    Commerce, // 商業ギルド・商家（採取護衛・調達、報酬金大）
    Civilian, // 民間・地域住民（害獣駆除・採取、若手育成・基礎信頼）
}

impl ClientFaction {
    pub fn name(&self) -> &'static str {
        match self {
            ClientFaction::Kingdom => "王宮・騎士団",
            ClientFaction::Commerce => "商業連盟・商家",
            ClientFaction::Civilian => "地域・民間自治会",
        }
    }
}

/// クエスト難易度
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum QuestDifficulty {
    F,
    E,
    D,
    C,
    B,
    A,
    S,
}

impl QuestDifficulty {
    pub fn name(&self) -> &'static str {
        match self {
            QuestDifficulty::F => "F",
            QuestDifficulty::E => "E",
            QuestDifficulty::D => "D",
            QuestDifficulty::C => "C",
            QuestDifficulty::B => "B",
            QuestDifficulty::A => "A",
            QuestDifficulty::S => "S",
        }
    }

    /// 要求総合戦闘力・能力スコアの目安
    pub fn base_target_score(&self) -> u32 {
        match self {
            QuestDifficulty::F => 25,
            QuestDifficulty::E => 50,
            QuestDifficulty::D => 100,
            QuestDifficulty::C => 180,
            QuestDifficulty::B => 300,
            QuestDifficulty::A => 500,
            QuestDifficulty::S => 800,
        }
    }
}

impl std::fmt::Display for QuestDifficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// クエスト結果の詳細
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuestResolution {
    pub outcome: QuestOutcomeType,
    pub guild_commission: i64,
    pub loot_items: Vec<InventoryItem>,
    pub fame_gain: u32,
    pub trust_change: i32,
    pub injured_member_ids: Vec<(u64, u32)>,  // (冒険者ID, 療養所要日数)
    pub fallen_member_ids: Vec<u64>,           // 戦死した冒険者ID
    pub log_summary: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum QuestOutcomeType {
    GreatSuccess,
    Success,
    Failure,
    Catastrophe,
}

/// クエスト定義
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Quest {
    pub id: u64,
    pub title: String,
    pub faction: ClientFaction,
    pub location: String,
    pub difficulty: QuestDifficulty,
    pub recommended_skill: SkillType, // 推奨スキル（適合すると大幅有利）
    pub reward_estimate: i64,
    pub days_required: u32,
    pub days_remaining: u32,
    pub is_dispatched: bool,
    pub assigned_party_id: Option<u64>,
    pub assigned_party_name: Option<String>,
}

impl Quest {
    pub fn new(
        id: u64,
        title: impl Into<String>,
        faction: ClientFaction,
        location: impl Into<String>,
        difficulty: QuestDifficulty,
        recommended_skill: SkillType,
        reward_estimate: i64,
        days_required: u32,
    ) -> Self {
        Self {
            id,
            title: title.into(),
            faction,
            location: location.into(),
            difficulty,
            recommended_skill,
            reward_estimate,
            days_required,
            days_remaining: 0,
            is_dispatched: false,
            assigned_party_id: None,
            assigned_party_name: None,
        }
    }

    /// クエストをパーティに派遣する
    pub fn dispatch(&mut self, party: &Party) {
        self.is_dispatched = true;
        self.days_remaining = self.days_required;
        self.assigned_party_id = Some(party.id);
        self.assigned_party_name = Some(party.name.clone());
    }

    /// ギルド仲介手数料基本額（15%）
    pub fn calculate_base_commission(&self) -> i64 {
        (self.reward_estimate as f64 * 0.15) as i64
    }

    /// クエスト成否を判定・解決する
    pub fn resolve<R: Rng>(
        &self,
        party: &mut Party,
        adventurers: &[Adventurer],
        rng: &mut R,
    ) -> QuestResolution {
        let members: Vec<&Adventurer> = party
            .member_ids
            .iter()
            .filter_map(|id| adventurers.iter().find(|a| a.id == *id))
            .collect();

        if members.is_empty() {
            return QuestResolution {
                outcome: QuestOutcomeType::Failure,
                guild_commission: 0,
                loot_items: Vec::new(),
                fame_gain: 0,
                trust_change: -2,
                injured_member_ids: Vec::new(),
                fallen_member_ids: Vec::new(),
                log_summary: format!("『{}』は参加メンバー不在で失敗となりました。", self.title),
            };
        }

        // 1. パーティの基礎戦闘力合計
        let base_power: u32 = members.iter().map(|m| m.total_combat_power()).sum();

        // 2. 推奨スキル適合ボーナス
        let skill_bonus: u32 = members
            .iter()
            .map(|m| m.skill_level(self.recommended_skill) * 20)
            .sum();

        // 3. 実効連携値（最大10人編成の統率状況を加味した倍率）
        let synergy = party.calculate_effective_synergy(adventurers);

        // 総実効スコア
        let party_score = ((base_power + skill_bonus) as f32 * synergy) as u32;

        // 乱数によるブレ（80%〜120%）
        let roll_percent = rng.gen_range(80..=120);
        let final_party_roll = (party_score * roll_percent) / 100;

        let target_score = self.difficulty.base_target_score();

        // 防御・治癒支援の充実度（戦死・負傷防止能力）
        let healing_power: u32 = members.iter().map(|m| m.skill_level(SkillType::HealingMagic)).sum();
        let shield_power: u32 = members.iter().map(|m| m.skill_level(SkillType::Shieldwork)).sum();

        if final_party_roll >= target_score * 3 / 2 {
            // 大成功（Targetの1.5倍以上）
            party.gain_cooperation(6.0);
            let commission = (self.calculate_base_commission() as f64 * 1.5) as i64;
            let fame = match self.faction {
                ClientFaction::Kingdom => 15,
                ClientFaction::Commerce => 8,
                ClientFaction::Civilian => 6,
            };
            let loot = vec![
                InventoryItem::new(format!("極上の{}", self.default_loot_name()), "上質素材", 2, 80, 200),
                InventoryItem::new("未知の宝箱の残骸", "お宝", 1, 300, 700),
            ];

            QuestResolution {
                outcome: QuestOutcomeType::GreatSuccess,
                guild_commission: commission,
                loot_items: loot,
                fame_gain: fame,
                trust_change: 4,
                injured_member_ids: Vec::new(),
                fallen_member_ids: Vec::new(),
                log_summary: format!(
                    "【大成功】部隊『{}』が『{}』で圧倒的な武功を挙げました！（手数料: {}G, 名声+{}）",
                    party.name, self.title, commission, fame
                ),
            }
        } else if final_party_roll >= target_score {
            // 通常成功
            party.gain_cooperation(3.0);
            let commission = self.calculate_base_commission();
            let fame = match self.faction {
                ClientFaction::Kingdom => 8,
                ClientFaction::Commerce => 4,
                ClientFaction::Civilian => 3,
            };
            let loot = vec![
                InventoryItem::new(self.default_loot_name(), "採取素材", 2, 40, 100),
            ];

            QuestResolution {
                outcome: QuestOutcomeType::Success,
                guild_commission: commission,
                loot_items: loot,
                fame_gain: fame,
                trust_change: 2,
                injured_member_ids: Vec::new(),
                fallen_member_ids: Vec::new(),
                log_summary: format!(
                    "【成功】部隊『{}』が『{}』を無事完遂！（手数料: {}G, 名声+{}）",
                    party.name, self.title, commission, fame
                ),
            }
        } else if final_party_roll >= target_score / 2 {
            // 失敗（軽傷・療養）
            party.gain_cooperation(1.0); // 失敗でもわずかに経験
            let mut injured = Vec::new();
            // ランダムで1〜2名が負傷（療養 2〜5日）
            let injured_count = rng.gen_range(1..=2).min(members.len());
            for member in members.iter().take(injured_count) {
                // 治癒・盾術があれば療養日数が短縮
                let reduction = (healing_power + shield_power) / 2;
                let days = (rng.gen_range(2..=5) as i32 - reduction as i32).max(1) as u32;
                injured.push((member.id, days));
            }

            QuestResolution {
                outcome: QuestOutcomeType::Failure,
                guild_commission: 0,
                loot_items: Vec::new(),
                fame_gain: 0,
                trust_change: -1,
                injured_member_ids: injured,
                fallen_member_ids: Vec::new(),
                log_summary: format!(
                    "【失敗】部隊『{}』は『{}』で苦戦し撤退しました（怪我人療養）。",
                    party.name, self.title
                ),
            }
        } else {
            // 壊滅的大失敗（戦死の危機）
            party.gain_cooperation(-4.0); // 混乱により連携値低下
            let mut injured = Vec::new();
            let mut fallen = Vec::new();

            // 治癒・盾術の救命チェック（十分な高Lvなら戦死を救命できる）
            let safety_check = healing_power * 15 + shield_power * 10;
            let death_risk = rng.gen_range(1..=100);

            if (death_risk > safety_check as i32) && (self.difficulty >= QuestDifficulty::C) {
                // 難易度C以上かつ救命失敗時、1名が戦死
                let victim = members[rng.gen_range(0..members.len())];
                fallen.push(victim.id);
            }

            // 残りのメンバーは重傷（療養 5〜10日）
            for member in &members {
                if !fallen.contains(&member.id) {
                    let days = rng.gen_range(4..=8);
                    injured.push((member.id, days));
                }
            }

            let log_msg = if !fallen.is_empty() {
                format!(
                    "【壊滅的被害】部隊『{}』が『{}』で部隊壊滅！ 尊い犠牲者が発生しました…",
                    party.name, self.title
                )
            } else {
                format!(
                    "【重傷退却】部隊『{}』は『{}』で危機に陥りましたが、決死の救護により全員が生還しました。",
                    party.name, self.title
                )
            };

            QuestResolution {
                outcome: QuestOutcomeType::Catastrophe,
                guild_commission: 0,
                loot_items: Vec::new(),
                fame_gain: 0,
                trust_change: -4,
                injured_member_ids: injured,
                fallen_member_ids: fallen,
                log_summary: log_msg,
            }
        }
    }

    fn default_loot_name(&self) -> &'static str {
        match self.faction {
            ClientFaction::Kingdom => "魔物の討伐証・牙",
            ClientFaction::Commerce => "高純度鉱石サンプル",
            ClientFaction::Civilian => "希少薬草の群生地採取品",
        }
    }
}
