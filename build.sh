#! /bin/sh

set -e -u

mkdir -p x86_64 aarch64

cd programs
for DIR in git neovim; do
  cd "${DIR}"
  docker compose up --build
  command find out -type f -executable | while read -r EXECUTABLE; do
    strip -s -o "../../x86_64/$(basename "${EXECUTABLE}")" "${EXECUTABLE}"
  done
  cd ..
done
cd ..

tar cJf hermes-custom.tar.xz x86_64 aarch64
