FROM rust:1.82-slim-bookworm

WORKDIR /app

COPY api/Cargo.toml ./Cargo.toml
COPY api/src ./src

RUN apt-get update && apt-get install -y \
    libgstreamer1.0-dev \
    libgstreamer-plugins-base1.0-dev \
    gstreamer1.0-tools \
    gstreamer1.0-plugins-good \
    gstreamer1.0-plugins-bad \
    gstreamer1.0-libav \
    pkg-config \
    build-essential \
    cmake \
    git \
    && rm -rf /var/lib/apt/lists/*
