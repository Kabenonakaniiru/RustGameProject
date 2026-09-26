use serde::{Deserialize, Serialize};
use super::guild_rank::GuildRank;

/// 月次監査の判定期間（日数）
pub const AUDIT_PERIOD_DAYS: u32 = 30;
/// ゲームオーバーまでの連続未達上限
pub const MAX_CONSECUTIVE_FAILURES: u32 = 3;

/// 月次監査の判定結果
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuditResult {
    /// 合格（上納金支払い・実績達成）
    Pass,
    /// 警告（条件ギリギリ、または部分的に未達）
    Warning,
    /// 未達（上納金不足または実績不足）
    Fail,
}

impl AuditResult {
    pub fn display_name(&self) -> &'static str {
        match self {
            AuditResult::Pass => "合格",
            AuditResult::Warning => "警告",
            AuditResult::Fail => "未達",
        }
    }
}

/// 月次監査の詳細レポート
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuditReport {
    pub result: AuditResult,
    pub audit_day: u32,
    pub tribute_required: i64,
    pub tribute_paid: i64,
    pub quests_required: u32,
    pub quests_completed: u32,
    pub consecutive_failures: u32,
    pub message: String,
}

/// 月次監査の状態管理
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MonthlyAudit {
    /// 前回監査日
    pub last_audit_day: u32,
    /// 今期のクエスト完了数（期間中にリセット）
    pub quests_completed_this_period: u32,
    /// 連続未達回数
    pub consecutive_failures: u32,
    /// 監査履歴（直近5件）
    pub history: Vec<AuditReport>,
}

impl Default for MonthlyAudit {
    fn default() -> Self {
        Self::new()
    }
}

impl MonthlyAudit {
    pub fn new() -> Self {
        Self {
            last_audit_day: 0,
            quests_completed_this_period: 0,
            consecutive_failures: 0,
            history: Vec::new(),
        }
    }

    /// クエスト完了を記録（期間中の累計）
    pub fn record_quest_completion(&mut self) {
        self.quests_completed_this_period += 1;
    }

    /// 監査が必要かチェック（30日周期）
    pub fn is_audit_due(&self, current_day: u32) -> bool {
        // 初回は30日目、以降30日ごと
        current_day > 0 && current_day >= self.last_audit_day + AUDIT_PERIOD_DAYS
    }

    /// 月次監査を実行し、結果を返す
    ///
    /// - `current_day`: 現在の経過日数
    /// - `guild_rank`: 現在のギルドランク
    /// - `current_gold`: 現在の所持金
    ///
    /// 返却: (AuditReport, 実際に徴収された上納金額)
    pub fn execute_audit(
        &mut self,
        current_day: u32,
        guild_rank: GuildRank,
        current_gold: i64,
    ) -> AuditReport {
        let tribute_required = guild_rank.monthly_tribute();
        let quests_required = guild_rank.monthly_quest_requirement();
        let quests_done = self.quests_completed_this_period;

        // 上納金チェック
        let can_pay_tribute = current_gold >= tribute_required;

        // クエスト実績チェック
        let quests_met = quests_done >= quests_required;

        // 判定
        let (result, tribute_paid, message) = if can_pay_tribute && quests_met {
            // 完全合格
            self.consecutive_failures = 0;
            (
                AuditResult::Pass,
                tribute_required,
                format!(
                    "【月次監査・合格】上納金 {}G を冒険者連盟に納付。クエスト実績 {}/{} 件達成。引き続き良好な運営を期待します。",
                    tribute_required, quests_done, quests_required
                ),
            )
        } else if can_pay_tribute && !quests_met && quests_done > 0 {
            // 上納金は払えるがクエスト実績不足（部分達成 → 警告）
            self.consecutive_failures += 1;
            (
                AuditResult::Warning,
                tribute_required,
                format!(
                    "【月次監査・警告】上納金 {}G は納付済みですが、クエスト実績が不足しています（{}/{} 件）。連続未達: {}回",
                    tribute_required, quests_done, quests_required, self.consecutive_failures
                ),
            )
        } else {
            // 上納金不足または実績ゼロ → 未達
            self.consecutive_failures += 1;
            let actual_paid = if can_pay_tribute { tribute_required } else { current_gold.max(0) };
            (
                AuditResult::Fail,
                actual_paid,
                format!(
                    "【月次監査・未達】上納金 {}G 中 {}G のみ納付。クエスト実績 {}/{} 件。連続未達: {}回目！",
                    tribute_required, actual_paid, quests_done, quests_required, self.consecutive_failures
                ),
            )
        };

        let report = AuditReport {
            result,
            audit_day: current_day,
            tribute_required,
            tribute_paid,
            quests_required,
            quests_completed: quests_done,
            consecutive_failures: self.consecutive_failures,
            message,
        };

        // 履歴に追加（最大5件保持）
        self.history.push(report.clone());
        if self.history.len() > 5 {
            self.history.remove(0);
        }

        // 期間リセット
        self.last_audit_day = current_day;
        self.quests_completed_this_period = 0;

        report
    }

    /// ゲームオーバー判定（連続未達が上限に達したか）
    pub fn is_game_over(&self) -> bool {
        self.consecutive_failures >= MAX_CONSECUTIVE_FAILURES
    }

    /// 次回監査までの残り日数
    pub fn days_until_next_audit(&self, current_day: u32) -> u32 {
        let next_audit_day = self.last_audit_day + AUDIT_PERIOD_DAYS;
        if current_day >= next_audit_day {
            0
        } else {
            next_audit_day - current_day
        }
    }
}
