FROM mcr.microsoft.com/devcontainers/base:ubuntu

ARG USERNAME=vscode

RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    git \
    pkg-config \
    libssl-dev \
    lld \
    clang \
    && rm -rf /var/lib/apt/lists/*

USER ${USERNAME}

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH="/home/${USERNAME}/.cargo/bin:${PATH}"

# Add components
RUN rustup component add rustfmt clippy

# Faster linker
RUN mkdir -p /home/${USERNAME}/.cargo
RUN echo '[target.x86_64-unknown-linux-gnu]' >> /home/${USERNAME}/.cargo/config.toml
RUN echo 'linker = "clang"' >> /home/${USERNAME}/.cargo/config.toml
RUN echo 'rustflags = ["-C", "link-arg=-fuse-ld=lld"]' >> /home/${USERNAME}/.cargo/config.toml