use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InventoryItem {
    pub name: String,
    pub category: String,
    pub count: u32,
    pub buy_price: i64,   // 冒険者から買い取った単価
    pub market_price: i64, // 市場への売却想定単価
}

impl InventoryItem {
    pub fn new(
        name: impl Into<String>,
        category: impl Into<String>,
        count: u32,
        buy_price: i64,
        market_price: i64,
    ) -> Self {
        Self {
            name: name.into(),
            category: category.into(),
            count,
            buy_price,
            market_price,
        }
    }

    /// 1個あたりの見込利益
    pub fn unit_profit(&self) -> i64 {
        self.market_price - self.buy_price
    }

    /// 在庫全体の市場売却額
    pub fn total_market_value(&self) -> i64 {
        self.market_price * (self.count as i64)
    }
}
