-- ███╗   ██╗███████╗ ██████╗ ██╗   ██╗██╗███╗   ███╗
-- ████╗  ██║██╔════╝██╔═══██╗██║   ██║██║████╗ ████║
-- ██╔██╗ ██║█████╗  ██║   ██║██║   ██║██║██╔████╔██║
-- ██║╚██╗██║██╔══╝  ██║   ██║╚██╗ ██╔╝██║██║╚██╔╝██║       version  3.0.0-alpha.1
-- ██║ ╚████║███████╗╚██████╔╝ ╚████╔╝ ██║██║ ╚═╝ ██║    sourced by  neovim
-- ╚═╝  ╚═══╝╚══════╝ ╚═════╝   ╚═══╝  ╚═╝╚═╝     ╚═╝          task  configure neovim

--
-- General
--

vim.scriptencoding = "utf-8"

local global  = vim.g
local options = vim.o
local keymap  = vim.keymap

--
-- Options
--

global.loaded_perl_provider = 0
global.loaded_ruby_provider = 0
global.loaded_node_provider = 0

global.show_whitespace      = 1

options.number              = true
options.cursorline          = true
options.relativenumber      = true

options.expandtab           = true  -- expand tab input with spaces characters
options.smartindent         = true  -- syntax aware indentations for newline inserts
options.tabstop             = 4     -- num of space characters per tab
options.shiftwidth          = 4     -- spaces per indentation level

--
-- Key Bindings
--

global.mapleader            = " "
global.maplocalleader       = " "

keymap.set("n", "<leader>cd", vim.cmd.Ex)

--
-- Version-Specific Configuration
--

local version = vim.version()
if version.major == 0 and version.minor < 10 then
    return
end

--
-- Plugins (lazy.nvim - https://github.com/folke/lazy.nvim)
--

-- bootstrap (https://lazy.folke.io/installation)
local lazypath = vim.fn.stdpath("data") .. "/lazy/lazy.nvim"
if not (vim.uv or vim.loop).fs_stat(lazypath) then
    local out = vim.fn.system({
        "git",
        "clone",
        "--filter=blob:none",
        "--branch=stable",
        "https://github.com/folke/lazy.nvim.git",
        lazypath
    })
    if vim.v.shell_error ~= 0 then
        vim.api.nvim_echo({
            {"Failed to clone lazy.nvim:\n", "ErrorMsg"},
            {out, "WarningMsg"},
            {"Press any key to exit..."}},
            true,
            {}
        )
        vim.fn.getchar()
        os.exit(1)
    end
end
vim.opt.rtp:append(lazypath)

require("lazy").setup({
    {
        "nvim-treesitter/nvim-treesitter",
        tag = 'v0.9.3',
        build = ":TSUpdate",
        config = function ()
            local configs = require("nvim-treesitter.configs")

            configs.setup({
                ensure_installed = {
                    "bash",
                    "lua",
                    "markdown",
                    "markdown_inline",
                    "regex",
                    "vim"
                },
                sync_install = false,
                highlight = { enable = true },
                indent = { enable = true },
            })
        end
    },
    {
        "nvim-telescope/telescope.nvim",
        tag = '0.1.8',
    config = function()
        local builtin = require('telescope.builtin')
        vim.keymap.set('n', '<leader>ff', builtin.find_files, { desc = 'Telescope find files' })
        vim.keymap.set('n', '<leader>fg', builtin.live_grep, { desc = 'Telescope live grep' })
        vim.keymap.set('n', '<leader>fb', builtin.buffers, { desc = 'Telescope buffers' })
        vim.keymap.set('n', '<leader>fh', builtin.help_tags, { desc = 'Telescope help tags' })
    end
    },
    {
        "sainnhe/gruvbox-material",
        lazy = false,
        priority = 2000
    },
    {
        "sainnhe/everforest",
        version = false,
        lazy = false,
        priority = 2000
    },
    {
        "f-person/auto-dark-mode.nvim",
        lazy = false,
        opts = {
            update_interval = 5000,
            set_dark_mode = function()
                vim.api.nvim_set_option_value("background", "dark", {})
                vim.cmd("colorscheme gruvbox-material")
            end,
            set_light_mode = function()
                vim.api.nvim_set_option_value("background", "light", {})
                vim.g.everforest_background = "hard"
                vim.cmd("colorscheme everforest")
            end,
        },
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
            },
            lsp = {
                override = {
                    ["vim.lsp.util.convert_input_to_markdown_lines"] = true,
                    ["vim.lsp.util.stylize_markdown"] = true,
                }
            }
        },
        dependencies = {
            {
                "MunifTanjim/nui.nvim",
                commit = "8d3bce9764e627b62b07424e0df77f680d47ffdb",
            },
            "rcarriga/nvim-notify"
        }
    }
})

