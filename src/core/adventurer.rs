use serde::{Deserialize, Serialize};

/// 冒険者の基礎能力値
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Stats {
    pub hp: u32,
    pub max_hp: u32,
    pub strength: u32,     // 物理攻撃力・運搬力
    pub defense: u32,      // 物理耐久力
    pub intelligence: u32, // 魔法力・知識・鑑定
    pub agility: u32,      // 行動力・回避・逃走
    pub dexterity: u32,    // 命中率・罠解除・採取
    pub leadership: u32,   // 統率力（パーティ連携値向上・混乱防止）
}

impl Stats {
    pub fn new(
        max_hp: u32,
        strength: u32,
        defense: u32,
        intelligence: u32,
        agility: u32,
        dexterity: u32,
        leadership: u32,
    ) -> Self {
        Self {
            hp: max_hp,
            max_hp,
            strength,
            defense,
            intelligence,
            agility,
            dexterity,
            leadership,
        }
    }

    /// 加齢による身体能力の減衰（知力・統率は減りにくい）
    pub fn apply_aging_decay(&mut self) {
        if self.max_hp > 10 {
            self.max_hp = (self.max_hp.saturating_sub(1)).max(10);
            self.hp = self.hp.min(self.max_hp);
        }
        if self.strength > 3 {
            self.strength = self.strength.saturating_sub(1);
        }
        if self.agility > 3 {
            self.agility = self.agility.saturating_sub(1);
        }
        if self.defense > 3 {
            self.defense = self.defense.saturating_sub(1);
        }
    }
}

/// スキルの種類（職業制を廃止し、スキル制を採用）
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SkillType {
    // 物理戦闘系
    Swordsmanship, // 剣術
    Shieldwork,    // 盾術（味方保護・被ダメ軽減）
    Archery,       // 弓術（遠隔・先制攻撃）
    MartialArts,   // 体術

    // 魔法・神秘系
    AttackMagic,   // 攻撃魔法（高火力殲滅）
    HealingMagic,  // 治癒魔法（負傷治療・戦死回避）
    SupportMagic,  // 支援魔法（バフ・防御障壁）

    // 探索・技術系
    Scouting,      // 斥候（索敵・危険察知）
    DisarmTrap,    // 罠解除・解錠
    Herbalism,     // 薬草採取（採取収量UP）
    Mining,        // 鉱石採掘

    // 指導・統率系
    Tactics,       // 戦術指揮（大部隊の統率）
    Cooperation,   // 連携補佐（連携値上昇ボーナス）
    Mentorship,    // 指導術（教官時の成長率ボーナス）
}

impl SkillType {
    pub fn name(&self) -> &'static str {
        match self {
            SkillType::Swordsmanship => "剣術",
            SkillType::Shieldwork => "盾術",
            SkillType::Archery => "弓術",
            SkillType::MartialArts => "体術",
            SkillType::AttackMagic => "攻撃魔法",
            SkillType::HealingMagic => "治癒魔法",
            SkillType::SupportMagic => "支援魔法",
            SkillType::Scouting => "斥候術",
            SkillType::DisarmTrap => "罠解除",
            SkillType::Herbalism => "薬草採取",
            SkillType::Mining => "鉱石採掘",
            SkillType::Tactics => "戦術指揮",
            SkillType::Cooperation => "連携協調",
            SkillType::Mentorship => "指導術",
        }
    }
}

/// 習得しているスキル
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Skill {
    pub skill_type: SkillType,
    pub level: u32,
    pub exp: u32,
}

impl Skill {
    pub fn new(skill_type: SkillType, level: u32) -> Self {
        Self {
            skill_type,
            level,
            exp: 0,
        }
    }

    /// 経験値加算とレベルアップ判定（Lv10上限）
    pub fn add_exp(&mut self, amount: u32) -> bool {
        if self.level >= 10 {
            return false;
        }
        self.exp += amount;
        let required = self.level * 100;
        if self.exp >= required {
            self.exp -= required;
            self.level += 1;
            true
        } else {
            false
        }
    }
}

/// 死因
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeathCause {
    KilledInAction(String), // クエスト名等の戦死
    NaturalDeath,           // 老衰
}

/// 冒険者の状態
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AdventurerStatus {
    Standby,                          // 待機中
    OnQuest { quest_id: usize },      // クエスト派遣中
    Training,                         // 訓練所で指導・特訓中
    Injured { days_remaining: u32 },  // 負傷療養中
    Retired,                          // 現役引退（教官雇用可能）
    Fallen { cause: DeathCause, day: u32 }, // 死亡・永眠
}

impl AdventurerStatus {
    pub fn is_available(&self) -> bool {
        matches!(self, AdventurerStatus::Standby)
    }

    pub fn display_name(&self) -> String {
        match self {
            AdventurerStatus::Standby => "待機中".to_string(),
            AdventurerStatus::OnQuest { .. } => "派遣中".to_string(),
            AdventurerStatus::Training => "訓練中".to_string(),
            AdventurerStatus::Injured { days_remaining } => format!("療養中（残{}日）", days_remaining),
            AdventurerStatus::Retired => "引退".to_string(),
            AdventurerStatus::Fallen { cause, day } => match cause {
                DeathCause::KilledInAction(q) => format!("殉職 ({}日目: {})", day, q),
                DeathCause::NaturalDeath => format!("永眠 ({}日目: 老衰)", day),
            },
        }
    }
}

/// 冒険者エンティティ
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Adventurer {
    pub id: u64,
    pub name: String,
    pub rank: String,             // F, E, D, C, B, A, S
    pub age: u32,                 // 年齢
    pub natural_lifespan: u32,    // 寿命（70〜85歳など）
    pub stats: Stats,             // 基礎能力値
    pub skills: Vec<Skill>,       // 習得スキル一覧
    pub status: AdventurerStatus, // 現在の状態
    pub completed_quests: u32,    // クエスト完了数
}

impl Adventurer {
    pub fn new(
        id: u64,
        name: impl Into<String>,
        rank: impl Into<String>,
        age: u32,
        natural_lifespan: u32,
        stats: Stats,
        skills: Vec<Skill>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            rank: rank.into(),
            age,
            natural_lifespan,
            stats,
            skills,
            status: AdventurerStatus::Standby,
            completed_quests: 0,
        }
    }

    /// 指定スキルのレベルを取得（未習得なら0）
    pub fn skill_level(&self, skill_type: SkillType) -> u32 {
        self.skills
            .iter()
            .find(|s| s.skill_type == skill_type)
            .map(|s| s.level)
            .unwrap_or(0)
    }

    /// スキル経験値を獲得（なければLv1として新規習得）
    pub fn gain_skill_exp(&mut self, skill_type: SkillType, amount: u32) -> bool {
        if let Some(skill) = self.skills.iter_mut().find(|s| s.skill_type == skill_type) {
            skill.add_exp(amount)
        } else {
            self.skills.push(Skill::new(skill_type, 1));
            false
        }
    }

    /// 最高レベルのスキル等から通称・代表ロールを動的に生成
    pub fn primary_role(&self) -> String {
        if self.skills.is_empty() {
            return "新人冒険者".to_string();
        }
        let best_skill = self.skills.iter().max_by_key(|s| s.level).unwrap();
        match best_skill.skill_type {
            SkillType::Swordsmanship => format!("剣士 (Lv{})", best_skill.level),
            SkillType::Shieldwork => format!("重装衛士 (Lv{})", best_skill.level),
            SkillType::Archery => format!("射手 (Lv{})", best_skill.level),
            SkillType::MartialArts => format!("武闘家 (Lv{})", best_skill.level),
            SkillType::AttackMagic => format!("魔導士 (Lv{})", best_skill.level),
            SkillType::HealingMagic => format!("治癒術士 (Lv{})", best_skill.level),
            SkillType::SupportMagic => format!("付与魔導士 (Lv{})", best_skill.level),
            SkillType::Scouting => format!("斥候 (Lv{})", best_skill.level),
            SkillType::DisarmTrap => format!("盗賊・工作員 (Lv{})", best_skill.level),
            SkillType::Herbalism => format!("薬草採取師 (Lv{})", best_skill.level),
            SkillType::Mining => format!("探鉱士 (Lv{})", best_skill.level),
            SkillType::Tactics => format!("指揮官 (Lv{})", best_skill.level),
            SkillType::Cooperation => format!("遊撃支援士 (Lv{})", best_skill.level),
            SkillType::Mentorship => format!("指導員 (Lv{})", best_skill.level),
        }
    }

    /// UI表示用の総合戦闘力・実力目安
    pub fn total_combat_power(&self) -> u32 {
        let stats_sum = self.stats.hp / 2
            + self.stats.strength
            + self.stats.defense
            + self.stats.intelligence
            + self.stats.agility;
        let skill_bonus: u32 = self.skills.iter().map(|s| s.level * 10).sum();
        stats_sum + skill_bonus
    }

    /// 1年経過による加齢処理（老衰判定は呼び出し側で行う）
    pub fn age_one_year(&mut self) {
        self.age += 1;
        // 45歳を超えると身体能力が徐々に衰退
        if self.age >= 45 {
            self.stats.apply_aging_decay();
        }
    }
}
