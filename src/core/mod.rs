pub mod adventurer;
pub mod finance;
pub mod inventory;
pub mod meta;
pub mod party;
pub mod quest;
pub mod state;
pub mod training;

pub use adventurer::{Adventurer, AdventurerStatus, DeathCause, Skill, SkillType, Stats};
pub use finance::Staff;
pub use inventory::InventoryItem;
pub use meta::{HallOfFameAdventurer, MetaState};
pub use party::{Party, MAX_PARTY_MEMBERS};
pub use quest::{ClientFaction, Quest, QuestDifficulty, QuestOutcomeType, QuestResolution};
pub use state::GameState;
pub use training::{Instructor, TrainingDojo};

use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

/// ゲーム状態を指定パスに JSON として保存
pub fn save_game(state: &GameState, path: impl AsRef<Path>) -> Result<(), std::io::Error> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, state)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    Ok(())
}

/// 指定パスの JSON からゲーム状態を読み込み
pub fn load_game(path: impl AsRef<Path>) -> Result<GameState, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let state = serde_json::from_reader(reader)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    Ok(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advance_day_expenses() {
        let mut state = GameState::new_default();
        let initial_gold = state.gold;
        let expenses = state.daily_total_expenses();

        state.advance_day();

        assert_eq!(state.day, 2);
        assert_eq!(state.gold, initial_gold - expenses);
        assert_eq!(state.total_expenses, expenses);
    }

    #[test]
    fn test_sell_single_and_all_items() {
        let mut state = GameState::new_default();
        let initial_gold = state.gold;
        let target_item = state.inventory[0].clone();
        let initial_count = target_item.count;

        let revenue = state.sell_item(0).expect("Item should exist");
        assert_eq!(revenue, target_item.market_price);
        assert_eq!(state.gold, initial_gold + revenue);
        assert_eq!(state.inventory[0].count, initial_count - 1);

        let total_earn = state.sell_all_items();
        assert!(total_earn > 0);
        assert!(state.inventory.is_empty());
    }

    #[test]
    fn test_party_max_10_members_and_synergy() {
        let state = GameState::new_default();
        let mut large_party = Party::new(10, "大遠征隊");

        // 冒険者を11人用意して追加を試みる
        for i in 1..=10 {
            assert!(large_party.add_member(i).is_ok());
        }
        assert_eq!(large_party.member_ids.len(), MAX_PARTY_MEMBERS);

        // 11人目は上限オーバーでエラー
        assert!(large_party.add_member(11).is_err());

        // 統率スキル持ち（アレン: id 1, leadership 15, Tactics Lv2）がいる場合といない場合のシナジー比較
        let synergy_with_leader = large_party.calculate_effective_synergy(&state.adventurers);

        // アレンを除外した場合（統率不足で烏合の衆ペナルティ）
        large_party.remove_member(1);
        let synergy_without_leader = large_party.calculate_effective_synergy(&state.adventurers);

        // 指揮官がいる方が実効連携値が高い
        assert!(synergy_with_leader > synergy_without_leader);
    }

    #[test]
    fn test_quest_dispatch_and_party_resolution() {
        let mut state = GameState::new_default();

        // クエスト1（所要1日）にパーティ1（第一遊撃小隊）を派遣
        let res = state.dispatch_quest_party(0, 1);
        assert!(res.is_ok());

        assert!(state.quests[0].is_dispatched);
        assert_eq!(state.quests[0].days_remaining, 1);
        assert_eq!(state.adventurers[0].status, AdventurerStatus::OnQuest { quest_id: 0 });

        // 1日経過 -> クエスト完了・報酬精算
        state.advance_day();

        assert!(!state.quests[0].is_dispatched);
        // クエスト完了後、メンバーは待機中に復帰
        assert_eq!(state.adventurers[0].status, AdventurerStatus::Standby);
        // 報酬手数料により所持金がプラス（経費引き落とし後でも十分補填）
        assert!(state.total_commission > 0);
        assert!(state.reputation > 45); // 名声獲得
    }

    #[test]
    fn test_training_dojo_instructor_mentorship() {
        let mut state = GameState::new_default();

        // アレン（剣術Lv4）を引退させ、教官として道場に配置
        let mut retired_allen = state.adventurers[0].clone();
        retired_allen.status = AdventurerStatus::Retired;
        let instructor = Instructor::from_retired_adventurer(&retired_allen, 50);
        assert_eq!(instructor.specialty_skill, SkillType::Swordsmanship);
        state.training_dojo.add_instructor(instructor);

        // 新人クルト（id 5）を訓練生として道場に登録
        assert!(state.training_dojo.enroll_trainee(5).is_ok());

        let kuruto_initial_sword_lvl = state.adventurers[4].skill_level(SkillType::Swordsmanship);
        assert_eq!(kuruto_initial_sword_lvl, 0); // 初期は剣術なし

        // 1日進める（道場での日次修練が実行される）
        state.advance_day();

        // クルトが教官から剣術の指導を受け、スキルを新規習得
        assert!(state.adventurers[4].skill_level(SkillType::Swordsmanship) >= 1);
    }

    #[test]
    fn test_aging_and_lifespan_death() {
        let mut state = GameState::new_default();
        let initial_str = state.adventurers[0].stats.strength;

        // 冒険者の年齢を44歳に引き上げ
        state.adventurers[0].age = 44;

        // 364日進める（まだ年を越さない）
        for _ in 0..364 {
            state.day += 1;
        }
        // 365日目に advance_day()
        state.advance_day();

        // 45歳に加齢し、身体能力の減衰が始まる
        assert_eq!(state.adventurers[0].age, 45);
        assert!(state.adventurers[0].stats.strength <= initial_str);

        // 寿命に到達した冒険者の老衰永眠テスト
        state.adventurers[1].age = 81;
        state.adventurers[1].natural_lifespan = 82;

        // 次の年の境界まで進める
        for _ in 0..364 {
            state.day += 1;
        }
        state.advance_day();

        // 82歳に達して老衰永眠となり、殿堂（Hall of Fame）に登録される
        assert!(matches!(state.adventurers[1].status, AdventurerStatus::Fallen { cause: DeathCause::NaturalDeath, .. }));
        assert!(!state.hall_of_fame.is_empty());
        assert_eq!(state.hall_of_fame[0].name, "エレナ");
    }

    #[test]
    fn test_save_and_load_roundtrip() {
        let mut state = GameState::new_default();
        state.day = 120;
        state.gold = 77777;
        state.reputation = 150;
        state.add_log("セーブデータ検証ログ");

        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("test_save_game_data.json");

        save_game(&state, &temp_file).expect("Save should succeed");
        let loaded = load_game(&temp_file).expect("Load should succeed");

        assert_eq!(state.day, loaded.day);
        assert_eq!(state.gold, loaded.gold);
        assert_eq!(state.rank, loaded.rank);
        assert_eq!(state.reputation, loaded.reputation);
        assert_eq!(state.adventurers.len(), loaded.adventurers.len());
        assert_eq!(state.parties.len(), loaded.parties.len());
        assert_eq!(state.faction_trust, loaded.faction_trust);
        assert_eq!(state.logs, loaded.logs);

        let _ = std::fs::remove_file(temp_file);
    }
}
