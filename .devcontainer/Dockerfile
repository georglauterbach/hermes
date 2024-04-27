FROM docker.io/ubuntu:24.04

SHELL [ "/usr/bin/env", "bash", "-eE", "-u", "-o", "pipefail", "-c" ]

RUN <<EOM
  export DEBIAN_FRONTEND=noninteractive
  export DEBCONF_NONINTERACTIVE_SEEN=true
  apt-get --yes update
  # we require git and gnupg2 to make the Dev Container work properly
  apt-get --yes install curl doas git gnupg2
EOM

RUN <<EOM
  echo 'permit nopass ubuntu' >/etc/doas.conf
  chown -c root:root /etc/doas.conf
  chmod -c 0400 /etc/doas.conf
  doas -C /etc/doas.conf
  ln -s "$(command -v doas)" /usr/local/bin/sudo
EOM
