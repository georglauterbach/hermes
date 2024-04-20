FROM docker.io/ubuntu:23.10

SHELL [ "/usr/bin/env", "bash", "-eE", "-u", "-o", "pipefail", "-c" ]

RUN <<EOM
  export DEBIAN_FRONTEND=noninteractive
  export DEBCONF_NONINTERACTIVE_SEEN=true
  apt-get --yes update
  apt-get --yes upgrade
  apt-get --yes install curl doas
EOM

RUN <<EOM
  echo 'permit nopass ubuntu' >/etc/doas.conf
  chown -c root:root /etc/doas.conf
  chmod -c 0400 /etc/doas.conf
  doas -C /etc/doas.conf
  ln -s "$(command -v doas)" /usr/local/bin/sudo
EOM
