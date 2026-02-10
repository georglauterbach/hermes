FROM docker.io/alpine:3.23

RUN apk add --no-cache \
    curl autoconf gcc flex bison make bash cmake libtool musl-dev g++ \
    zlib-dev zlib-static \
    tcl tk \
    tcl-dev gettext

ENV GIT_VERSION=2.53.0

RUN curl -ssfL -o "git_v${GIT_VERSION}.tar.gz" \
    "https://github.com/git/git/archive/v${GIT_VERSION}.tar.gz"
RUN tar xf "git_v${GIT_VERSION}.tar.gz" && mv "/git-${GIT_VERSION}" /git
WORKDIR /git

RUN make configure && sed -i s/qversion/-version/g configure
RUN ./configure prefix=/git/out/ LDFLAGS=--static CFLAGS="${CFLAGS} -O3 -static"
RUN make -j"$(nproc)" && make install

ENTRYPOINT ["/bin/sh", "-c"]
CMD ["cp /git/out/bin/git /out/"]
