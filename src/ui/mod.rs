pub mod components;
pub mod views;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    GuildOverview,     // ギルド情報
    QuestsAndDispatch, // クエスト・ダンジョン派遣
    Adventurers,       // 冒険者管理
    MarketAndTrading,  // 倉庫・売買（素材・部位の売却）
    StaffAndFacility,  // 職員・施設管理（人件費・税金・光熱費）
    FinancialReport,   // 収支台帳
}
