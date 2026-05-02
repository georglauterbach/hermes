-- ! neovim configuration
-- ! suitable for neovim >= 12.0.0
--
-- REF https://youtu.be/lljs_7xB7Ps?si=4JjeonT_3daPzB1h
-- REF https://youtu.be/yI9R13h9IEE?si=y_po8LydY3ZYgaVm
--
-- cSpell: enable
-- cSpell: disable nvim

-- ? version check

local version = vim.version()
if version.major ~= 0 or version.minor < 12 then
  error("neovim version unsupported")
end

-- ? editor

-- * editor::indentation
vim.opt.autoindent  = true -- copy indent from current line
vim.opt.smartindent = true -- smart auto-indent
vim.opt.shiftwidth  = 2    -- indent width
vim.opt.tabstop     = 2    -- tab width
vim.opt.softtabstop = 2    -- soft tab stop not tabs on tab/backspace
vim.opt.expandtab   = true -- use spaces instead of tabs

-- * editor::file handling
vim.opt.autoread    = true  -- auto-reload changes if outside of neovim
vim.opt.autowrite   = false -- do not auto-save
vim.opt.backup      = false -- do not create a backup file
vim.opt.writebackup = false -- do not write to a backup file
vim.opt.swapfile    = false -- do not create a swapfile

local undo_dir      = vim.fn.expand("~/.cache/nvim/undo_dir")
if vim.fn.isdirectory(undo_dir) == 0 then
  vim.fn.mkdir(undo_dir, "p")
end

vim.opt.undodir        = undo_dir -- set the undo directory
vim.opt.undofile       = true     -- do create an undo file

-- * editor::signcolumn
vim.opt.signcolumn     = "yes"    -- always show a sign column
vim.opt.fillchars      = { eob = " " } -- hide "~" on empty lines

-- * editor::lines & current line
vim.opt.number         = true        -- enable line numbers
vim.opt.relativenumber = false       -- disable relative line numbers
vim.opt.cursorline     = true        -- highlight the current line
vim.opt.scrolloff      = 10          -- keep 10 lines above and below the cursor
vim.opt.sidescrolloff  = 10          -- keep 10 lines to the left & right of cursor
vim.opt.wrap           = false       -- do not wrap lines by default
vim.opt.selection      = "inclusive" -- include last char in selection

-- * editor::search
vim.opt.hlsearch       = true -- highlight search matches
vim.opt.incsearch      = true -- show matches as you type
vim.opt.ignorecase     = true -- case insensitive search
vim.opt.smartcase      = true -- case sensitive if uppercase in string

-- * editor::brackets
vim.opt.showmatch      = true -- highlights matching brackets

-- ? system

vim.opt.encoding       = "utf-8" -- set encoding

-- * system::performance
vim.opt.lazyredraw     = true  -- do not redraw during macros
vim.opt.synmaxcol      = 300   -- syntax highlighting limit
vim.opt.timeoutlen     = 500   -- timeout duration
vim.opt.ttimeoutlen    = 50    -- key code timeout
vim.opt.updatetime     = 300   -- faster completion
vim.opt.redrawtime     = 10000 -- increase neovim redraw tolerance
vim.opt.maxmempattern  = 20000 -- increase max memory

-- * system::clipboard
vim.opt.clipboard:append("unnamedplus") -- use system clipboard

-- ? plugins

vim.pack.add({
  -- ? status bar
  { src = 'https://github.com/nvim-lualine/lualine.nvim' },
  -- ? LSP
  -- configurations for various language servers
  { src = 'https://github.com/neovim/nvim-lspconfig' },
  -- installer for language servers
  { src = 'https://github.com/mason-org/mason.nvim' },
  -- bridge between mason and lspconfig
  { src = 'https://github.com/mason-org/mason-lspconfig.nvim' },
  -- improve mason-lspconfig to install any tool, not just language servers
  { src = 'https://github.com/WhoIsSethDaniel/mason-tool-installer.nvim' }
})

-- ? LSP configuration

require("mason").setup()
require("mason-lspconfig").setup()
require("mason-tool-installer").setup({
  ensure_installed = { "lua_ls", "rust-analyzer" }
})

-- ? theme

vim.cmd("set shortmess+=I") -- disable startup screen

-- make UI components transparent
local function set_transparent()
  local groups = {
    "Normal",
    "NormalNC",
    "EndOfBuffer",
    "NormalFloat",
    "FloatBorder",
    "SignColumn",
    "StatusLine",
    "StatusLineNC",
    "TabLine",
    "TabLineFill",
    "TabLineSel",
    "ColorColumn",
  }
  for _, g in ipairs(groups) do
    vim.api.nvim_set_hl(0, g, { bg = "none" })
  end
  vim.api.nvim_set_hl(
    0,
    "TabLineFill",
    { bg = "none", fg = "#767676" })
end

--set_transparent()

-- ? status line

require('lualine').setup {
  options = {
    icons_enabled = false,
    theme = 'ayu',
    component_separators = { left = ' ', right = ' ' },
    section_separators = { left = ' ', right = ' ' }
  },
  sections = {
    lualine_a = { 'mode' },
    lualine_b = { 'diagnostics' },
    lualine_c = { 'filename' },
    lualine_x = { 'location' },
    lualine_y = { 'encoding', 'fileformat' },
    lualine_z = { 'filetype', 'lsp_status' }
  }
}

vim.opt.cmdheight = 0     -- single line command line
vim.opt.showmode  = false -- do not show the mode, instead have it in statusline
