#! /usr/bin/env -S bash -eE -u -o pipefail -O inherit_errexit

if ! command -v git &>/dev/null; then
  echo "ERROR Command 'git' is required but not installed or in PATH" >&2
  exit 1
fi

readonly LOCAL_ICONS_DIR="${HOME}/.local/share/icons"
readonly GIT_ICONS_DIR="${LOCAL_ICONS_DIR}/.Gruvbox-Plus"

mkdir -p "${LOCAL_ICONS_DIR}"

if [[ -d ${GIT_ICONS_DIR} ]]; then
  cd ${GIT_ICONS_DIR}
  git pull
else
  git clone 'https://github.com/SylEleuth/gruvbox-plus-icon-pack.git' "${GIT_ICONS_DIR}"
fi

ln -fs "${GIT_ICONS_DIR}/Gruvbox-Plus-Light" "${LOCAL_ICONS_DIR}/Gruvbox-Plus-Light"
ln -fs "${GIT_ICONS_DIR}/Gruvbox-Plus-Dark" "${LOCAL_ICONS_DIR}/Gruvbox-Plus-Dark"
