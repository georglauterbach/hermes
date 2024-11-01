#! /usr/bin/env bash

# version       0.4.0
# sourced by    ${HOME}/.bashrc
# task          provides helper and wrapper functions
#               for common tasks and commands

function ls() {
  if __is_command 'eza'; then
    eza --header --long --binary --group --classify --extended --group-directories-first "${@}"
  else
    command -p 'ls' "${@}"
  fi
}

function cat() {
  if __is_command 'batcat'; then
    batcat --theme="gruvbox-dark" --paging=never --style=plain --italic-text=always "${@}"
  else
    command -p 'cat' "${@}"
  fi
}

function apt() {
  local PROGRAM='apt'
  __is_command 'nala' && PROGRAM='nala'

  if [[ ${1:-} =~ ^show|search$ ]]
  then
    command "${PROGRAM}" "${@}"
  else
    __do_as_root "${PROGRAM}" "${@}"
  fi
}

# stolen, ahh adopted from
# https://github.com/casperklein/bash-pack/blob/master/x
function x() {
  [[ -f ${1:-} ]] || { echo "File '${1:-}' not found" >&2 ; return 1 ; }

  case "${1}" in
    ( *.7z )      7za x "${1}"       ;;
    ( *.tar.bz2 ) bzip2 -v -d "${1}" ;;
    ( *.bz2 )     bzip2 -d "${1}"    ;;
    ( *.deb )     ar -x "${1}"       ;;
    ( *.tar.gz )  tar -xvzf "${1}"   ;;
    ( *.gz )      gunzip -d "${1}"   ;;
    ( *.lzh )     lha x "${1}"       ;;
    ( *.tar )     tar -xvf "${1}"    ;;
    ( *.tbz2 )    tar -jxvf "${1}"   ;;
    ( *.tgz )     tar -xvzf "${1}"   ;;
    ( *.xz )      xz -dv "${1}"      ;;
    ( *.zip )     unzip "${1}"       ;;
    ( * )
      echo "Compression type for file '${1}' unknown" >&2
      return 1
      ;;
  esac
}
