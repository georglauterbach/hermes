FROM docker.io/alpine:3.23

RUN apk add --no-cache build-base cmake coreutils curl gettext-tiny-dev git

RUN git clone --depth=1 --branch=v0.11.6 https://github.com/neovim/neovim.git
WORKDIR /neovim

RUN make -j"$(nproc)" CMAKE_BUILD_TYPE=Release CMAKE_EXTRA_FLAGS="-DSTATIC_BUILD=1"

ENTRYPOINT ["/bin/sh", "-c"]
CMD ["cp build/bin/nvim /out/"]
