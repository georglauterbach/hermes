# syntax=docker.io/docker/dockerfile:1

FROM docker.io/rust:1.90.0-slim-trixie

RUN <<EOF
    apt-get --yes update
    # CI
    apt-get --yes install binutils-aarch64-linux-gnu gcc gcc-aarch64-linux-gnu musl-tools pkg-config zstd
    # development
    apt-get --yes install git ssh

    # create a custom user
    useradd --create-home --shell /bin/bash user

    # ensure we can exercise elevated privileges
    passwd --delete root
EOF

ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=aarch64-linux-gnu-gcc
ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=x86_64-linux-musl-gcc
