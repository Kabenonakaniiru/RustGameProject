use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Quest {
    pub title: String,
    pub location: String,
    pub difficulty: String,
    pub reward_estimate: i64,
    pub days_required: u32,
    pub days_remaining: u32,
    pub is_dispatched: bool,
    pub assigned_party: Option<String>,
}

impl Quest {
    pub fn new(
        title: impl Into<String>,
        location: impl Into<String>,
        difficulty: impl Into<String>,
        reward_estimate: i64,
        days_required: u32,
    ) -> Self {
        Self {
            title: title.into(),
            location: location.into(),
            difficulty: difficulty.into(),
            reward_estimate,
            days_required,
            days_remaining: 0,
            is_dispatched: false,
            assigned_party: None,
        }
    }

    /// クエストを冒険者に派遣する
    pub fn dispatch(&mut self, party_name: impl Into<String>) {
        self.is_dispatched = true;
        self.days_remaining = self.days_required;
        self.assigned_party = Some(party_name.into());
    }

    /// ギルド仲介手数料（15%）の計算
    pub fn calculate_commission(&self) -> i64 {
        (self.reward_estimate as f64 * 0.15) as i64
    }
}
