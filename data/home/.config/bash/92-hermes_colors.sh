#! /usr/bin/env bash

# ! Customize hermes' colors
#   Sourced by "${HOME}/.config/bash/90-hermes.sh" when sourced with '--colors'

if __evaluates_to_true HERMES_COLOR_BAT && __is_command 'bat'; then
  export BAT_STYLE=plain
  export BAT_THEME_DARK=evergruv-dark
  export BAT_THEME_LIGHT=evergruv-light
fi

if __evaluates_to_true HERMES_COLOR_BTOP && __is_command btop; then
  function __hermes__set_theme_btop() {
    local CONFIG_FILE=${HOME}/.config/btop/btop.conf
    if [[ -f ${CONFIG_FILE} ]]; then
      sed --in-place --regexp-extended \
        "s/^(color_theme =).*/\1 \"evergruv-${__HERMES__THEME_VARIANT:?}\"/" \
        "${CONFIG_FILE}"
    fi
  }
fi

if __evaluates_to_true HERMES_COLOR_GITUI && __is_command gitui; then
  function __hermes__set_theme_gitui() {
    local THEME_FILE=${HOME}/.config/gitui/evergruv-${__HERMES__THEME_VARIANT:?}.ron
    if [[ -f ${THEME_FILE} ]]; then
      ln --symbol --force "${THEME_FILE}" "${HOME}/.config/gitui/theme.ron"
    fi
  }
fi

if __evaluates_to_true HERMES_COLOR_FLYLINE && [[ -s ${HOME}/.local/lib/libflyline.so ]]; then
  eval "$(dircolors || :)" # LS_COLORS for coloring completions

  if [[ ! -v __HERMES__FLYLINE_BASE_COLORS ]]; then
    readonly __HERMES__FLYLINE_BASE_COLORS=(
      recognised-command='green'
      unrecognised-command='bold red'
      single-quoted-text='yellow'
      double-quoted-text='yellow'
      inline-suggestion='cyan'
      key-sequence-style='red'
      opening-and-closing-pair='magenta'
      rainbow-bracket1='blue'
      rainbow-bracket2='dim blue'
      rainbow-bracket3='purple'
      rainbow-bracket4='dim purple'
    )
  fi

  function __hermes__set_theme_flyline() {
    if [[ ${__HERMES__THEME_VARIANT:?} =~ ^l(ight)?$ ]]; then
      flyline set-style "${__HERMES__FLYLINE_BASE_COLORS[@]}" \
        normal-text='dim black' secondary-text='black'
    elif [[ ${__HERMES__THEME_VARIANT} =~ ^d(ark)?$ ]]; then
      flyline set-style "${__HERMES__FLYLINE_BASE_COLORS[@]}" \
        normal-text='white' secondary-text='dim white'
    fi
  }
fi

if __evaluates_to_true HERMES_COLOR_FZF && __is_command fzf; then
  [[ -v __HERMES__FZF_DEFAULT_OPTS ]] || export __HERMES__FZF_DEFAULT_OPTS=${FZF_DEFAULT_OPTS:-}

  # shellcheck disable=SC2329
  function __hermes__set_theme_fzf() {
    if [[ ${__HERMES__THEME_VARIANT:?} =~ ^l(ight)?$ ]]; then
      export FZF_DEFAULT_OPTS="--color=light --color=fg:#5C6A72,bg:#F5F5F2,hl:#8DA101 --color=fg+:#5C6A72,bg+:#EBEBE4,hl+:#8DA101 --color=info:#D69A00,prompt:#8DA101,pointer:#3A94C5 --color=marker:#8DA101,spinner:#35A77C,header:#8DA101 --color=border:#EBEBE4,gutter:#F5F5F2,query:#5C6A72 --color=scrollbar:#8DA101,separator:#EBEBE4${__HERMES__FZF_DEFAULT_OPTS+ ${__HERMES__FZF_DEFAULT_OPTS}}"
    elif [[ ${__HERMES__THEME_VARIANT} =~ ^d(ark)?$ ]]; then
      export FZF_DEFAULT_OPTS="--color=dark --color=fg:#DDC7A1,bg:#1D2021,hl:#A9B665 --color=fg+:#DDC7A1,bg+:#141617,hl+:#A9B665 --color=info:#D8A657,prompt:#A9B665,pointer:#7DAEA3 --color=marker:#A9B665,spinner:#89B482,header:#A9B665 --color=border:#141617,gutter:#1D2021,query:#DDC7A1 --color=scrollbar:#A9B665,separator:#141617${__HERMES__FZF_DEFAULT_OPTS+ ${__HERMES__FZF_DEFAULT_OPTS}}"
    fi
  }
fi

function __hermes__setup_signal_handlers() {
  # When hermes is used in conjunction with https://github.com/georglauterbach/desktop,
  # theme variant changes to certain CLI utilities (that cannot be updated in another way)
  # can be applied with the `theme` script; for this to work properly, we need to handle
  # the 'SIGUSR2' signal.
  function __hermes__signal_handler_sigusr2() {
    local __HERMES__THEME_VARIANT
    __HERMES__THEME_VARIANT=$(gsettings get org.gnome.desktop.interface color-scheme 2>/dev/null || :)

    if   [[ ${__HERMES__THEME_VARIANT} == "'prefer-light'" ]]; then __HERMES__THEME_VARIANT=light
    elif [[ ${__HERMES__THEME_VARIANT} == "'prefer-dark'" ]];  then __HERMES__THEME_VARIANT=dark
    else return 0
    fi

    __hermes__set_theme_btop
    __hermes__set_theme_gitui
    __evaluates_to_true HERMES_INIT_FLYLINE && __hermes__set_theme_flyline
    __evaluates_to_true HERMES_INIT_FZF     && __hermes__set_theme_fzf
  }

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

  trap __hermes__signal_handler_sigusr2 SIGUSR2
  trap __hermes__signal_handler_exit    EXIT

  { flock -x 3 ; echo "${$}" >&3 ; } 3>>/tmp/.hermes_shells_to_update
  __hermes__signal_handler_sigusr2
}
