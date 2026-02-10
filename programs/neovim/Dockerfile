FROM docker.io/alpine:3.23

RUN apk add --no-cache \
    build-base cmake coreutils curl gettext-tiny-dev git

ARG NEOVIM_VERSION=0.11.5

RUN curl -sSfL -o "neovim_v${NEOVIM_VERSION}.tar.gz" \
    "https://github.com/neovim/neovim/archive/refs/tags/v${NEOVIM_VERSION}.tar.gz"
RUN tar xf "neovim_v${NEOVIM_VERSION}.tar.gz" && mv "/neovim-${NEOVIM_VERSION}" /neovim
WORKDIR /neovim

RUN make -j"$(nproc)" CMAKE_BUILD_TYPE=Release CMAKE_EXTRA_FLAGS="-DSTATIC_BUILD=1"

ENTRYPOINT ["/bin/sh", "-c"]
CMD ["cp build/bin/nvim /out/"]
