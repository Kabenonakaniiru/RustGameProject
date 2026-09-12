FROM mcr.microsoft.com/devcontainers/rust:1

ENV TZ=Asia/Tokyo

# WSLg/GUI表示およびeframe/winitに必要なシステムライブラリ
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
        pkg-config \
        libx11-dev \
        libasound2-dev \
        libudev-dev \
        libwayland-dev \
        libxkbcommon-dev \
        libegl1-mesa-dev \
        libgl1 \
        libglx-mesa0 \
        mesa-vulkan-drivers \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

WORKDIR /workspace
