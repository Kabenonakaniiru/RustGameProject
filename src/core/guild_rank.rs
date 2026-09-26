use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use super::quest::ClientFaction;

/// ギルドランク（F → E → D → C → B → A → S の7段階）
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GuildRank {
    F,
    E,
    D,
    C,
    B,
    A,
    S,
}

impl GuildRank {
    /// 表示名
    pub fn display_name(&self) -> &'static str {
        match self {
            GuildRank::F => "F（見習い）",
            GuildRank::E => "E（銅級）",
            GuildRank::D => "D（鉄級）",
            GuildRank::C => "C（銀級）",
            GuildRank::B => "B（金級）",
            GuildRank::A => "A（白金級）",
            GuildRank::S => "S（伝説級）",
        }
    }

    /// ランク記号（短縮表示）
    pub fn symbol(&self) -> &'static str {
        match self {
            GuildRank::F => "F",
            GuildRank::E => "E",
            GuildRank::D => "D",
            GuildRank::C => "C",
            GuildRank::B => "B",
            GuildRank::A => "A",
            GuildRank::S => "S",
        }
    }

    /// 次のランク（Sランクの場合はNone）
    pub fn next_rank(&self) -> Option<GuildRank> {
        match self {
            GuildRank::F => Some(GuildRank::E),
            GuildRank::E => Some(GuildRank::D),
            GuildRank::D => Some(GuildRank::C),
            GuildRank::C => Some(GuildRank::B),
            GuildRank::B => Some(GuildRank::A),
            GuildRank::A => Some(GuildRank::S),
            GuildRank::S => None,
        }
    }

    /// 月次監査の上納金額
    pub fn monthly_tribute(&self) -> i64 {
        match self {
            GuildRank::F => 100,
            GuildRank::E => 200,
            GuildRank::D => 400,
            GuildRank::C => 800,
            GuildRank::B => 1500,
            GuildRank::A => 3000,
            GuildRank::S => 5000,
        }
    }

    /// 月次監査の最低クエスト完了要件
    pub fn monthly_quest_requirement(&self) -> u32 {
        match self {
            GuildRank::F => 1,
            GuildRank::E => 2,
            GuildRank::D => 3,
            GuildRank::C => 4,
            GuildRank::B => 5,
            GuildRank::A => 6,
            GuildRank::S => 8,
        }
    }
}

impl std::fmt::Display for GuildRank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl Default for GuildRank {
    fn default() -> Self {
        GuildRank::F
    }
}

/// 昇格に必要な条件
#[derive(Debug, Clone)]
pub struct PromotionRequirement {
    /// 対象ランク（このランクへの昇格条件）
    pub target_rank: GuildRank,
    /// 必要な総合名声値
    pub required_reputation: u32,
    /// 必要な最低勢力信頼度（指定勢力それぞれの最低ライン）
    pub min_faction_trust: Vec<(FactionTrustRequirement, i32)>,
    /// 必要なクエスト完了総数（ギルド全体の累計）
    pub required_total_quests: u32,
    /// 必要最低経過日数
    pub required_min_days: u32,
}

/// 勢力信頼度の要求条件タイプ
#[derive(Debug, Clone)]
pub enum FactionTrustRequirement {
    /// いずれか1つの勢力が条件を満たす
    Any,
    /// すべての勢力が条件を満たす
    All,
    /// 特定の勢力が条件を満たす
    Specific(ClientFaction),
}

impl PromotionRequirement {
    /// 各ランクの昇格条件を取得
    pub fn for_rank(target: GuildRank) -> Option<Self> {
        match target {
            GuildRank::F => None, // Fランクは初期ランク、昇格先はない
            GuildRank::E => Some(Self {
                target_rank: GuildRank::E,
                required_reputation: 50,
                min_faction_trust: vec![(FactionTrustRequirement::Any, 15)],
                required_total_quests: 5,
                required_min_days: 15,
            }),
            GuildRank::D => Some(Self {
                target_rank: GuildRank::D,
                required_reputation: 120,
                min_faction_trust: vec![(FactionTrustRequirement::Any, 25)],
                required_total_quests: 15,
                required_min_days: 45,
            }),
            GuildRank::C => Some(Self {
                target_rank: GuildRank::C,
                required_reputation: 250,
                min_faction_trust: vec![(FactionTrustRequirement::Any, 40)],
                required_total_quests: 30,
                required_min_days: 90,
            }),
            GuildRank::B => Some(Self {
                target_rank: GuildRank::B,
                required_reputation: 500,
                min_faction_trust: vec![
                    // いずれか2勢力が50以上 → Any(50) を2回チェックする代わりに、
                    // ここでは簡易的に「いずれか1勢力が50以上」とする
                    (FactionTrustRequirement::Any, 50),
                ],
                required_total_quests: 60,
                required_min_days: 180,
            }),
            GuildRank::A => Some(Self {
                target_rank: GuildRank::A,
                required_reputation: 1000,
                min_faction_trust: vec![(FactionTrustRequirement::All, 60)],
                required_total_quests: 120,
                required_min_days: 365,
            }),
            GuildRank::S => Some(Self {
                target_rank: GuildRank::S,
                required_reputation: 2000,
                min_faction_trust: vec![(FactionTrustRequirement::All, 80)],
                required_total_quests: 250,
                required_min_days: 730,
            }),
        }
    }

    /// 条件を満たしているかチェック
    pub fn is_met(
        &self,
        reputation: u32,
        faction_trust: &HashMap<ClientFaction, i32>,
        total_quests: u32,
        current_day: u32,
    ) -> bool {
        // 名声チェック
        if reputation < self.required_reputation {
            return false;
        }

        // クエスト完了数チェック
        if total_quests < self.required_total_quests {
            return false;
        }

        // 経過日数チェック
        if current_day < self.required_min_days {
            return false;
        }

        // 勢力信頼度チェック
        for (req_type, min_trust) in &self.min_faction_trust {
            let met = match req_type {
                FactionTrustRequirement::Any => {
                    faction_trust.values().any(|&t| t >= *min_trust)
                }
                FactionTrustRequirement::All => {
                    let all_factions = [
                        ClientFaction::Kingdom,
                        ClientFaction::Commerce,
                        ClientFaction::Civilian,
                    ];
                    all_factions.iter().all(|f| {
                        faction_trust.get(f).copied().unwrap_or(0) >= *min_trust
                    })
                }
                FactionTrustRequirement::Specific(faction) => {
                    faction_trust.get(faction).copied().unwrap_or(0) >= *min_trust
                }
            };
            if !met {
                return false;
            }
        }

        true
    }

    /// 達成度をパーセンテージで表示するための進捗情報
    pub fn progress_summary(
        &self,
        reputation: u32,
        _faction_trust: &HashMap<ClientFaction, i32>,
        total_quests: u32,
        current_day: u32,
    ) -> Vec<(String, u32, u32)> {
        let mut items = Vec::new();

        items.push((
            "総合名声".to_string(),
            reputation.min(self.required_reputation),
            self.required_reputation,
        ));
        items.push((
            "クエスト完了数".to_string(),
            total_quests.min(self.required_total_quests),
            self.required_total_quests,
        ));
        items.push((
            "経過日数".to_string(),
            current_day.min(self.required_min_days),
            self.required_min_days,
        ));

        items
    }
}
