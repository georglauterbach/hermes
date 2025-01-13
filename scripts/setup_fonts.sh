#! /usr/bin/env -S bash -eE -u -o pipefail -O inherit_errexit

if [[ ${EUID} -ne 0 ]]; then
  echo "ERROR: This script needs to run with superuser privileges" >&2
  exit 1
fi

readonly NERD_FONT_VERSION='v3.3.0'

function download_extract_place() {
  local FONT_NAME=${1:?Font name required}

  local URI="https://github.com/ryanoasis/nerd-fonts/releases/download/${NERD_FONT_VERSION}/${FONT_NAME}.tar.xz"
  local TARGET_DIR="/usr/local/share/fonts/${FONT_NAME}-Nerd-Font"

  rm -rf "${TARGET_DIR}"
  mkdir -p "${TARGET_DIR}"
  curl -sSfL "${URI}" | tar xJ -C "${TARGET_DIR}"
  chown -R root:root "${TARGET_DIR}"
}

download_extract_place 'FiraCode'
download_extract_place 'JetBrainsMono'

fc-cache -fv
