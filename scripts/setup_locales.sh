#! /usr/bin/env -S bash -eE -u -o pipefail -O inherit_errexit

# shellcheck disable=SC2154

if [[ ${EUID} -ne 0 ]]; then
  echo "ERROR: This script needs to run with superuser privileges" >&2
  exit 1
fi

if [[ ${#} -gt 1 ]]; then
  echo "ERROR: More than one argument is not supported" >&1
  exit 1
fi

NEW_LOCALES=()
readonly LOCALES=(
  LANG
  LC_ADDRESS
  LC_NAME
  LC_MONETARY
  LC_PAPER
  LC_IDENTIFICATION
  LC_TELEPHONE
  LC_MEASUREMENT
  LC_TIME
  LC_NUMERIC
)

for REQUIRED_COMMAND in 'locale-gen' 'update-locale'; do
  if ! command -v "${REQUIRED_COMMAND}" &>/dev/null; then
    echo "ERROR The command '${REQUIRED_COMMAND}' is not available - is the package 'locales' installed?" >&1
    exit 1
  fi
done
unset REQUIRED_COMMAND

function uncomment_in_locale_gen() {
  sed --in-place --regexp-extended "s/# (${1}.*)/\1/g" /etc/locale.gen
}

# Comment everything in `/etc/locale.gen`
sed --in-place --regexp-extended 's/^([a-zA-Z]+)/# \1/g' /etc/locale.gen

# Remove old, leftover locale configurations
rm --recursive --force /var/lib/locales/supported.d/*
rm --recursive --force /usr/lib/locale/*

if [[ ${#} -eq 0 ]]; then
  echo "INFO  Reading new locale settings now"
  PREVIOUS_LOCALE='en_US.UTF-8'

  function read_locale() {
    local __LOCALE
    read -r -p "Which locale do you want to use for ${1}? [default=${2}] " '__LOCALE'
    declare -g "__${1}=${__LOCALE:-${2}}"
    declare -g "PREVIOUS_LOCALE=${__LOCALE:-${2}}"
  }

  for __LOCALE in "${LOCALES[@]}"; do
    read_locale "${__LOCALE}" "${PREVIOUS_LOCALE}"
    __NEW_LOCALE="__${__LOCALE}"
    uncomment_in_locale_gen "${!__NEW_LOCALE}"
    __LOCALE=${__LOCALE}=${!__NEW_LOCALE}
    echo "DEBUG Adding '${__LOCALE}' to new locales"
    NEW_LOCALES+=("${__LOCALE}")
	# shellcheck disable=SC2163
    export "${__LOCALE}"
  done
  unset PREVIOUS_LOCALE __NEW_LOCALE

  echo "INFO  Setting LANGUAGE and LC_CTYPE to the value of LANG (${__LANG})"
  export "LANGUAGE=${__LANG}" "LC_CTYPE=${__LANG}" "LC_ALL=${__LANG}"
  NEW_LOCALES+=("LANGUAGE=${__LANG}" "LC_CTYPE=${__LANG}" "LC_ALL=${__LANG}")
else
  echo "INFO  Locale will be set to '${1}'"
  uncomment_in_locale_gen "${1}"
  for __LOCALE in "${LOCALES[@]}"; do
    NEW_LOCALES+=("${__LOCALE}=${1}")
  done
  NEW_LOCALES+=("LANGUAGE=${1}" "LC_CTYPE=${1}")
fi

unset __LOCALE

echo "INFO New locales: ${NEW_LOCALES[*]}"

echo "INFO  Generating new locales"
locale-gen --purge

echo "INFO  Updating new locales"
update-locale --reset "${NEW_LOCALES[@]}"

echo "INFO  Here are the contents of /etc/default/locale"
echo ''
cat /etc/default/locale

echo ''
echo "INFO  You may need to run 'sudo dpkg-reconfigure locales' manaully again"
