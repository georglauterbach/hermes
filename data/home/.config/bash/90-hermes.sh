#! /usr/bin/env bash

# ! USE ${HOME}/.config/bash/91-hermes_settings.sh
#   FOR ADJUSTING HOW HERMES SETS UP YOUR TERMINAL

[[ ${-} == *i* ]] || return 0

function __is_command() { command -v "${1:?}" &>/dev/null ; }
function __evaluates_to_true() { [[ -v ${1:?} ]] && [[ ${!1,,} == 'true' ]] ; }
function __call_and_unset() { "${1:?}" "${@:2}" ; unset "${1}" ; }

function __hermes__setup_variables() {
  local SEGMENT
  PATH=${PATH:-'/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin'}

  # shellcheck disable=SC2066
  for SEGMENT in "${HOME}/.local/bin"; do
    [[ -d ${SEGMENT} ]] && [[ ${PATH} != *${SEGMENT}* ]] && PATH="${SEGMENT}:${PATH}"
  done

  # shellcheck disable=SC2066
  for SEGMENT in "${HOME}/.cargo/env"; do
    # shellcheck source=/dev/null
    [[ -s ${SEGMENT} ]] && [[ -r ${SEGMENT} ]] && source "${SEGMENT}"
  done

  XDG_CONFIG_HOME=${XDG_CONFIG_HOME:-${HOME}/.config}
  XDG_CACHE_HOME=${XDG_CACHE_HOME:-${HOME}/.cache}
  XDG_DATA_HOME=${XDG_DATA_HOME:-${HOME}/.local/share}
  XDG_STATE_HOME=${XDG_STATE_HOME:-${HOME}/.local/state}

  if [[ ! -v VISUAL ]]; then
    if   __is_command nvim ; then VISUAL='nvim'
    elif __is_command vim  ; then VISUAL='vim'
    elif __is_command nano ; then VISUAL='nano'
    else VISUAL=''
    fi
  fi

  EDITOR=${EDITOR:-${VISUAL}}
  __is_command less && export PAGER=${PAGER:-$(command -v less)}

  if [[ ! -v LANG ]]; then
    # shellcheck source=/dev/null
    [[ -r /etc/locale.conf ]] && source /etc/locale.conf
    LANG=${LANG:-C.UTF-8} LANGUAGE=${LANGUAGE:-${LANG}} LC_ALL=${LC_ALL:-${LANG}}
  fi

  export PATH VISUAL EDITOR LANG LANGUAGE LC_ALL
  export XDG_CONFIG_HOME XDG_CACHE_HOME XDG_DATA_HOME XDG_STATE_HOME
}

function __hermes__setup_completion() {
  shopt -oq posix && return 0

  # shellcheck source=/dev/null
  [[ -r ${XDG_CONFIG_HOME}/bash_completion ]] && source "${XDG_CONFIG_HOME}/bash_completion"
  # shellcheck source=/dev/null
  [[ -r /usr/share/bash-completion/bash_completion ]] && source /usr/share/bash-completion/bash_completion
}

function __hermes__setup_prompt() {
  if __evaluates_to_true HERMES_INIT_STARSHIP && __is_command starship; then
    export STARSHIP_CONFIG=${STARSHIP_CONFIG:-${HOME}/.config/starship/starship.toml}
    # shellcheck source=/dev/null
    source <(starship init bash)

    # We disable PS0 and preexec hooks because they mess with trapping signals like SIGUSR2
    unset -f starship_preexec_ps0 2>/dev/null
    export PS0=''

    # shellcheck disable=SC2016
    __evaluates_to_true HERMES_INIT_FLYLINE && export PS1_FINAL='$(starship module directory) ➜ '
  else
    export PROMPT_DIRTRIM=4 PS2='' PS4='> '
  fi

  if __evaluates_to_true HERMES_INIT_FLYLINE && [[ -s ${HOME}/.local/lib/libflyline.so ]]; then
    command -v flyline &>/dev/null || enable -f "${HOME}/.local/lib/libflyline.so" flyline

    flyline editor --auto-close-chars false
    flyline editor --show-inline-history true
    flyline history --backend flyline --jsonl-path "${XDG_CACHE_HOME}/bash_history.jsonl"
    flyline mouse --mode disabled
    flyline set-style selected-text=reverse
    flyline suggestions set-fuzzy-mode none

    flyline key --clear-defaults
    flyline key bind AnyChar          always=insertChar
    flyline key bind Shift+AnyChar    always=insertChar

    flyline key bind Down             always=nextHistoryEntry
    flyline key bind Down             !cursorOnFinalLine=moveLineDown
    flyline key bind Shift+Down       always=moveLineDownExtendSelection

    flyline key bind Right            always=moveRight
    flyline key bind Shift+Right      always=moveRightExtendSelection
    flyline key bind Ctrl+Right       always=moveRightOneWordPart
    flyline key bind Ctrl+Shift+Right always=moveRightOneWordPartExtendSelection
    flyline key bind Alt+Right        always=moveRightOneWord
    flyline key bind Alt+Shift+Right  always=moveRightOneWordExtendSelection
    flyline key bind Right \
      inlineSuggestionAvailable+cursorAtEnd+!tabCompletionMultiColAvailable=inlineSuggestionAccept

    flyline key bind Up               always=prevHistoryEntry
    flyline key bind Up               !cursorOnFirstLine=moveLineUp
    flyline key bind Shift+Up         always=moveLineUpExtendSelection

    flyline key bind Left             always=moveLeft
    flyline key bind Shift+Left       always=moveLeftExtendSelection
    flyline key bind Ctrl+Left        always=moveLeftOneWordPart
    flyline key bind Ctrl+Shift+Left  always=moveLeftOneWordPartExtendSelection
    flyline key bind Alt+Left         always=moveLeftOneWord
    flyline key bind Alt+Shift+Left   always=moveLeftOneWordExtendSelection

    flyline key bind Home             always=moveLeftStartOfLine
    flyline key bind End              always=moveRightEndOfLine

    flyline key bind Backspace        always=deleteLeft
    flyline key bind Ctrl+Backspace   always=deleteLeftOneWordPart
    flyline key bind Alt+Backspace    always=deleteLeftOneWord

    flyline key bind Delete           always=deleteRight
    flyline key bind Ctrl+Delete      always=deleteRightOneWordPart
    flyline key bind Alt+Delete       always=deleteRightOneWord

    flyline key bind Ctrl+l           always=clearScreen
    flyline key bind Ctrl+d           bufferIsEmpty=exit
    flyline key bind Ctrl+c           always=cancel
    flyline key bind Ctrl+c           textSelected=copyTarget
    flyline key bind Ctrl+v           always=pasteSystemClipboard

    flyline key bind Esc              always=toggleMouse
    flyline key bind Esc              textSelected=escapeToNormalMode
    flyline key bind Esc              tabCompletionWaiting=escapeToNormalMode
    flyline key bind Esc              tabCompletion=escapeToNormalMode
    flyline key bind Esc              tabCompletionAvailable=escapeToNormalMode

    flyline key bind Tab              always=runTabCompletion
    flyline key bind Tab              tabCompletionAvailable=tabCompletionNextSuggestion
    flyline key bind Tab              tabCompletionOneResult=tabCompletionAcceptEntry
    flyline key bind Shift+Tab        tabCompletionAvailable=tabCompletionPrevSuggestion
    flyline key bind Shift+BackTab    tabCompletionAvailable=tabCompletionPrevSuggestion

    flyline key bind Enter            always=submitOrNewline
    flyline key bind Enter            multilineBuffer=insertNewline
    flyline key bind Enter            multilineBuffer+cursorAtEndTrimmed=submitOrNewline
    flyline key bind Enter            tabCompletionEntrySelected=tabCompletionAcceptEntry
  fi
}

function __hermes__setup_history() {
  if __evaluates_to_true HERMES_INIT_STINKPOT && __is_command stinkpot; then
    export HISTFILE=${HISTFILE:-/dev/null} HISTCONTROL=${HISTCONTROL:-ignoreboth}
    # shellcheck source=/dev/null
    source <(stinkpot init bash)
    __is_command flyline && flyline key bind Ctrl+r 'always=runBashCommand(__stinkpot_search)'
  else
    HISTFILE=${HISTFILE:-${XDG_CACHE_HOME}/bash/history.txt}
    HISTCONTROL=${HISTCONTROL:-ignoreboth}
    HISTSIZE=${HISTSIZE:-10000}
    HISTFILESIZE=${HISTFILESIZE:-10000}
    shopt -s histappend
    mkdir -p "${HISTFILE%/*}" &>/dev/null || :
    export HISTFILE HISTCONTROL HISTSIZE HISTFILESIZE
  fi
}

function __hermes__setup_programs() {
  if __evaluates_to_true HERMES_INIT_BAT && __is_command bat; then
    export BAT_STYLE=plain
    if [[ -v PAGER ]]; then
      export MANPAGER="sh -c 'awk '\''{ gsub(/\x1B\[[0-9;]*m/, \"\", \$0); gsub(/.\x08/, \"\", \$0); print }'\'' | bat --plain --language=man'"
      export MANROFFOPT='-c'
    fi
  fi

  # shellcheck source=/dev/null
  __evaluates_to_true HERMES_INIT_FZF && __is_command fzf && source <(fzf --bash)
  # shellcheck source=/dev/null
  __evaluates_to_true HERMES_INIT_ZOXIDE && __is_command zoxide && source <(zoxide init bash)
}

function __hermes__setup_overrides() {
  # The checks on `[[ -t 0 ]]` guard against missing stdin, which
  # is a problem in agent sessions because it leads to hanging.

  if __evaluates_to_true HERMES_OVERRIDE_CAT_WITH_BAT && __is_command bat; then
    # shellcheck disable=SC2329
    function cat() {
      if [[ -t 0 ]]; then command bat --paging=never "${@}"; else command cat "${@}"; fi
    }
  fi

  __evaluates_to_true HERMES_OVERRIDE_CD_WITH_ZOXIDE && __is_command zoxide && alias cd='z'

  if __evaluates_to_true HERMES_OVERRIDE_DIFF_WITH_DELTA && __is_command delta; then
    # shellcheck disable=SC2329
    function diff() {
      if [[ -t 0 ]]; then command delta "${@}"; else  command diff "${@}"; fi
    }
  fi

  if __evaluates_to_true HERMES_OVERRIDE_LESS_WITH_BAT && __is_command bat; then
    # shellcheck disable=SC2329
    function less() {
      if [[ -t 0 ]]; then command bat --paging=always "${@}"; else command less "${@}"; fi
    }
  fi

  if __evaluates_to_true HERMES_OVERRIDE_LS_WITH_EZA && __is_command eza; then
    # shellcheck disable=SC2329
    function ls() {
      if [[ -t 0 ]]; then
        command eza --long --binary --group --classify --extended --group-directories-first "${@}"
      else
        command ls "${@}"
      fi
    }
  fi

  if __evaluates_to_true HERMES_OVERRIDE_Y_WITH_YAZI && __is_command yazi; then
    # shellcheck disable=SC2329
    function y() {
      local YAZI_DIR_FILE YAZI_DIR
      YAZI_DIR_FILE="$(mktemp -t ".yazi_dir_XXXXXX")"

      yazi "${@}" --cwd-file="${YAZI_DIR_FILE}"
      YAZI_DIR="$(<"${YAZI_DIR_FILE}")"

      if [[ -n ${YAZI_DIR} ]] && [[ ${YAZI_DIR} != "${PWD}" ]]; then
        builtin cd -- "${YAZI_DIR}" || { rm --force -- "${YAZI_DIR_FILE}" ; return 1 ; }
      fi
      rm --force -- "${YAZI_DIR_FILE}"
    }
  fi
}

function __hermes__setup_aliases() {
  alias gcs='git commit --signoff --gpg-sign'
  alias gf='git fetch --prune --tags --force'

  # shellcheck disable=SC2139
  [[ -n ${EDITOR} ]] && alias v="${EDITOR}" sv="sudo $(command -v "${EDITOR}")"

  alias lsa='ls -A'
  alias ...='cd ../..'
  alias ....='cd ../../..'
  alias .....='cd ../../../..'
  alias ......='cd ../../../../..'
  alias .......='cd ../../../../../..'
}

function __hermes__setup_signal_handlers() {
  # To perform cleanup of the current shell's PID, we run a dedicated script when we
  # receive the 'EXIT' signal. Not doing this is not an issue because the `theme` script
  # (https://github.com/georglauterbach/desktop) will also clean up - having this handler
  # is simply an optimization.
  # shellcheck disable=SC2329
  function __hermes__signal_handler_exit() {
    [[ -r /tmp/.hermes_shells_to_update ]] || return 0

    # We ignore SC2094 because we fully read the file first and overwrite it only afterward.
    # shellcheck disable=SC2094
    {
      flock -x 3
      grep -v ${$} <&3 >/tmp/.hermes_shells_to_update_exit || :
      mv /tmp/.hermes_shells_to_update_exit /tmp/.hermes_shells_to_update
    } 3</tmp/.hermes_shells_to_update
  }

  __HERMES__SIGNAL_HANDLERS_SIGUSR2=()

  trap 'hermes_switch_theme --force' SIGUSR2
  trap __hermes__signal_handler_exit EXIT

  { flock -x 3 ; echo "${$}" >&3 ; } 3>>/tmp/.hermes_shells_to_update
}

function __hermes__setup_theme() {
  if __evaluates_to_true HERMES_OVERRIDE_COLORS_BAT && __is_command bat; then
    export BAT_THEME_DARK=evergruv-dark
    export BAT_THEME_LIGHT=evergruv-light
  fi

  if __evaluates_to_true HERMES_OVERRIDE_COLORS_BTOP && __is_command btop; then
    # shellcheck disable=SC2329
    function __hermes__set_theme_btop() {
      local CONFIG_FILE=${XDG_CONFIG_HOME}/btop/btop.conf
      if [[ -f ${CONFIG_FILE} ]]; then
        sed --in-place --regexp-extended \
          "s/^(color_theme =).*/\1 \"evergruv-${HERMES_THEME_VARIANT:?}\"/" \
          "${CONFIG_FILE}"
      fi
    }
    __HERMES__SIGNAL_HANDLERS_SIGUSR2+=(__hermes__set_theme_btop)
  fi

  if __evaluates_to_true HERMES_OVERRIDE_COLORS_EZA && __is_command eza; then
    # shellcheck disable=SC2329
    function __hermes__set_theme_eza() {
      local CONFIG_DIR=${XDG_CONFIG_HOME}/eza
      local THEME_FILE=themes/${HERMES_THEME_VARIANT:?}.yaml
      if [[ -f ${CONFIG_DIR}/${THEME_FILE} ]]; then
        ln --symbolic --force "${THEME_FILE}" "${CONFIG_DIR}/theme.yaml"
      fi
    }
    __HERMES__SIGNAL_HANDLERS_SIGUSR2+=(__hermes__set_theme_eza)
  fi

  if __evaluates_to_true HERMES_OVERRIDE_COLORS_FLYLINE && [[ -s ${HOME}/.local/lib/libflyline.so ]]; then
    eval "$(dircolors || :)" # LS_COLORS for coloring completions

    __HERMES__FLYLINE_BASE_COLORS=(
      recognised-command='green'
      unrecognised-command='bold red'
      single-quoted-text='yellow'
      double-quoted-text='yellow'
      inline-suggestion='cyan'
      key-sequence-style='bold cyan'
      opening-and-closing-pair='magenta'
      rainbow-bracket1='yellow'
      rainbow-bracket2='green'
      rainbow-bracket3='cyan'
      rainbow-bracket4='blue'
      matching-char='green'
      selected-text='blue'
    )

    function __hermes__set_theme_flyline() {
      __is_command flyline || return 0
      if [[ ${HERMES_THEME_VARIANT:?} == light ]]; then
        flyline set-style --default-theme "${HERMES_THEME_VARIANT}" \
          "${__HERMES__FLYLINE_BASE_COLORS[@]}" normal-text=black secondary-text=
      elif [[ ${HERMES_THEME_VARIANT} == dark ]]; then
        flyline set-style --default-theme "${HERMES_THEME_VARIANT}" \
          "${__HERMES__FLYLINE_BASE_COLORS[@]}" normal-text= secondary-text=white
      fi
      flyline set-cursor --backend flyline --effect fade --style "${__HERMES__COLOR_BLUE}"
    }
    __HERMES__SIGNAL_HANDLERS_SIGUSR2+=(__hermes__set_theme_flyline)
    __hermes__set_theme_flyline
  fi

  if __evaluates_to_true HERMES_OVERRIDE_COLORS_FZF && __is_command fzf; then
    [[ -v __HERMES__FZF_DEFAULT_OPTS ]] || __HERMES__FZF_DEFAULT_OPTS=${FZF_DEFAULT_OPTS:-}

    function __hermes__set_theme_fzf() {
      export FZF_DEFAULT_OPTS="--color=${HERMES_THEME_VARIANT} --color=fg:${__HERMES__COLOR_FOREGROUND},bg:${__HERMES__COLOR_BACKGROUND},hl:${__HERMES__COLOR_GREEN} --color=info:${__HERMES__COLOR_YELLOW},prompt:${__HERMES__COLOR_GREEN},pointer:${__HERMES__COLOR_BLUE} --color=marker:${__HERMES__COLOR_GREEN},spinner:${__HERMES__COLOR_CYAN},header:${__HERMES__COLOR_GREEN}"
      if [[ ${HERMES_THEME_VARIANT:?} == light ]]; then
        FZF_DEFAULT_OPTS+=" --color=fg+:${__HERMES__COLOR_FOREGROUND},bg+:${__HERMES__COLOR_BRIGHT_WHITE},hl+:${__HERMES__COLOR_GREEN}"
        FZF_DEFAULT_OPTS+=" --color=border:${__HERMES__COLOR_BRIGHT_WHITE},gutter:${__HERMES__COLOR_BACKGROUND},query:${__HERMES__COLOR_FOREGROUND}"
        FZF_DEFAULT_OPTS+=" --color=scrollbar:${__HERMES__COLOR_GREEN},separator:${__HERMES__COLOR_BRIGHT_WHITE}"
      elif [[ ${HERMES_THEME_VARIANT} == dark ]]; then
        FZF_DEFAULT_OPTS+=" --color=fg+:${__HERMES__COLOR_FOREGROUND},bg+:${__HERMES__COLOR_BLACK},hl+:${__HERMES__COLOR_GREEN}"
        FZF_DEFAULT_OPTS+=" --color=border:${__HERMES__COLOR_BLACK},gutter:${__HERMES__COLOR_BACKGROUND},query:${__HERMES__COLOR_FOREGROUND}"
        FZF_DEFAULT_OPTS+=" --color=scrollbar:${__HERMES__COLOR_GREEN},separator:${__HERMES__COLOR_BLACK}"
      fi
      FZF_DEFAULT_OPTS+=${__HERMES__FZF_DEFAULT_OPTS+ ${__HERMES__FZF_DEFAULT_OPTS}}
    }
    __HERMES__SIGNAL_HANDLERS_SIGUSR2+=(__hermes__set_theme_fzf)
    __hermes__set_theme_fzf
  fi

  if __evaluates_to_true HERMES_OVERRIDE_COLORS_GITUI && __is_command gitui; then
    # shellcheck disable=SC2329
    function __hermes__set_theme_gitui() {
      local CONFIG_DIR=${XDG_CONFIG_HOME}/gitui
      local THEME_FILE=themes/evergruv-${HERMES_THEME_VARIANT:?}.ron
      if [[ -f ${CONFIG_DIR}/${THEME_FILE} ]]; then
        ln --symbolic --force "${THEME_FILE}" "${CONFIG_DIR}/theme.ron"
      fi
    }
    __HERMES__SIGNAL_HANDLERS_SIGUSR2+=(__hermes__set_theme_gitui)
  fi
}

function __hermes__export_colors() {
  local THEME_VARIANT=${HERMES_THEME_VARIANT:-}

  [[ ${1:-} == dark ]]  && THEME_VARIANT=dark
  [[ ${1:-} == light ]] && THEME_VARIANT=light

  if [[ ${1:-} == --force ]] || [[ -z ${THEME_VARIANT} ]]; then
    if ! __is_command gsettings; then
      echo "Command 'gsettings' not found - using theme 'dark' by default"
      THEME_VARIANT=dark
    else
      THEME_VARIANT=$(gsettings get org.gnome.desktop.interface color-scheme |& tr -d "'" || :)
      if [[ ${THEME_VARIANT} == prefer-light ]]; then
        THEME_VARIANT=light
      elif [[ ${THEME_VARIANT} == prefer-dark ]]; then
        THEME_VARIANT=dark
      elif [[ ${THEME_VARIANT} == default ]]; then
        echo "Theme variant '${THEME_VARIANT}' is treated as 'dark'" >&2
        THEME_VARIANT=dark
      else
        echo "Theme variant '${THEME_VARIANT}' parsed from 'gsettings' unknown and treated as 'dark'" >&2
        THEME_VARIANT=dark
      fi
    fi
  fi

  # ! The color values set in this function are kept in sync with
  #   https://github.com/georglauterbach/desktop/tree/main/data/home/.config/alacritty/themes
  if [[ ${THEME_VARIANT} == light ]]; then
    HERMES_THEME_VARIANT=light
    __HERMES__COLOR_BACKGROUND='#F5F5F2'
    __HERMES__COLOR_FOREGROUND='#5C6A72'
    __HERMES__COLOR_BLACK='#363E42'
    __HERMES__COLOR_RED='#F85552'
    __HERMES__COLOR_GREEN='#8DA101'
    __HERMES__COLOR_YELLOW='#D69A00'
    __HERMES__COLOR_BLUE='#3A94C5'
    __HERMES__COLOR_MAGENTA='#DF69BA'
    __HERMES__COLOR_CYAN='#35A77C'
    __HERMES__COLOR_WHITE='#999997'
    __HERMES__COLOR_BRIGHT_BLACK='#7E919C'
    __HERMES__COLOR_BRIGHT_WHITE='#EBEBE4'
  elif [[ ${THEME_VARIANT} == dark ]]; then
    HERMES_THEME_VARIANT=dark
    __HERMES__COLOR_BACKGROUND='#1D2021'
    __HERMES__COLOR_FOREGROUND='#DDC7A1'
    __HERMES__COLOR_BLACK='#141617'
    __HERMES__COLOR_RED='#EA6962'
    __HERMES__COLOR_GREEN='#A9B665'
    __HERMES__COLOR_YELLOW='#D8A657'
    __HERMES__COLOR_BLUE='#7DAEA3'
    __HERMES__COLOR_MAGENTA='#D3869B'
    __HERMES__COLOR_CYAN='#7BB674'
    __HERMES__COLOR_WHITE='#A39377'
    __HERMES__COLOR_BRIGHT_BLACK='#2B2A29'
    __HERMES__COLOR_BRIGHT_WHITE='#FFE6BA'
  else
    echo "Theme variant '${THEME_VARIANT}' unknown - must be 'dark' or 'light'" >&2
    return 1
  fi

  export HERMES_THEME_VARIANT
}

# Trigger a theme switch for TUI applications
#
# Certain applications (e.g., gitui, btop, etc.) have no (working) automation for
# triggering theme variant switches. Hence, you can either call this function manually
# or, when hermes is used in conjunction with <https://github.com/georglauterbach/desktop>,
# have it called when SIGUSR2 is received.
function hermes_switch_theme() {
  __hermes__export_colors "${@}"

  local HANDLER_FUNCTION
  for HANDLER_FUNCTION in "${__HERMES__SIGNAL_HANDLERS_SIGUSR2[@]}"; do
    "${HANDLER_FUNCTION}"
  done
}

# Show information related to the current setup of hermes
function hermes_debug() {
  local __NAME
  for __NAME in ${!HERMES_*} ${!__HERMES__*}; do
    declare -n __VAL=${__NAME}
    echo "${__NAME}=\"${__VAL[*]}\""
    unset __VAL
  done
}

# Download the latest version of hermes
function hermes_download_latest_version() {
  local HERMES_LOCATION=${HOME}/.local/bin/hermes
  local HERMES_RELEASE_URI_BASE=https://github.com/georglauterbach/hermes/releases
  local HERMES_VERSION

  HERMES_VERSION=$(curl --silent --show-error --fail --location \
    --write-out '%{url_effective}' --output /dev/null \
    "${HERMES_RELEASE_URI_BASE}/latest" | sed 's|.*/||')

  mkdir --parents "${HERMES_LOCATION%/*}"
  curl --silent --show-error --fail --location --output "${HERMES_LOCATION}" \
    "${HERMES_RELEASE_URI_BASE}/download/${HERMES_VERSION}/hermes-${HERMES_VERSION}-$(uname -m)-unknown-linux-musl"

  chmod +x "${HERMES_LOCATION}"
}

function __hermes__main() {
  if [[ -r ${HOME}/.config/bash/91-hermes_settings.sh ]]; then
    # shellcheck source=91-hermes_settings.sh
    source "${HOME}/.config/bash/91-hermes_settings.sh"
  fi

  shopt -s checkwinsize globstar autocd

  local SETUP_FUNCTIONS=(variables completion prompt history programs overrides)
  if __evaluates_to_true HERMES_ENABLE_ADDITIONAL_ALIASES; then
    SETUP_FUNCTIONS+=(aliases)
  else
    unset __hermes__setup_aliases
  fi
  if __evaluates_to_true HERMES_ENABLE_THEMING; then
    __hermes__export_colors
    SETUP_FUNCTIONS+=(signal_handlers theme)
  else
    unset __hermes__{export_colors,setup_{signal_handlers,theme}} hermes_switch_theme
  fi

  local __FUNCTION
  for __FUNCTION in "${SETUP_FUNCTIONS[@]}"; do
    __call_and_unset "__hermes__setup_${__FUNCTION}" || :
  done

  if __evaluates_to_true HERMES_ENABLE_EXPORT_OF_ENVS; then
    # shellcheck disable=SC2086
    export ${!HERMES_*} ${!__HERMES__*}
  fi
}

__call_and_unset __hermes__main "${@}"
