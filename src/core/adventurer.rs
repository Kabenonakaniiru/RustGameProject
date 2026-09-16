use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Adventurer {
    pub name: String,
    pub rank: String,
    pub class_name: String,
    pub level: u32,
    pub status: String,
}

impl Adventurer {
    pub fn new(name: impl Into<String>, rank: impl Into<String>, class_name: impl Into<String>, level: u32) -> Self {
        Self {
            name: name.into(),
            rank: rank.into(),
            class_name: class_name.into(),
            level,
            status: "待機中".to_string(),
        }
    }
}
