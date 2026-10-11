pub mod components;
pub mod config;
pub mod font;
pub mod views;

pub use config::{APP_TITLE, UserSettings, WindowConfig, WindowSizePreset};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    GuildOverview,     // ギルド情報
    QuestsAndDispatch, // クエスト・ダンジョン派遣
    Adventurers,       // 冒険者管理
    MarketAndTrading,  // 倉庫・売買（素材・部位の売却）
    StaffAndFacility,  // 職員・施設管理（人件費・税金・光熱費）
    FinancialReport,   // 収支台帳
}

impl Tab {
    /// 全タブの定義リスト
    pub const ALL: [Self; 6] = [
        Self::GuildOverview,
        Self::QuestsAndDispatch,
        Self::Adventurers,
        Self::MarketAndTrading,
        Self::StaffAndFacility,
        Self::FinancialReport,
    ];

    /// タブの表示用ラベル（アイコン付き）
    pub const fn label(&self) -> &'static str {
        match self {
            Self::GuildOverview => "🏛 ギルド基本情報",
            Self::QuestsAndDispatch => "📋 受付・クエスト派遣",
            Self::Adventurers => "⚔ 冒険者管理",
            Self::MarketAndTrading => "📦 倉庫・戦利品売買",
            Self::StaffAndFacility => "👥 職員雇用・施設維持",
            Self::FinancialReport => "📊 収支台帳",
        }
    }
}

