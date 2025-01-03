#! /usr/bin/env -S bash -eE -u -o pipefail -O inherit_errexit

sudo apt-get --yes install gnupg2

readonly GNUPG_HOME_DIR="${HOME}/.gnupg"
mkdir -p "${GNUPG_HOME_DIR}"
sudo chown -R "${USER}:${USER}" "${GNUPG_HOME_DIR}"
find "${GNUPG_HOME_DIR}" -type f -exec chmod 600 {} \;
find "${GNUPG_HOME_DIR}" -type d -exec chmod 700 {} \;
