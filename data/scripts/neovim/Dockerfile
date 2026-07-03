# We accept the following hadolint issues because
# this is a build container.
#
# hadolint global ignore=DL3008,DL3009,DL3015

FROM docker.io/ubuntu:24.04

RUN apt-get -qq update \
    && apt-get -qq install build-essential ca-certificates cmake gettext git ninja-build

RUN git clone --depth=1 --branch=v0.12.3 https://github.com/neovim/neovim.git
WORKDIR /neovim

# We do not link statically as linking glibc statically causes all sorts of
# issues; in our case, we have issues with NSS..
RUN make -j"$(nproc)" CMAKE_BUILD_TYPE=Release CMAKE_EXTRA_FLAGS="-DUSE_BUNDLED=1" \
    && make install DESTDIR=/neovim/install

RUN strip -s /neovim/install/usr/local/bin/nvim \
    && tar cJf "nvim_$(uname -m).tar.xz" -C /neovim/install/usr/local .

ENTRYPOINT ["/bin/sh", "-c"]
CMD ["cp -rf /neovim/install /neovim/nvim_*.tar.xz /out/"]
