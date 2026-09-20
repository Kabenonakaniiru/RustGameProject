use serde::{Deserialize, Serialize};
use super::adventurer::{Adventurer, SkillType};

pub const MAX_PARTY_MEMBERS: usize = 10;

/// パーティ（最大10人編成・連携値管理）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Party {
    pub id: u64,
    pub name: String,
    pub member_ids: Vec<u64>,
    pub cooperation: f32, // 部隊の基礎連携値（0.0 〜 100.0）
    pub completed_quests: u32,
}

impl Party {
    pub fn new(id: u64, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            member_ids: Vec::new(),
            cooperation: 10.0, // 初期連携値
            completed_quests: 0,
        }
    }

    /// メンバー追加（最大10人制限）
    pub fn add_member(&mut self, adventurer_id: u64) -> Result<(), &'static str> {
        if self.member_ids.len() >= MAX_PARTY_MEMBERS {
            return Err("パーティの最大人数（10名）に達しています");
        }
        if self.member_ids.contains(&adventurer_id) {
            return Err("既にパーティに含まれています");
        }
        self.member_ids.push(adventurer_id);
        Ok(())
    }

    /// メンバー除外
    pub fn remove_member(&mut self, adventurer_id: u64) -> bool {
        if let Some(pos) = self.member_ids.iter().position(|&id| id == adventurer_id) {
            self.member_ids.remove(pos);
            true
        } else {
            false
        }
    }

    /// クエスト成功等による連携値の上昇
    pub fn gain_cooperation(&mut self, amount: f32) {
        self.cooperation = (self.cooperation + amount).clamp(0.0, 100.0);
    }

    /// 実効連携値（Effective Synergy）の計算
    /// 
    /// - 人数が多い（5〜10人）場合、高い統率力（LeadershipやTacticsスキル）がないと
    ///   指揮統制が乱れてペナルティが発生する。
    /// - 適切な指揮官と高い部隊連携値があれば、大部隊による圧倒的戦力補正を得られる。
    pub fn calculate_effective_synergy(&self, all_adventurers: &[Adventurer]) -> f32 {
        let members: Vec<&Adventurer> = self
            .member_ids
            .iter()
            .filter_map(|id| all_adventurers.iter().find(|a| a.id == *id))
            .collect();

        let count = members.len();
        if count == 0 {
            return 0.0;
        }

        // パーティ内の最高統率力と戦術指揮スキル
        let max_leadership = members.iter().map(|m| m.stats.leadership).max().unwrap_or(0);
        let max_tactics = members
            .iter()
            .map(|m| m.skill_level(SkillType::Tactics))
            .max()
            .unwrap_or(0);

        // 連携協調スキルの総和
        let sum_cooperation_skills: u32 = members
            .iter()
            .map(|m| m.skill_level(SkillType::Cooperation))
            .sum();

        // 基本の連携値スコア（0.0〜1.0基準）
        let base_ratio = self.cooperation / 100.0;

        // 人数による要求統率値
        // 1〜3人: 要求0
        // 4〜6人: 要求10
        // 7〜10人: 要求25
        let required_leadership = match count {
            1..=3 => 0,
            4..=6 => 10,
            _ => 25,
        };

        // 指揮官の統率力評価（戦術スキルも加味）
        let command_power = max_leadership + max_tactics * 8;

        let leadership_modifier = if command_power >= required_leadership {
            // 指揮が十分に行き届いている場合: ボーナス
            1.0 + ((command_power - required_leadership) as f32 * 0.02)
        } else {
            // 指揮不足による大部隊の混乱・統率不全ペナルティ
            let shortage = required_leadership - command_power;
            (1.0 - (shortage as f32 * 0.05)).max(0.3)
        };

        // 協調スキルによる底上げ
        let skill_bonus = (sum_cooperation_skills as f32) * 0.05;

        // 最終実効連携値（0.1 〜 2.0 程度の倍率）
        (0.5 + base_ratio * 0.5 + skill_bonus) * leadership_modifier
    }
}
