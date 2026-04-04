#! /bin/sh

set -e -u

docker compose up --build
strip -s -o nvim out/nvim
tar cJf "nvim_$(uname -m).tar.xz" nvim
