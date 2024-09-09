#! /usr/bin/env bash

# shellcheck disable=SC2154

if [[ ${EUID} -ne 0 ]]; then
  echo "ERROR Run this script with superuser privileges!" >&2
  exit 1
fi

echo "INFO  Reading new locale settings now"

PREVIOUS_LOCALE='en_US.UTF-8'
LOCALES=(
  LANG LC_ADDRESS LC_NAME LC_MONETARY LC_PAPER LC_IDENTIFICATION LC_TELEPHONE LC_MEASUREMENT LC_TIME LC_NUMERIC
)

function read_locale() {
  local __LOCALE
  read -r -p "Which locale do you want to use for ${1}? [default=${2}] " '__LOCALE'
  declare -g "__${1}=${__LOCALE:-${2}}"
  declare -g "PREVIOUS_LOCALE=${__LOCALE:-${2}}"
}

for __LOCALE in "${LOCALES[@]}"; do
  read_locale "${__LOCALE}" "${PREVIOUS_LOCALE}"
done
unset __LOCALE

rm -r -f -v /var/lib/locales/supported.d/*
rm -r -f -v /usr/lib/locale/*

sed -i -E 's/^([a-zA-Z]+)/# \1/g' /etc/locale.gen

echo "INFO  Setting LANGUAGE and LC_CTYPE to the value of LANG (${__LANG})"
NEW_LOCALES=("LANGUAGE=${__LANG}" "LC_CTYPE=${__LANG}")

for __LOCALE in "${LOCALES[@]}"; do
  __NEW_LOCALE="__${__LOCALE}"
  sed -i -E "s/# (${!__NEW_LOCALE}.*)/\1/g" /etc/locale.gen
  NEW_LOCALES+=("${__LOCALE}=${!__NEW_LOCALE}")
done
unset __LOCALE

echo "INFO  Generating new locales"
locale-gen --purge

echo "INFO  Updating new locales"
update-locale "${NEW_LOCALES[@]}"

echo "INFO  Here are the contents of /etc/default/locale"
cat /etc/default/locale
