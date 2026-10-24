pub mod app;
pub mod core;
pub mod ui;

use app::GuildApp;
use ui::config::WindowConfig;
use ui::font;

/// Linux / DevContainer 環境向け: X11 バックエンドの優先と日本語 IME (fcitx5-mozc) 環境変数の自動補完
#[cfg(all(unix, not(target_os = "macos")))]
#[allow(unsafe_code)]
fn setup_linux_ime_env() {
    if std::env::var("DISPLAY").is_ok() {
        // SAFETY: main 関数の最序盤・シングルスレッド時点で環境変数を安全に設定
        unsafe {
            // WSLg 上で Wayland IME バインド制約（error 71）を回避するため X11 を優先
            std::env::remove_var("WAYLAND_DISPLAY");
            std::env::set_var("WINIT_UNIX_BACKEND", "x11");

            // winit が XIM (X Input Method) 経由で fcitx5 に接続するための必須環境変数
            if std::env::var("XMODIFIERS").is_err() {
                std::env::set_var("XMODIFIERS", "@im=fcitx");
            }
            if std::env::var("GTK_IM_MODULE").is_err() {
                std::env::set_var("GTK_IM_MODULE", "fcitx");
            }
            if std::env::var("QT_IM_MODULE").is_err() {
                std::env::set_var("QT_IM_MODULE", "fcitx");
            }
            if std::env::var("DBUS_SESSION_BUS_ADDRESS").is_err() {
                std::env::set_var(
                    "DBUS_SESSION_BUS_ADDRESS",
                    "unix:path=/tmp/dbus-session-vscode.sock",
                );
            }
        }

        // D-Bus ソケットが存在しない、または fcitx5 が未起動の場合は自動起動
        let dbus_socket = std::path::Path::new("/tmp/dbus-session-vscode.sock");
        if !dbus_socket.exists() {
            let _ = std::process::Command::new("bash")
                .arg(".devcontainer/init-ime.sh")
                .spawn();
        }
    }
}

#[cfg(any(not(unix), target_os = "macos"))]
const fn setup_linux_ime_env() {}

#[allow(unsafe_code)]
fn main() -> eframe::Result<()> {
    setup_linux_ime_env();

    // 保存済みユーザー設定を読み込み、環境変数の指定があれば優先適用
    let settings = ui::config::UserSettings::load_from_file_or_default(ui::config::SETTINGS_FILE_PATH);
    let config = WindowConfig::from_settings_and_env(&settings);
    let options = config.to_native_options();

    eframe::run_native(
        config.title,
        options,
        Box::new(|cc| {
            font::setup_fonts(&cc.egui_ctx);
            Ok(Box::new(GuildApp::new()))
        }),
    )
}

