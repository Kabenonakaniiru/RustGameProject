use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Staff {
    pub name: String,
    pub role: String,
    pub salary: i64, // 日給
}

impl Staff {
    pub fn new(name: impl Into<String>, role: impl Into<String>, salary: i64) -> Self {
        Self {
            name: name.into(),
            role: role.into(),
            salary,
        }
    }
}
