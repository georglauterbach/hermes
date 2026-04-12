# We accept the following hadolint issues because
# this is a build container.
#
# hadolint global ignore=DL3018,DL3059

FROM docker.io/alpine:3.23

RUN apk add --no-cache build-base cmake coreutils curl gettext-tiny-dev git linux-headers tar xz

RUN git clone --depth=1 --branch=v0.12.1 https://github.com/neovim/neovim.git
WORKDIR /neovim

RUN make -j"$(nproc)" CMAKE_BUILD_TYPE=Release CMAKE_EXTRA_FLAGS="-DSTATIC_BUILD=1"
RUN mv build/bin/nvim . && strip -s nvim && tar cJf "nvim_$(uname -m).tar.xz" nvim

ENTRYPOINT ["/bin/sh", "-c"]
CMD ["cp /neovim/nvim_*.tar.xz /out/"]
