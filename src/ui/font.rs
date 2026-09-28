use eframe::egui;

/// 使用するフォントの識別名
pub const DEFAULT_FONT_NAME: &str = "japanese_font";

/// 静的埋め込みフォントデータ (IPAゴシック)
const FONT_DATA_BYTES: &[u8] = include_bytes!("../../assets/ipag.ttf");

/// フォントマネージャ・設定ローダー
pub struct FontManager;

impl FontManager {
    /// egui コンテキストに日本語フォントを登録・設定する
    pub fn setup_fonts(ctx: &egui::Context) {
        let mut fonts = egui::FontDefinitions::default();

        // フォントデータの登録
        fonts.font_data.insert(
            DEFAULT_FONT_NAME.to_owned(),
            egui::FontData::from_static(FONT_DATA_BYTES).into(),
        );

        // プロポーショナルフォントファミリー（UI通常テキスト）の最優先に設定
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, DEFAULT_FONT_NAME.to_owned());

        // 等幅フォントファミリー（数値やコード用）のフォールバックとして追加
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push(DEFAULT_FONT_NAME.to_owned());

        ctx.set_fonts(fonts);
    }
}

/// 簡易関数エイリアス
pub fn setup_fonts(ctx: &egui::Context) {
    FontManager::setup_fonts(ctx);
}
