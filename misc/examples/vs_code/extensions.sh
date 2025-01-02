#! /usr/bin/env -S bash -eE -u -o pipefail -O inherit_errexit

readonoly EXTENSIONS=(
  'aaron-bond.better-comments'
  'adpyke.codesnap'
  'asciidoctor.asciidoctor-vscode'
  'editorconfig.editorconfig'
  'maattdd.gitless'
  'mads-hartmann.bash-ide-vscode'
  'ms-vscode-remote.remote-containers'
  'ms-vscode-remote.remote-ssh'
  'ms-vscode-remote.remote-ssh-edit'
  'ms-vscode.remote-explorer'
  'pkief.material-icon-theme'
  'redhat.vscode-yaml'
  'sainnhe.gruvbox-material'
  'streetsidesoftware.code-spell-checker'
  'timonwong.shellcheck'
  'usernamehw.errorlens'
  'xshrim.txt-syntax'
)

for EXTENSION in "${EXTENSIONS[@]}"; do
  code --install-extension "${EXTENSION}"
done

