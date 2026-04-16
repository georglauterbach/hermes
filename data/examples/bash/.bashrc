#! /usr/bin/env bash

function download_hermes_latest() {
  local HERMES_LOCATION="${HOME}/.local/bin/hermes"
  local HERMES_RELEASE_URI_BASE='https://github.com/georglauterbach/hermes/releases'
  local HERMES_VERSION

  HERMES_VERSION=$(curl --silent --show-error --fail --location --write-out '%{url_effective}' --output /dev/null \
    "${HERMES_RELEASE_URI_BASE}/latest" | sed 's|.*/||')

  mkdir --parents "$(dirname "${HERMES_LOCATION}")"
  curl --silent --show-error --fail --location --output "${HERMES_LOCATION}" \
    "${HERMES_RELEASE_URI_BASE}/download/${HERMES_VERSION}/hermes-${HERMES_VERSION}-$(uname -m)-unknown-linux-musl"

  chmod +x "${HERMES_LOCATION}"
}

# shellcheck source=/dev/null
source "${HOME}/.config/bash/90-hermes.sh"

if __is_command 'doas'; then
  complete -cf doas
  alias sudo='doas'
fi

if __is_command 'gitui'; then
  alias g='gitui'
else
  alias g='git diff'
fi
