FROM rust:1-bookworm

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        clang \
        cmake \
        gdb \
        lldb \
        pkg-config \
        libasound2-dev \
        libudev-dev \
        libwayland-dev \
        libx11-dev \
        libxcursor-dev \
        libxi-dev \
        libxkbcommon-dev \
        libxrandr-dev \
    && rustup component add clippy rustfmt \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /workspace

CMD ["bash"]
