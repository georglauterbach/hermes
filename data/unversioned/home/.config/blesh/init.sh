# version       0.2.0
# sourced by    ble.sh
# task          setup for ble.sh (https://github.com/akinomyoga/ble.sh)

# shellcheck disable=SC2016,SC2034

# -----------------------------------------------
# ----  Integrations / Contrib  -----------------
# -----------------------------------------------

_ble_contrib_fzf_base="${HOME}/.config/fzf"

# -----------------------------------------------
# ----  Features  -------------------------------
# -----------------------------------------------

bleopt highlight_syntax=yes       # enable syntax highlighting
bleopt highlight_filename=yes     # enable highlighting based on filenames
bleopt highlight_variable=yes     # enable highlighting based on variable types

bleopt complete_auto_complete=yes # enable auto-complete
bleopt complete_auto_history=     # disable auto-complete based on the command history
bleopt complete_ambiguous=        # disable ambiguous completion
bleopt complete_menu_complete=yes # enable menu-complete by TAB

bleopt complete_menu_filter=
bleopt complete_menu_style=align-nowrap
bleopt complete_menu_maxlines=3

# disable marker
bleopt prompt_eol_mark=''
bleopt exec_errexit_mark=''
bleopt exec_elapsed_mark=''

# -----------------------------------------------
# ----  Character Set / Encoding  ---------------
# -----------------------------------------------

bleopt char_width_mode=west
bleopt input_encoding=UTF-8

# -----------------------------------------------
# ----  Miscellaneous  --------------------------
# -----------------------------------------------

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

# -----------------------------------------------
# ----  Key Bindings  ---------------------------
# -----------------------------------------------

# Make CTRL+BACKSPACE delete a whole word (in Alacritty)
ble-bind -f 'M-C-?' kill-backward-cword

# -----------------------------------------------
# ----  Theming  --------------------------------
# -----------------------------------------------

# background
bg_dim='#141617'
bg0='#1D2021'
bg1='#282828'
bg3='#3C3836'
# middleground
greyO='#7C6F64'
grey1='#928374'
# foreground
fgO='#D4BE98'
fg1='#DDC7A1'
# normal colors
red='#EA696A'
green='#A9B665'
yellow='#D8A657'
blue='#7DAEA3'
purple='#D3869B'
aqua='#89B482'
orange='#E78A4E'
# dim colors
bg_red='#EA6962'
bg_green='#A9B665'
bg_yellow='#D8A657'
bg_blue='#659F92'
bg_purple='C86580'
bg_aqua='#6BA163'
bg_orange='#E16F23'

ble-face "syntax_default=fg=${fg1}"              # default color
ble-face "disabled=fg=${bg3}"                    # not executed command
ble-face "syntax_comment=fg=${bg3}"              # comment
ble-face "auto_complete=fg=${grey1}"             # auto-completion
ble-face "region_insert=fg=${bg1},bg=${yellow}"  # when tabbing through options

ble-face "syntax_quotation=fg=${purple}"         # quotes
ble-face "syntax_quoted=fg=${aqua}"              # quoted content

ble-face 'varname_unset=fg=#076678'              # declare variable
ble-face 'varname_empty=fg=#076678'              # empty vars
ble-face 'syntax_varname=fg=#458588'             # use non-exported variable
ble-face 'varname_array=fg=#458588,bold'         # array
ble-face 'varname_export=fg=#83a598,bold'        # exported variable
ble-face 'varname_readonly=fg=#83a598,bold'      # readonly variables
ble-face 'syntax_param_expansion=fg=#d79921'     # dollar sign and curly braces
ble-face 'syntax_expr=fg=#b57614'                # [@], [*], etc.

ble-face "command_function=fg=${green}"          # valid command
ble-face "syntax_function_name=fg=${green}"      # function name
ble-face "command_alias=fg=${green}"             # alias
ble-face "syntax_error=bg=${red},fg=${fg1}"      # invalid command
ble-face "command_builtin=fg=${green}"           # builtins
ble-face "command_builtin_dot=fg=${green}"       # :
ble-face "argument_option=fg=${yellow}"          # flags
ble-face "command_keyword=fg=${orange}"          # function, while, for, do, etc.
ble-face "syntax_delimiter=fg=${orange}"         # ;, (), etc.
ble-face "syntax_history_expansion=fg=${orange}" # = !!

ble-face "filename_character=fg=${fg1}"          # normal file names
ble-face "filename_ls_colors=fg=${fg1}"          # normal file names
ble-face "filename_other=fg=${fg1}"              # normal file names
ble-face "filename_warning=fg=${red}"            # warning on overwrite

ble-face "filename_directory=fg=${blue},bold"    # valid directory names
ble-face "command_directory=fg=${blue},bold"
ble-face "filename_link=fg=${aqua},bold,underline"
ble-face "filename_directory_sticky=fg=${blue},bg=${fg1},bold"
