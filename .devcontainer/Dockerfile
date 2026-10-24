FROM mcr.microsoft.com/devcontainers/rust:1

ENV TZ=Asia/Tokyo

# WSLg/GUI表示およびeframe/winitに必要なシステムライブラリ + 日本語IME(fcitx5-mozc)環境
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
        pkg-config \
        libx11-dev \
        libxcursor-dev \
        libxrandr-dev \
        libxi-dev \
        libxinerama-dev \
        libxfixes-dev \
        libxkbcommon-x11-dev \
        libasound2-dev \
        libudev-dev \
        libwayland-dev \
        libxkbcommon-dev \
        libegl1-mesa-dev \
        libgl1 \
        libglx-mesa0 \
        mesa-vulkan-drivers \
        fonts-ipafont \
        fonts-ipafont-gothic \
        fonts-noto-cjk \
        fcitx5 \
        fcitx5-mozc \
        dbus-x11 \
        im-config \
        locales \
    && sed -i -e 's/# ja_JP.UTF-8 UTF-8/ja_JP.UTF-8 UTF-8/' /etc/locale.gen \
    && locale-gen \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# 日本語IME（fcitx5-mozc）の既定プロファイル・ホットキー設定および環境変数の配備
RUN mkdir -p /etc/skel/.config/fcitx5 \
    && printf "[Groups/0]\nName=Default\nDefault Layout=jp\nDefaultIM=mozc\n\n[Groups/0/Items/0]\nName=keyboard-jp\nLayout=\n\n[Groups/0/Items/1]\nName=keyboard-us\nLayout=\n\n[Groups/0/Items/2]\nName=mozc\nLayout=\n\n[GroupOrder]\n0=Default\n" > /etc/skel/.config/fcitx5/profile \
    && printf "[Hotkey]\nEnumerateWithTriggerKeys=True\nEnumerateForwardKeys=\nEnumerateBackwardKeys=\nEnumerateSkipFirst=False\n\n[Hotkey/TriggerKeys]\n0=Zenkaku_Hankaku\n1=Control+space\n2=Shift+space\n3=Alt+grave\n" > /etc/skel/.config/fcitx5/config \
    && printf 'export GTK_IM_MODULE=fcitx\nexport QT_IM_MODULE=fcitx\nexport XMODIFIERS=@im=fcitx\nexport DBUS_SESSION_BUS_ADDRESS="unix:path=/tmp/dbus-session-vscode.sock"\n' > /etc/profile.d/fcitx5-ime.sh \
    && chmod +x /etc/profile.d/fcitx5-ime.sh

# Windows向けクロスコンパイルターゲット
RUN rustup target add x86_64-pc-windows-msvc

# Cargo ツール群（Windowsクロスビルド用、ライセンス調査用）
RUN cargo install cargo-xwin cargo-license

WORKDIR /workspace
