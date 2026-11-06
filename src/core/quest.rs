use serde::{Deserialize, Serialize};
use rand::Rng;
use super::adventurer::{Adventurer, SkillType};
use super::guild_rank::GuildRank;
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
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Kingdom => "王宮・騎士団",
            Self::Commerce => "商業連盟・商家",
            Self::Civilian => "地域・民間自治会",
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
    pub const fn name(&self) -> &'static str {
        match self {
            Self::F => "F",
            Self::E => "E",
            Self::D => "D",
            Self::C => "C",
            Self::B => "B",
            Self::A => "A",
            Self::S => "S",
        }
    }

    /// 要求総合戦闘力・能力スコアの目安
    pub const fn base_target_score(&self) -> u32 {
        match self {
            Self::F => 25,
            Self::E => 50,
            Self::D => 100,
            Self::C => 180,
            Self::B => 300,
            Self::A => 500,
            Self::S => 800,
        }
    }
}

impl std::fmt::Display for QuestDifficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// クエスト結果の詳細
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

impl QuestOutcomeType {
    pub const fn name(&self) -> &'static str {
        match self {
            Self::GreatSuccess => "大成功",
            Self::Success => "成功",
            Self::Failure => "失敗",
            Self::Catastrophe => "壊滅（大失敗）",
        }
    }
}

/// クエスト完了結果レポート（UI表示・プレイヤー通知用）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QuestReport {
    pub quest_title: String,
    pub party_name: String,
    pub outcome: QuestOutcomeType,
    pub commission: i64,
    pub fame_gain: u32,
    pub trust_change: i32,
    pub loot_items: Vec<String>,
    pub injured_names: Vec<(String, u32)>,
    pub fallen_names: Vec<String>,
    pub log_summary: String,
    pub completed_day: u32,
}

/// クエスト定義
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
    #[allow(clippy::too_many_arguments)]
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
                let reduction = u32::midpoint(healing_power, shield_power);
                let days = (rng.gen_range(2..=5) - reduction as i32).max(1) as u32;
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

            let log_msg = if fallen.is_empty() {
                format!(
                    "【重傷退却】部隊『{}』は『{}』で危機に陥りましたが、決死の救護により全員が生還しました。",
                    party.name, self.title
                )
            } else {
                format!(
                    "【壊滅的被害】部隊『{}』が『{}』で部隊壊滅！ 尊い犠牲者が発生しました…",
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

    const fn default_loot_name(&self) -> &'static str {
        match self.faction {
            ClientFaction::Kingdom => "魔物の討伐証・牙",
            ClientFaction::Commerce => "高純度鉱石サンプル",
            ClientFaction::Civilian => "希少薬草の群生地採取品",
        }
    }
}

// ──────────────────────────────────────────────────────────────
// クエスト自動生成（ギルドランク・勢力に応じた動的補充）
// ──────────────────────────────────────────────────────────────

/// ランクと勢力に応じてクエストを1件ランダム生成する
pub fn generate_quest<R: Rng>(id: u64, guild_rank: GuildRank, rng: &mut R) -> Quest {
    let faction = match rng.gen_range(0..3u8) {
        0 => ClientFaction::Kingdom,
        1 => ClientFaction::Commerce,
        _ => ClientFaction::Civilian,
    };
    let difficulty = pick_difficulty(guild_rank, rng);
    let (title, location, skill, reward, days) = quest_content(faction, difficulty, rng);
    Quest::new(id, title, faction, location, difficulty, skill, reward, days)
}

fn pick_difficulty<R: Rng>(rank: GuildRank, rng: &mut R) -> QuestDifficulty {
    let weights: &[(QuestDifficulty, u32)] = match rank {
        GuildRank::F => &[(QuestDifficulty::F, 40), (QuestDifficulty::E, 50), (QuestDifficulty::D, 10)],
        GuildRank::E => &[(QuestDifficulty::E, 40), (QuestDifficulty::D, 45), (QuestDifficulty::C, 15)],
        GuildRank::D => &[(QuestDifficulty::D, 35), (QuestDifficulty::C, 45), (QuestDifficulty::B, 20)],
        GuildRank::C => &[(QuestDifficulty::C, 30), (QuestDifficulty::B, 45), (QuestDifficulty::A, 25)],
        GuildRank::B => &[(QuestDifficulty::B, 30), (QuestDifficulty::A, 45), (QuestDifficulty::S, 25)],
        GuildRank::A | GuildRank::S => &[(QuestDifficulty::A, 30), (QuestDifficulty::S, 70)],
    };
    let total: u32 = weights.iter().map(|(_, w)| w).sum();
    let mut roll = rng.gen_range(0..total);
    for (diff, w) in weights {
        if roll < *w {
            return *diff;
        }
        roll -= w;
    }
    weights.last().unwrap().0
}

fn quest_content<R: Rng>(
    faction: ClientFaction,
    difficulty: QuestDifficulty,
    rng: &mut R,
) -> (String, String, SkillType, i64, u32) {
    let (title, location, skill) = match faction {
        ClientFaction::Kingdom  => KINGDOM_QUESTS[rng.gen_range(0..KINGDOM_QUESTS.len())],
        ClientFaction::Commerce => COMMERCE_QUESTS[rng.gen_range(0..COMMERCE_QUESTS.len())],
        ClientFaction::Civilian => CIVILIAN_QUESTS[rng.gen_range(0..CIVILIAN_QUESTS.len())],
    };
    let (base_reward, base_days) = match difficulty {
        QuestDifficulty::F => (200i64,  1u32),
        QuestDifficulty::E => (500,     1),
        QuestDifficulty::D => (1200,    2),
        QuestDifficulty::C => (2800,    3),
        QuestDifficulty::B => (6000,    5),
        QuestDifficulty::A => (12000,   7),
        QuestDifficulty::S => (25000,   10),
    };
    let pct = rng.gen_range(80..=120) as i64;
    let mut reward = base_reward * pct / 100;
    // 勢力による報酬補正
    reward = match faction {
        ClientFaction::Commerce => (reward as f64 * 1.3) as i64,
        ClientFaction::Civilian => (reward as f64 * 0.7) as i64,
        ClientFaction::Kingdom  => reward,
    };
    (title.to_string(), location.to_string(), skill, reward, base_days)
}

static KINGDOM_QUESTS: &[(&str, &str, SkillType)] = &[
    ("ゴブリン集落の殲滅討伐",    "南の深森",       SkillType::Swordsmanship),
    ("オーク野営地への強襲作戦",  "北部山岳地帯",   SkillType::Tactics),
    ("ワイバーン巣への警戒任務",  "王都北街道",     SkillType::Archery),
    ("魔物の王城周辺調査",        "旧王城廃墟",     SkillType::Scouting),
    ("街道の盗賊団一掃",          "南部街道",       SkillType::Swordsmanship),
    ("魔導兵器の回収・無力化",    "古代遺跡地下",   SkillType::DisarmTrap),
    ("騎士団との合同討伐演習",    "演習場・北郊外", SkillType::Tactics),
    ("国境砦の守備支援",          "東部国境砦",     SkillType::Shieldwork),
    ("魔将軍の部下の討伐",        "霧の沼地",       SkillType::AttackMagic),
    ("封印が解けた魔物の再封印",  "封印の祠",       SkillType::SupportMagic),
];

static COMMERCE_QUESTS: &[(&str, &str, SkillType)] = &[
    ("鉱山の魔物駆除と採掘再開",  "東部廃鉱山",     SkillType::Mining),
    ("商隊護衛（王都〜港湾）",    "王都街道",       SkillType::Shieldwork),
    ("希少鉱石の採掘・輸送",      "銀嶺鉱山",       SkillType::Mining),
    ("工房向け魔獣素材の採取",    "魔の森",         SkillType::Scouting),
    ("港湾の海賊討伐・荷物奪還",  "南部港湾都市",   SkillType::Swordsmanship),
    ("秘薬の原料採取（緊急）",    "精霊の泉",       SkillType::Herbalism),
    ("廃坑のガス噴出源の調査",    "旧炭坑",         SkillType::Scouting),
    ("上質毛皮の大量採取依頼",    "北の狩場",       SkillType::Archery),
    ("魔導石の大規模採掘調査",    "魔素の渓谷",     SkillType::Mining),
    ("競合商会の妨害排除",        "交易路",         SkillType::Tactics),
];

static CIVILIAN_QUESTS: &[(&str, &str, SkillType)] = &[
    ("村を脅かす魔猪の駆除",        "農村・南の丘",   SkillType::Swordsmanship),
    ("薬草の緊急採取（村の疫病）",  "精霊の泉周辺",   SkillType::Herbalism),
    ("子供たちの迷子捜索",          "東の迷宮洞窟",   SkillType::Scouting),
    ("大型スライムの農地侵食対策",  "麦畑・東農村",   SkillType::MartialArts),
    ("古い井戸の怪物退治",          "西の集落",       SkillType::DisarmTrap),
    ("学者の遺跡調査の護衛",        "旧王都地下遺跡", SkillType::Scouting),
    ("牧場の家畜を狙う魔獣退治",    "北牧場",         SkillType::Archery),
    ("花祭りへの道の安全確保",      "祭礼街道",       SkillType::Swordsmanship),
    ("廃屋に住み着いたゾンビ退治",  "廃村",           SkillType::HealingMagic),
    ("市場の偽造品調査",            "王都市場",       SkillType::Scouting),
];
