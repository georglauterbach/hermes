#! /bin/sh

set -e -u

mkdir -p x86_64 aarch64

cd programs

for DIR in git neovim; do
  cd "${DIR}"
  earthly +build-all-platforms
  [ -d out/arm64 ] && cp -f out/arm64/* ../../aarch64/
  [ -d out/amd64 ] && cp -f out/amd64/* ../../x86_64/
  cd ..
done

cd ..

strip -s x86_64/*
tar cJf hermes-custom.tar.xz x86_64 aarch64
