#! /bin/sh

set -e -u

docker build --tag neovim-builder --file Dockerfile .
docker run --rm --volume ./out:/out neovim-builder
