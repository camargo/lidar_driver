FROM ubuntu:22.04

RUN apt update; \
    apt upgrade; \
    DEBIAN_FRONTEND=noninteractive \
    apt install -y \
        build-essential \
        curl \
        git \
        libudev-dev \
        pkg-config \
        socat \
        vim;

RUN mkdir rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rust/rustup-installer.sh
RUN chmod 754 rust/rustup-installer.sh && rust/rustup-installer.sh -y
