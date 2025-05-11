#! /usr/bin/env bash

set -eE -u -o pipefail
shopt -s inherit_errexit

readonly EXTENSIONS=(
  'aaron-bond.better-comments'
  'asciidoctor.asciidoctor-vscode'
  'editorconfig.editorconfig'
  'jonathanharty.gruvbox-material-icon-theme'
  'maattdd.gitless'
  'mads-hartmann.bash-ide-vscode'
  'ms-vscode-remote.remote-containers'
  'ms-vscode-remote.remote-ssh'
  'ms-vscode-remote.remote-ssh-edit'
  'ms-vscode.remote-explorer'
  'ms-vsliveshare.vsliveshare'
  'sainnhe.everforest'
  'sainnhe.gruvbox-material'
  'streetsidesoftware.code-spell-checker'
  'timonwong.shellcheck'
  'usernamehw.errorlens'
  'xshrim.txt-syntax'
)

for EXTENSION in "${EXTENSIONS[@]}"; do
  code --install-extension "${EXTENSION}"
done
