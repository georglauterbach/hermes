#! /usr/bin/env bash

set -eE -u -o pipefail
shopt -s inherit_errexit

readonly DOAS_CONFIG_FILE='/etc/doas.conf'

if [[ ! -e ${DOAS_CONFIG_FILE} ]]; then
  echo "Setting up 'doas'"
  sudo bash -c "echo \"permit persist ${USER}\" >\"${DOAS_CONFIG_FILE}\""
  sudo chown root:root "${DOAS_CONFIG_FILE}"
  sudo chmod 0400 "${DOAS_CONFIG_FILE}"
else
  if [[ -d ${DOAS_CONFIG_FILE} ]]; then
    echo "ERROR '${DOAS_CONFIG_FILE}' is a directory"
    exit 1
  fi
fi

if sudo doas -C "${DOAS_CONFIG_FILE}"; then
  echo 'Configuration for doas looks good'
else
  echo 'doas configuration has errors - do not use it immediately'
  exit 1
fi
