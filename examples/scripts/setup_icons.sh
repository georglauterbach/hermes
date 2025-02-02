#! /usr/bin/env -S bash -eE -u -o pipefail -O inherit_errexit

if ! command -v git &>/dev/null; then
  echo "ERROR Command 'git' is required but not installed or in PATH" >&2
  exit 1
fi

readonly LOCAL_ICONS_DIR="${HOME}/.local/share/icons"
mkdir -p "${LOCAL_ICONS_DIR}"

function setup_icons_dark() {
  local DIR_NAME='.Gruvbox-Plus'
  local GIT_ICONS_DIR="${LOCAL_ICONS_DIR}/${DIR_NAME}"

  if [[ -d ${GIT_ICONS_DIR} ]]; then
    cd "${GIT_ICONS_DIR}"
    git pull
  else
    git clone 'https://github.com/SylEleuth/gruvbox-plus-icon-pack.git' "${GIT_ICONS_DIR}"
  fi

  cd "${LOCAL_ICONS_DIR}"
  for VARIANT in 'Gruvbox-Plus-Light' 'Gruvbox-Plus-Dark'; do
    rm -f "${VARIANT,,}"
    ln -fs "${DIR_NAME}/${VARIANT}/" "${VARIANT,,}"
  done
}

function setup_icons_light() {
  local DIR_NAME='.Everforest-GTK-Theme'
  local GIT_ICONS_DIR="${LOCAL_ICONS_DIR}/${DIR_NAME}"

  if [[ -d ${GIT_ICONS_DIR} ]]; then
    cd "${GIT_ICONS_DIR}"
    git pull
  else
    git clone 'https://github.com/SylEleuth/gruvbox-plus-icon-pack.git' "${GIT_ICONS_DIR}"
  fi

  cd "${LOCAL_ICONS_DIR}"
  for VARIANT in 'Everforest-Dark' 'Everforest-Light'; do
    rm -f "${VARIANT,,}"
    ln -fs "${DIR_NAME}/icons/${VARIANT}/" "${VARIANT,,}"
  done
}

setup_icons_dark
setup_icons_light
