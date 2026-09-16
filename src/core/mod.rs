pub mod adventurer;
pub mod finance;
pub mod inventory;
pub mod quest;
pub mod state;

pub use adventurer::Adventurer;
pub use finance::Staff;
pub use inventory::InventoryItem;
pub use quest::Quest;
pub use state::GameState;

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
    fn test_sell_single_item() {
        let mut state = GameState::new_default();
        let initial_gold = state.gold;
        let target_item = state.inventory[0].clone();
        let initial_count = target_item.count;

        let revenue = state.sell_item(0).expect("Item should exist");

        assert_eq!(revenue, target_item.market_price);
        assert_eq!(state.gold, initial_gold + revenue);
        assert_eq!(state.inventory[0].count, initial_count - 1);
    }

    #[test]
    fn test_sell_all_items() {
        let mut state = GameState::new_default();
        let initial_gold = state.gold;
        let expected_revenue: i64 = state.inventory.iter().map(|i| i.total_market_value()).sum();

        let total_earn = state.sell_all_items();

        assert_eq!(total_earn, expected_revenue);
        assert_eq!(state.gold, initial_gold + expected_revenue);
        assert!(state.inventory.is_empty());
    }

    #[test]
    fn test_quest_dispatch_and_completion() {
        let mut state = GameState::new_default();
        let initial_gold = state.gold;
        let initial_inv_count = state.inventory.len();

        // 1日クエスト（index 0: ゴブリンの森の掃討, days_required: 1）をアレンに派遣
        let dispatched = state.dispatch_quest(0, "アレン");
        assert!(dispatched);
        assert_eq!(state.adventurers[0].status, "派遣中");
        assert!(state.quests[0].is_dispatched);
        assert_eq!(state.quests[0].days_remaining, 1);

        // 1日経過 -> クエスト完了
        state.advance_day();

        assert!(!state.quests[0].is_dispatched);
        assert_eq!(state.adventurers[0].status, "待機中");
        assert!(state.gold > initial_gold - state.daily_total_expenses()); // 手数料で補填されている
        assert_eq!(state.inventory.len(), initial_inv_count + 1); // 戦利品追加
    }

    #[test]
    fn test_save_and_load_roundtrip() {
        let mut state = GameState::new_default();
        state.day = 10;
        state.gold = 99999;
        state.add_log("テストログメッセージ");

        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("test_save_data.json");

        save_game(&state, &temp_file).expect("Save should succeed");
        let loaded_state = load_game(&temp_file).expect("Load should succeed");

        assert_eq!(state.day, loaded_state.day);
        assert_eq!(state.gold, loaded_state.gold);
        assert_eq!(state.rank, loaded_state.rank);
        assert_eq!(state.logs, loaded_state.logs);

        let _ = std::fs::remove_file(temp_file);
    }
}
