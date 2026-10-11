FROM mcr.microsoft.com/devcontainers/rust:1

ENV TZ=Asia/Tokyo

# WSLg/GUI表示およびeframe/winitに必要なシステムライブラリ
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
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Windows向けクロスコンパイルターゲット
RUN rustup target add x86_64-pc-windows-msvc

# Cargo ツール群（Windowsクロスビルド用、ライセンス調査用）
RUN cargo install cargo-xwin cargo-license

WORKDIR /workspace
