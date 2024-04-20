#! /usr/bin/env bash

# shellcheck disable=SC2034

set -eu

if [[ ${EUID} -ne 0 ]]; then
  # shellcheck disable=SC2312
  sudo env -                 \
    USER="${USER}"           \
    HOME="${HOME}"           \
    LOG_LEVEL="${LOG_LEVEL:-trace}" \
    bash "$(realpath -eL "${BASH_SOURCE[0]}")" "${@}"

  exit
fi

function preflight_checks() {
  if ! command -v curl &>/dev/null; then
    echo "The command 'curl' is not installed but required" >&2
    exit 1
  fi
}

function parse_command_line_arguments() {
  while [[ ${#} -gt 0 ]]; do
    case ${1:-} in
      ( '--gui' )
        GUI=1
        shift 1
        ;;

      ( '--local' )
        LOCAL_INSTALLATION=1
        shift 1
        ;;

      ( * )
        log 'error' "Unknown argument '${1:-}'"
        exit 1
        ;;
    esac
  done
}

function user_setup() {
  log 'info' 'Starting user setup'

  # Place configuration files
  local CONFIG
  for CONFIG in "${!USER_CONFIGS[@]}"; do
    local DESTINATION="${USER_CONFIGS[${CONFIG}]}"
    su -s /usr/bin/bash "${USER}" -c "mkdir -p '$(dirname "${DESTINATION}")'"

    if [[ ${LOCAL_INSTALLATION} -eq 0 ]]; then
      log 'trace' "Installing configuration file '${DESTINATION}' from '${GITHUB_RAW_URI}/${CONFIG}'"
      su -s /usr/bin/bash "${USER}" -c \
        "curl -sSfL -o '${DESTINATION}' '${GITHUB_RAW_URI}/${CONFIG}'"
    else
      log 'trace' "Installing configuration file '${DESTINATION}' from local source '${CONFIG}'"
      su -s /usr/bin/bash "${USER}" -c "cp '${SCRIPT_DIR}/${CONFIG}' '${DESTINATION}'"
    fi
  done
}

function root_setup() {
  log 'info' 'Starting root setup'

  # Place configuration files
  local CONFIG
  for CONFIG in "${!ROOT_CONFIGS[@]}"; do
    local DESTINATION="${ROOT_CONFIGS[${CONFIG}]}"
    mkdir -p "$(dirname "${DESTINATION}")"

    if [[ ${LOCAL_INSTALLATION} -eq 0 ]]; then
      log 'trace' "Installing configuration file '${DESTINATION}' from '${GITHUB_RAW_URI}/${CONFIG}'"
      curl -sSfL -o "${DESTINATION}" "${GITHUB_RAW_URI}/${CONFIG}"
    else
      log 'trace' "Installing configuration file '${DESTINATION}' from local source '${CONFIG}'"
      cp "${SCRIPT_DIR}/${CONFIG}" "${DESTINATION}"
    fi
  done

  local CODE_SOURCES_FILE='/etc/apt/sources.list.d/vscode.list'
  if [[ -f ${CODE_SOURCES_FILE} ]]; then
    log 'trace' 'Applying VS Code PPA patch'
    sed -i 's/^deb/#deb/g' "${CODE_SOURCES_FILE}"
  fi

  export DEBIAN_FRONTEND=noninteractive
  export DEBCONF_NONINTERACTIVE_SEEN=true
  log 'debug' 'Updating package signatures'
  apt-get -qq update
  log 'debug' 'Upgrading existing packages'
  apt-get -qq upgrade
  log 'debug' 'Auto-removing unused packages'
  apt-get -qq autoremove

  # Install packages
  if [[ ${LOCAL_INSTALLATION} -eq 0 ]]; then
    readarray -t PACKAGES < <(curl -sSfL "${GITHUB_RAW_URI}/data/versioned/${VERSION_ID}/no_gui/packages.txt")
    readarray -t GUI_PACKAGES < <(curl -sSfL "${GITHUB_RAW_URI}/data/versioned/${VERSION_ID}/gui/packages.txt")
  else
    readarray -t PACKAGES < "${SCRIPT_DIR}/data/versioned/${VERSION_ID}/no_gui/packages.txt"
    readarray -t GUI_PACKAGES < "${SCRIPT_DIR}/data/versioned/${VERSION_ID}/gui/packages.txt"
  fi

  log 'debug' "Installing non-GUI packages (${PACKAGES[*]})"
  apt-get -qq install "${PACKAGES[@]}"

  if [[ ${GUI} -eq 1 ]]; then
    log 'debug' "Installing GUI packages (${GUI_PACKAGES[*]})"
    apt-get -qq install "${GUI_PACKAGES[@]}"
  fi

  log 'debug' 'To install ble.sh, visit https://github.com/akinomyoga/ble.sh'
  log 'debug' 'Installing Starship prompt'
  wget -q -O- 'https://starship.rs/install.sh' | sh -s -- --force >/dev/null
  log 'debug' 'Installing fzf'
  (
    git clone --depth 1 'https://github.com/junegunn/fzf.git' "${HOME}/.fzf"
    cd "${HOME}"
    bash '.fzf/install' --key-bindings --completion --no-update-rc --no-zsh --no-fish >/dev/null
  )
  log 'debug' 'Installing gitui'
  curl -sSfL 'https://github.com/extrawurst/gitui/releases/download/v0.26.1/gitui-linux-x86_64.tar.gz' \
    | tar -xz -C /usr/local/bin
}

function main() {
  preflight_checks

  # shellcheck source=/dev/null
  source <(curl -qsSfL https://raw.githubusercontent.com/georglauterbach/libbash/main/load) \
    --version '6.1.1' --online 'log' 'errors'

  source /etc/os-release

  SCRIPT_DIR="$(realpath -eL "$(dirname "${BASH_SOURCE[0]}")")"
  GITHUB_RAW_URI='https://raw.githubusercontent.com/georglauterbach/hermes/main'
  readonly SCRIPT_DIR GITHUB_RAW_URI

  SCRIPT='hermes'
  GUI=0
  LOCAL_INSTALLATION=0

  log 'trace' "Starting"
  parse_command_line_arguments "${@}"

  log 'info' "Ubuntu version is '${VERSION_ID}'"

  local LOCATIONS=('data/unversioned/no_gui' "data/versioned/${VERSION_ID}/no_gui")
  if [[ ${GUI} -eq 1 ]]; then
    log 'info' 'GUI will be installed too'
    LOCATIONS+=('data/unversioned/gui' "data/versioned/${VERSION_ID}/gui")
  else
    log 'info' 'GUI will not be installed'
  fi

  # We parse all index files into associative arrays so we can handle them later.
  declare -A USER_CONFIGS ROOT_CONFIGS
  local LOCATION
  for LOCATION in "${LOCATIONS[@]}"; do
    local SOURCE DESTINATION
    while read -r SOURCE DESTINATION; do
      local EXPANDED_DESTINATION
      EXPANDED_DESTINATION=$(eval "echo \"${DESTINATION}\"")
      # shellcheck disable=SC2016
      if [[ ${DESTINATION} == '${HOME}/'* ]]; then
        USER_CONFIGS[${SOURCE}]=${EXPANDED_DESTINATION}
      else
        ROOT_CONFIGS[${SOURCE}]=${EXPANDED_DESTINATION}
      fi
    done < <(/usr/bin/grep -E -v "^\s*$|^\s*#" "${LOCATION}/index.txt")
  done

  user_setup || return ${?}
  root_setup || return ${?}

  if [[ ${LOCAL_INSTALLATION} -eq 0 ]]; then
    # shellcheck source=/dev/null
    source <(curl -qsSfL "${GITHUB_RAW_URI}/data/versioned/${VERSION_ID}/post_setup.sh")
  else
    # shellcheck source=/dev/null
    source "${SCRIPT_DIR}/data/versioned/${VERSION_ID}/post_setup.sh"
  fi

  log 'info' 'Finished'
}

main "${@}"
