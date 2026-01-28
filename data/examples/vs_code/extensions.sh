#! /usr/bin/env bash

set -eE -u -o pipefail
shopt -s inherit_errexit

readonly EXTENSIONS=(
  'asciidoctor.asciidoctor-vscode'
  'editorconfig.editorconfig'
  'edwinhuish.better-comments-next'
  'mads-hartmann.bash-ide-vscode'
  'ms-vscode-remote.remote-containers'
  'ms-vscode-remote.remote-ssh'
  'ms-vscode-remote.remote-ssh-edit'
  'ms-vscode.remote-explorer'
  'streetsidesoftware.code-spell-checker'
  'timonwong.shellcheck'
  'usernamehw.errorlens'

  'georglauterbach.evergruv'
  'navernoedenis.gruvbox-material-icons'
)

for EXTENSION in "${EXTENSIONS[@]}"; do
  code --install-extension "${EXTENSION}"
done
