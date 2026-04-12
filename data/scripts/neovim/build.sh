#! /bin/sh

set -e -u

docker build --tag neovim-builder .
docker run --rm --volume ./out:/out neovim-builder
