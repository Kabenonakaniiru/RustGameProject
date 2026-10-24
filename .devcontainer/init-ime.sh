#!/bin/bash
# DevContainer 用 日本語入力（fcitx5-mozc）自動初期化・起動スクリプト
set -e

# 1. fcitx5 設定ファイルの初期化（未配備の場合のみ作成）
mkdir -p "$HOME/.config/fcitx5"

if [ ! -f "$HOME/.config/fcitx5/profile" ]; then
    cat <<'EOF' > "$HOME/.config/fcitx5/profile"
[Groups/0]
# Group Name
Name=Default
# Layout
Default Layout=us
# Default Input Method
DefaultIM=mozc

[Groups/0/Items/0]
# Name
Name=keyboard-us
# Layout
Layout=

[Groups/0/Items/1]
# Name
Name=mozc
# Layout
Layout=

[GroupOrder]
0=Default
EOF
fi

if [ ! -f "$HOME/.config/fcitx5/config" ]; then
    cat <<'EOF' > "$HOME/.config/fcitx5/config"
[Hotkey]
EnumerateWithTriggerKeys=True
EnumerateForwardKeys=
EnumerateBackwardKeys=
EnumerateSkipFirst=False

[Hotkey/TriggerKeys]
0=Zenkaku_Hankaku
1=Control+space
EOF
fi

# 2. D-Bus セッションバスの確認と起動
DBUS_SOCKET="/tmp/dbus-session-vscode.sock"
export DBUS_SESSION_BUS_ADDRESS="unix:path=$DBUS_SOCKET"

if [ ! -e "$DBUS_SOCKET" ] || ! dbus-send --session --dest=org.freedesktop.DBus --type=method_call --print-reply /org/freedesktop/DBus org.freedesktop.DBus.GetId >/dev/null 2>&1; then
    rm -f "$DBUS_SOCKET"
    dbus-daemon --session --address="unix:path=$DBUS_SOCKET" --fork
    # D-Bus が応答するまで待機（最大2秒）
    for _ in $(seq 1 20); do
        if dbus-send --session --dest=org.freedesktop.DBus --type=method_call --print-reply /org/freedesktop/DBus org.freedesktop.DBus.GetId >/dev/null 2>&1; then
            break
        fi
        sleep 0.1
    done
fi

# 3. fcitx5 デーモンの確認と起動
# WSLg 環境（Weston）で Wayland 入力メソッド権限エラーを回避するため、Wayland アドオンを無効化して X11 経由で動作
if ! pgrep -u "$(id -u)" fcitx5 >/dev/null 2>&1; then
    export DISPLAY="${DISPLAY:-:0}"
    fcitx5 -d --disable wayland,waylandim >/dev/null 2>&1 || true
    # fcitx5 が起動するまで少し待機（最大2秒）
    for _ in $(seq 1 20); do
        if pgrep -u "$(id -u)" fcitx5 >/dev/null 2>&1; then
            break
        fi
        sleep 0.1
    done
fi
