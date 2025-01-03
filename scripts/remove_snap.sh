#! /usr/bin/env -S bash -eE -u -o pipefail -O inherit_errexit

if [[ ${EUID} -ne 0 ]]; then
  echo "ERROR: This script needs to run with superuser privileges" >&2
  exit 1
fi

if command -v snap &>/dev/null; then
  echo "Purging 'snapd'"

  killall snap &>/dev/null || :

  until [[ $(snap list 2>&1 || :) == 'No snaps'*'installed'* ]]; do
    while read -r SNAP _; do
      snap remove --purge "${SNAP}" &>/dev/null || :
    done < <(snap list |& tail -n +2 || :)
  done

  systemctl stop snapd.service snapd.socket
  apt-get -qq purge snapd gnome-software-plugin-snap
  apt-mark -qq hold snapd
  rm -rf /var/cache/snapd/ "${HOME}/snapd" "${HOME}/snap"

  echo "Finished purging 'snapd'"
fi
