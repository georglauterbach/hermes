-- ███╗   ██╗███████╗ ██████╗ ██╗   ██╗██╗███╗   ███╗
-- ████╗  ██║██╔════╝██╔═══██╗██║   ██║██║████╗ ████║
-- ██╔██╗ ██║█████╗  ██║   ██║██║   ██║██║██╔████╔██║
-- ██║╚██╗██║██╔══╝  ██║   ██║╚██╗ ██╔╝██║██║╚██╔╝██║
-- ██║ ╚████║███████╗╚██████╔╝ ╚████╔╝ ██║██║ ╚═╝ ██║    sourced by  nvim
-- ╚═╝  ╚═══╝╚══════╝ ╚═════╝   ╚═══╝  ╚═╝╚═╝     ╚═╝          task  configure NeoVIM

-- Make sure to setup `mapleader` and `maplocalleader` before loading lazy.nvim so that mappings are correct.
-- This is also a good place to setup other settings (vim.opt)
vim.g.mapleader = " "
vim.g.maplocalleader = "\\"

local configuration_options = {
  backup = false,
  writebackup = false,
  undofile = true,
  swapfile = false,
  clipboard = "unnamedplus,unnamed",
  number = true,
  cursorline = true,
  smartindent = true,
  shiftwidth = 2,
  tabstop = 2,
  expandtab = true,
  wrap = false,
  scrolloff = 8,
  sidescrolloff = 8,
  termguicolors = true
}
for option_name, option_value in pairs(configuration_options) do vim.opt[option_name] = option_value end

-- the following configuration options only supports NeoVim version >= v0.10.0
local version = vim.version()
if version.major == 0 and version.minor < 10 then
  return
end

-- Bootstrap lazy.nvim (https://github.com/folke/lazy.nvim)
-- https://lazy.folke.io/installation
local lazypath = vim.fn.stdpath("data") .. "/lazy/lazy.nvim"
if not (vim.uv or vim.loop).fs_stat(lazypath) then
  local lazyrepo = "https://github.com/folke/lazy.nvim.git"
  local out = vim.fn.system({ "git", "clone", "--filter=blob:none", "--branch=stable", lazyrepo, lazypath })
  if vim.v.shell_error ~= 0 then
    vim.api.nvim_echo({{"Failed to clone lazy.nvim:\n", "ErrorMsg"}, {out, "WarningMsg"}, {"Press any key to exit..."}}, true, {})
    vim.fn.getchar()
    os.exit(1)
  end
end
vim.opt.rtp:prepend(lazypath)

-- Setup lazy.nvim
require("lazy").setup({
  spec = {
    {
      "sainnhe/gruvbox-material",
      lazy = false,
      priority = 1000,
      config = function()
        vim.cmd([[colorscheme gruvbox-material]])
      end
    },
    {
      "nvim-treesitter/nvim-treesitter",
      lazy = true,
      build = ":TSUpdate",
    },
    {
      "folke/noice.nvim",
      lazy = true,
      event = "VeryLazy",
      opts = {
        presets = {
          bottom_search = false,        -- use a classic bottom cmdline for search
          command_palette = true,       -- position the cmdline and popupmenu together
          long_message_to_split = true, -- long messages will be sent to a split
          inc_rename = false,           -- enables an input dialog for inc-rename.nvim
          lsp_doc_border = false,       -- add a border to hover docs and signature help
        }
      },
      dependencies = {
        "MunifTanjim/nui.nvim",
        "rcarriga/nvim-notify"
      }
    }
  }
})

