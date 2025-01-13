# ██████╗ ██╗     ███████╗   ███████╗██╗  ██╗
# ██╔══██╗██║     ██╔════╝   ██╔════╝██║  ██║
# ██████╔╝██║     █████╗     ███████╗███████║
# ██╔══██╗██║     ██╔══╝     ╚════██║██╔══██║       what  line editor written in pure Bash
# ██████╔╝███████╗███████╗██╗███████║██║  ██║    used by  shell
# ╚═════╝ ╚══════╝╚══════╝╚═╝╚══════╝╚═╝  ╚═╝       link  https://github.com/akinomyoga/ble.sh

# shellcheck disable=SC2016,SC2034

# -----------------------------------------------
# ----  Integrations / Contrib  -----------------
# -----------------------------------------------

_ble_contrib_fzf_base="${HOME}/.config/fzf"

# -----------------------------------------------
# ----  Completion  -----------------------------
# -----------------------------------------------

bleopt complete_auto_complete=yes # enable auto-complete
bleopt complete_auto_history=     # disable auto-complete based on the command history
bleopt complete_ambiguous=        # disable ambiguous completion
bleopt complete_menu_complete=yes # enable menu-complete by TAB

bleopt complete_menu_filter=
bleopt complete_menu_style=align-nowrap
bleopt complete_menu_maxlines=3

# -----------------------------------------------
# ----  Key Bindings  ---------------------------
# -----------------------------------------------

# Make CTRL+BACKSPACE delete a whole word (in Alacritty)
ble-bind -f 'M-C-?' kill-backward-cword

# -----------------------------------------------
# ----  Theming  --------------------------------
# -----------------------------------------------

bleopt highlight_syntax=yes

bleopt term_index_colors=16
bleopt term_true_colors=semicolon

# -----------------------------------------------
# ----  Miscellaneous  --------------------------
# -----------------------------------------------

# character set & encoding
bleopt char_width_mode=west
bleopt input_encoding=UTF-8

# transient prompt
bleopt prompt_ps1_final=
bleopt prompt_ps1_transient=same-dir:trim

# disable writing history to file
bleopt history_share=

# bells
bleopt edit_abell=
bleopt edit_vbell=

# maximum line length
bleopt line_limit_type=none

# disable marker
bleopt prompt_eol_mark=''
bleopt exec_errexit_mark=''
bleopt exec_elapsed_mark=''
