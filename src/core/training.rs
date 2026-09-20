use serde::{Deserialize, Serialize};
use super::adventurer::{Adventurer, SkillType};

/// 訓練所の教官（引退した冒険者または専任指導員）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Instructor {
    pub adventurer_id: Option<u64>, // 引退冒険者ならそのID
    pub name: String,
    pub specialty_skill: SkillType, // 指導得意スキル
    pub skill_bonus_level: u32,     // 指導品質・スキルLv
    pub mentorship_level: u32,      // 指導術Lv（全体効率向上）
    pub daily_salary: i64,          // 教官日給
}

impl Instructor {
    pub fn from_retired_adventurer(adv: &Adventurer, daily_salary: i64) -> Self {
        let best_skill = adv
            .skills
            .iter()
            .max_by_key(|s| s.level)
            .map(|s| (s.skill_type, s.level))
            .unwrap_or((SkillType::Swordsmanship, 1));

        let mentorship = adv.skill_level(SkillType::Mentorship);

        Self {
            adventurer_id: Some(adv.id),
            name: format!("{}教官", adv.name),
            specialty_skill: best_skill.0,
            skill_bonus_level: best_skill.1,
            mentorship_level: mentorship,
            daily_salary,
        }
    }
}

/// ギルド付属訓練所（Training Dojo）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrainingDojo {
    pub instructors: Vec<Instructor>,
    pub trainee_ids: Vec<u64>,
    pub max_trainees: usize,
}

impl Default for TrainingDojo {
    fn default() -> Self {
        Self {
            instructors: Vec::new(),
            trainee_ids: Vec::new(),
            max_trainees: 6,
        }
    }
}

impl TrainingDojo {
    pub fn new() -> Self {
        Self::default()
    }

    /// 教官の日給合計
    pub fn daily_salary_total(&self) -> i64 {
        self.instructors.iter().map(|i| i.daily_salary).sum()
    }

    /// 訓練生を追加
    pub fn enroll_trainee(&mut self, adventurer_id: u64) -> Result<(), &'static str> {
        if self.trainee_ids.len() >= self.max_trainees {
            return Err("訓練所の受け入れ枠がいっぱいです");
        }
        if self.trainee_ids.contains(&adventurer_id) {
            return Err("既に訓練所に登録されています");
        }
        self.trainee_ids.push(adventurer_id);
        Ok(())
    }

    /// 訓練生を卒業・除外
    pub fn remove_trainee(&mut self, adventurer_id: u64) -> bool {
        if let Some(pos) = self.trainee_ids.iter().position(|&id| id == adventurer_id) {
            self.trainee_ids.remove(pos);
            true
        } else {
            false
        }
    }

    /// 教官を雇用・配置
    pub fn add_instructor(&mut self, instructor: Instructor) {
        self.instructors.push(instructor);
    }

    /// 1日の訓練処理
    /// 訓練生に経験値を付与し、レベルアップや成長メッセージを返す
    pub fn process_daily_training(&self, adventurers: &mut [Adventurer]) -> Vec<String> {
        let mut messages = Vec::new();

        for &trainee_id in &self.trainee_ids {
            if let Some(adv) = adventurers.iter_mut().find(|a| a.id == trainee_id) {
                // 基礎訓練による微小なステータス・経験値付与
                let base_exp = 30;

                // 各教官からの指導ボーナス
                for inst in &self.instructors {
                    let mentor_multiplier = 1.0 + (inst.mentorship_level as f32 * 0.15);
                    let gained_exp = ((base_exp + inst.skill_bonus_level * 10) as f32 * mentor_multiplier) as u32;

                    let leveled_up = adv.gain_skill_exp(inst.specialty_skill, gained_exp);
                    if leveled_up {
                        let new_lvl = adv.skill_level(inst.specialty_skill);
                        messages.push(format!(
                            "【道場修練】{} が {}教官 の指導により『{}』が Lv{} に上昇！",
                            adv.name,
                            inst.name,
                            inst.specialty_skill.name(),
                            new_lvl
                        ));
                    }
                }

                // 連携協調スキルの自然学習
                adv.gain_skill_exp(SkillType::Cooperation, 15);
            }
        }

        messages
    }
}
