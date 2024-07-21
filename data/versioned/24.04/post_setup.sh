#! /usr/bin/env bash

function setup_doas() {
  local DOAS_CONFIG_FILE='/etc/doas.conf'
  readonly DOAS_CONFIG_FILE
  if [[ ! -e ${DOAS_CONFIG_FILE} ]]; then
    log 'debug' "Setting up 'doas'"
    echo "permit persist ${USER}" >"${DOAS_CONFIG_FILE}"
    chown root:root "${DOAS_CONFIG_FILE}"
    chmod 0400 "${DOAS_CONFIG_FILE}"

    if doas -C "${DOAS_CONFIG_FILE}"; then
      log 'trace' 'Configuration for doas looks good'
    else
      log 'warn' 'doas configuration has errors - do not use it immediately'
    fi
  fi
}

function setup_gnupg() {
  local GNUPG_HOME_DIR="${HOME}/.gnupg"
  mkdir -p "${GNUPG_HOME_DIR}"
  chown -R "${USER}:${USER}" "${GNUPG_HOME_DIR}"
  find "${GNUPG_HOME_DIR}" -type f -exec chmod 600 {} \;
  find "${GNUPG_HOME_DIR}" -type d -exec chmod 700 {} \;
}

function setup_snap() {
  if command -v snap &>/dev/null; then
    log 'info' "Purging 'snapd'"

    killall snap
    systemctl stop snapd

    until [[ $(snap list 2>&1 || :) == 'No snaps'*'installed'* ]]; do
      while read -r SNAP _; do
        snap remove --purge "${SNAP}" &>/dev/null || :
      done < <(snap list |& tail -n +2 || :)
    done

    apt-get -qq purge snapd gnome-software-plugin-snap
    apt-mark -qq hold snapd
    rm -rf /var/cache/snapd/ "${HOME}/snapd" "${HOME}/snap"

    log 'debug' "Finished purging 'snapd'"
  fi
}

function setup_misc() {
  # shellcheck disable=SC2154
  if [[ ${GUI} -eq 1 ]]; then
    log 'debug' 'To change the bookmarks in Nautilus, edit ~/.config/user-firs.dirs, ~/.config/gtk-3.0/bookmarks, and /etc/xdg/user-dirs.defaults'
  fi
}

setup_doas
setup_gnupg
setup_snap
setup_misc
