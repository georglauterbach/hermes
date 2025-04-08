#! /usr/bin/env -S bash -eE -u -o pipefail -O inherit_errexit

set -eE -u -o pipefail
shopt -s inherit_errexit

readonly GNUPG_HOME_DIR="${HOME}/.gnupg"
mkdir -p "${GNUPG_HOME_DIR}"

sudo chown -R "$(id --user):$(id --group)" "${GNUPG_HOME_DIR}"
sudo apt-get --yes install gnupg2

find "${GNUPG_HOME_DIR}" -type f -exec chmod 600 {} \;
find "${GNUPG_HOME_DIR}" -type d -exec chmod 700 {} \;
