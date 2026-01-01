# syntax=docker.io/docker/dockerfile:1

FROM docker.io/rust:1.91.0-slim-trixie

ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=aarch64-linux-gnu-gcc
ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=x86_64-linux-musl-gcc

RUN useradd --create-home --shell /bin/bash user     \
    && passwd --delete root                          \
    && apt-get update                                \
    && apt-get install --yes --no-install-recommends \
      git ssh                                        \
      gcc musl-tools pkg-config zstd                 \
      binutils-aarch64-linux-gnu gcc-aarch64-linux-gnu
