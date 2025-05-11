#! /usr/bin/env bash

set -eE -u -o pipefail
shopt -s inherit_errexit

sudo apt-get --yes install --no-install-recommends doas

readonly DOAS_CONFIG_FILE='/etc/doas.conf'

if [[ -d ${DOAS_CONFIG_FILE} ]]; then
  echo "ERROR: Configuration location '${DOAS_CONFIG_FILE}' is a directory"
  exit 1
elif [[ ! -f ${DOAS_CONFIG_FILE} ]]; then
  sudo bash -c "echo 'permit persist ${USER}' >${DOAS_CONFIG_FILE}"
  sudo chown root:root "${DOAS_CONFIG_FILE}"
  sudo chmod 0400 "${DOAS_CONFIG_FILE}"
fi

if doas -C "${DOAS_CONFIG_FILE}"; then
  echo 'Configuration for doas looks good'
else
  echo 'Configuration has errors - do not use it immediately'
  exit 1
fi
