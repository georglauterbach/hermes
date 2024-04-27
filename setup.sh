#! /usr/bin/env bash

# shellcheck disable=SC2034

set -eE -u

if [[ ${EUID} -ne 0 ]]; then
  # shellcheck disable=SC2312
  sudo env -                        \
    USER="${USER}"                  \
    HOME="${HOME}"                  \
    PATH="${PATH}"                  \
    LOG_LEVEL="${LOG_LEVEL:-info}"  \
    bash "$(realpath -eL "${BASH_SOURCE[0]}")" --assume-correct-incovation "${@}"

  exit ${?}
fi

if [[ ${*} != *--assume-correct-incovation* ]]; then
  echo 'ERROR: Do not start this script as root yourself' >&2
  exit 1
fi

function preflight_checks() {
  if ! command -v 'curl' &>/dev/null; then
    log 'error' "The command 'curl' is not installed but required for installation type 'remote'"
    exit 1
  fi

  if [[ ! ${VERSION_ID} =~ ^(23.10|24.04)$ ]]; then
    log 'error' "Ubuntu version '${VERSION}' is not supported" >&2
    exit 1
  fi

  if [[ $(uname -m) != 'x86_64' ]]; then
    log 'error' "The only supported architecture is x86_64 (yours is '$(uname -m)')"
    exit 1
  fi
}

function parse_command_line_arguments() {
  while [[ ${#} -gt 0 ]]; do
    case ${1:-} in
      ( '--gui' | '-g' )                     GUI=1                    ;;
      ( '--local-installation' | '-l' )      LOCAL_INSTALLATION=1     ;;
      ( '--assume-data-is-correct'  | '-a' ) ASSUME_DATA_IS_CORRECT=1 ;;
      ( '--assume-correct-incovation' )                               ;;

      ( * )
        echo "ERROR: Unknown argument '${1:-}'" >&2
        exit 1
        ;;
    esac

    shift 1
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
  apt-get -qq install --no-install-recommends --no-install-suggests "${PACKAGES[@]}"

  if [[ ${GUI} -eq 1 ]]; then
    log 'debug' "Installing GUI packages (${GUI_PACKAGES[*]})"
    apt-get -qq install --no-install-recommends --no-install-suggests "${GUI_PACKAGES[@]}"
  fi
}

function user_setup() {
  log 'info' 'Starting user setup'

  # Place configuration files
  local CONFIG
  for CONFIG in "${!USER_CONFIGS[@]}"; do
    local DESTINATION="${USER_CONFIGS[${CONFIG}]}"
    mkdir -p "$(dirname "${DESTINATION}")"

    if [[ ${LOCAL_INSTALLATION} -eq 0 ]]; then
      log 'trace' "Installing configuration file '${DESTINATION}' from '${GITHUB_RAW_URI}/${CONFIG}'"
      curl -sSfL -o "${DESTINATION}" "${GITHUB_RAW_URI}/${CONFIG}"
    else
      log 'trace' "Installing configuration file '${DESTINATION}' from local source '${CONFIG}'"
      cp "${SCRIPT_DIR}/${CONFIG}" "${DESTINATION}"
    fi
  done

  mkdir -p "${HOME}/.local/bin"
  [[ ${PATH} == *${HOME}/.local/bin* ]] || export PATH="${HOME}/.local/bin:${PATH}"
  [[ ${PATH} == *${HOME}/.fzf/bin* ]] || export PATH="${HOME}/.fzf/bin:${PATH}"

  if command -v 'fzf' &>/dev/null; then
    log 'debug' "fzf seems to be installed already"
  else
    log 'debug' 'Installing fzf'
    (
      git clone --quiet --depth 1 'https://github.com/junegunn/fzf.git' "${HOME}/.fzf"
      cd "${HOME}"
      bash '.fzf/install' --key-bindings --completion --no-update-rc --no-zsh --no-fish &>/dev/null
    )
  fi

  if command -v 'zoxide' &>/dev/null; then
    log 'debug' "zoxide seems to be installed already"
  else
    log 'debug' 'Installing zoxide'
    curl -sSfL 'https://raw.githubusercontent.com/ajeetdsouza/zoxide/main/install.sh' | bash
  fi

  if command -v 'gitui' &>/dev/null; then
    log 'debug' "gitui seems to be installed already"
  else
    log 'debug' 'Installing gitui'
    curl -sSfL 'https://github.com/extrawurst/gitui/releases/download/v0.26.1/gitui-linux-x86_64.tar.gz' \
      | tar -xz -C "${HOME}/.local/bin"
  fi

  if command -v 'starship' &>/dev/null; then
    log 'debug' "Starship seems to be installed already"
  else
    log 'debug' 'Installing Starship'
    curl -sSfL 'https://starship.rs/install.sh' \
      | sh -s -- --bin-dir="${HOME}/.local/bin" --force >/dev/null
  fi

  chown -R "${USER}:$(id -g "${USER}")" "${HOME}"

  log 'debug' 'To install ble.sh, visit https://github.com/akinomyoga/ble.sh'
}

function main() {
  SCRIPT_DIR="$(realpath -eL "$(dirname "${BASH_SOURCE[0]}")")"
  GITHUB_RAW_URI='https://raw.githubusercontent.com/georglauterbach/hermes/main'
  readonly SCRIPT_DIR GITHUB_RAW_URI

  GUI=0
  LOCAL_INSTALLATION=0
  ASSUME_DATA_IS_CORRECT=0

  source /etc/os-release
  readonly VERSION VERSION_ID

  parse_command_line_arguments "${@}"
  readonly GUI LOCAL_INSTALLATION

  if [[ ${LOCAL_INSTALLATION} -eq 0 ]]; then
    # shellcheck source=/dev/null
    source <(curl -qsSfL https://raw.githubusercontent.com/georglauterbach/libbash/main/load) \
      --online --version '6.1.1' 'log' 'errors'
  else
    function log() {
      printf "%s  %-5s  %s  --  %s\n" \
        "$(date --iso-8601=seconds)" "${1^^}" "${SCRIPT:-${0}}" "${2}"
    }
  fi
  export SCRIPT='hermes'

  log 'trace' "Starting"
  log 'info' "Ubuntu version is '${VERSION}'"

  if [[ ${LOCAL_INSTALLATION} -eq 0 ]]; then
    log 'info' "Installation type is: remote (default)"
  else
    log 'info' "Installation type is: local"
  fi

  if [[ ${GUI} -eq 0 ]]; then
    log 'info' 'GUI will not be installed (default)'
  else
    log 'info' 'GUI will be installed'
  fi

  preflight_checks

  if [[ ${ASSUME_DATA_IS_CORRECT} -eq 0 ]]; then
    read -r -p "Does the information printed above look correct? [Y/n] " IS_CORRECT
    if [[ ! ${IS_CORRECT} =~ ^(y|yes|)$ ]]; then
      log 'error' 'Aborted due to user input'
      exit 1
    fi
  fi

  local LOCATIONS=('data/unversioned/no_gui' "data/versioned/${VERSION_ID}/no_gui")
  if [[ ${GUI} -eq 1 ]]; then
    LOCATIONS+=('data/unversioned/gui' "data/versioned/${VERSION_ID}/gui")
  fi

  # We parse all index files into associative arrays so we can handle them later.
  declare -A USER_CONFIGS ROOT_CONFIGS
  local LOCATION
  for LOCATION in "${LOCATIONS[@]}"; do
    local SOURCE DESTINATION INDEX_FILE_CONTENT

    if [[ ${LOCAL_INSTALLATION} -eq 0 ]]; then
      INDEX_FILE_CONTENT=$(curl -qsSfL "${GITHUB_RAW_URI}/${LOCATION}/index.txt" | grep -E -v "^\s*$|^\s*#")
    else
      INDEX_FILE_CONTENT=$(grep -E -v "^\s*$|^\s*#" "${LOCATION}/index.txt")
    fi

    while read -r SOURCE DESTINATION; do
      local EXPANDED_DESTINATION
      EXPANDED_DESTINATION=$(eval "echo \"${DESTINATION}\"")

      # shellcheck disable=SC2016
      if [[ ${DESTINATION} == '${HOME}/'* ]]; then
        USER_CONFIGS[${SOURCE}]=${EXPANDED_DESTINATION}
      else
        ROOT_CONFIGS[${SOURCE}]=${EXPANDED_DESTINATION}
      fi
    done <<< "${INDEX_FILE_CONTENT}"
  done

  root_setup || return ${?}
  user_setup || return ${?}

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
