use serde::{Deserialize, Serialize};
use super::adventurer::Adventurer;

/// ギルドの殿堂（歴代の功労者・追悼録）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HallOfFameAdventurer {
    pub adventurer_id: u64,
    pub name: String,
    pub final_rank: String,
    pub age: u32,
    pub completed_quests: u32,
    pub primary_role: String,
    pub reason: String, // "老衰永眠", "戦死 (○○クエスト)", "名誉引退"
    pub recorded_day: u32,
}

impl HallOfFameAdventurer {
    pub fn from_adventurer(adv: &Adventurer, reason: impl Into<String>, day: u32) -> Self {
        Self {
            adventurer_id: adv.id,
            name: adv.name.clone(),
            final_rank: adv.rank.clone(),
            age: adv.age,
            completed_quests: adv.completed_quests,
            primary_role: adv.primary_role(),
            reason: reason.into(),
            recorded_day: day,
        }
    }
}

/// 周回引継ぎ・全周回共有メタデータ
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MetaState {
    pub playthrough_count: u32,
    pub highest_guild_rank: String,
    pub hall_of_fame: Vec<HallOfFameAdventurer>,
    pub unlocked_blueprints: Vec<String>, // アンロックされた道場秘伝や施設
}

impl MetaState {
    pub fn new() -> Self {
        Self {
            playthrough_count: 1,
            highest_guild_rank: "F".to_string(),
            hall_of_fame: Vec::new(),
            unlocked_blueprints: Vec::new(),
        }
    }

    /// 殿堂への登録
    pub fn record_adventurer(&mut self, entry: HallOfFameAdventurer) {
        self.hall_of_fame.push(entry);
    }
}
