#! /usr/bin/env bash

set -eE -u -o pipefail
shopt -s inherit_errexit

if [[ ${EUID} -ne 0 ]]; then
  echo "ERROR: This script needs to run with superuser privileges" >&2
  exit 1
fi

function purge_old_config_files() {
  local UNNECESSARY_CONFIG_FILES
  readarray -t UNNECESSARY_CONFIG_FILES < <(dpkg -l | awk '/^rc/{print $2}')
  if [[ ${#UNNECESSARY_CONFIG_FILES} -gt 0 ]]; then
    dpkg --purge "${UNNECESSARY_CONFIG_FILES[@]}"
  fi
}

function purge_old_kernels() {
  local CURRENT_KERNEL_VERSION OLD_LINUX_IMAGES OLD_LINUX_HEADERS

  CURRENT_KERNEL_VERSION=$(uname -r | sed -E "s/([0-9.-]*)-([^0-9]+)/\1/")
  readarray -t OLD_LINUX_IMAGES  < <(dpkg --list | grep 'linux-image'   | awk '{print $2}' \
    | sort -V | sed -n "/$(uname -r)/q;p")
  readarray -t OLD_LINUX_HEADERS < <(dpkg --list | grep 'linux-headers' | awk '{print $2}' \
    | sort -V | sed -n "/${CURRENT_KERNEL_VERSION}/q;p")

  apt-get --yes purge "${OLD_LINUX_IMAGES[@]}" "${OLD_LINUX_HEADERS[@]}"
  apt-get --yes autoremove --purge
}

purge_old_config_files
purge_old_kernels
purge_old_config_files
