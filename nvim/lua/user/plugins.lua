-- Bootstrap Lazy
local lazypath = vim.fn.stdpath("data") .. "/lazy/lazy.nvim"
if not vim.loop.fs_stat(lazypath) then
  vim.fn.system({
    "git",
    "clone",
    "--filter=blob:none",
    "https://github.com/folke/lazy.nvim.git",
    "--branch=stable",
    lazypath,
  })
end
vim.opt.rtp:prepend(lazypath)

require("lazy").setup({
    -- Color scheme
    { import = 'user.plugins.tokyonight' },

    -- Add comments support
    { import = 'user.plugins.vim-commentary' },

    -- Add, change, and delete surrounding text.
    { 'tpope/vim-surround' },

    -- Pairs of handy bracket mappings, like [b and ]b.
    { 'tpope/vim-unimpaired', event = 'VeryLazy' },

    -- Indent autodetection with editorconfig support.
    { 'tpope/vim-sleuth' },

    -- Allow plugins to enable repeating of commands.
    { 'tpope/vim-repeat' },

    -- Jump to the last location when opening a file.
    { 'farmergreg/vim-lastplace' },

    -- Automatically create parent dirs when saving.
    { 'jessarcher/vim-heritage' },

    -- Automatically set the working directory to the project root.
    { import = 'user.plugins.vim-rooter' },

    -- Automatically add closing brackets, quotes, etc.
    { 'windwp/nvim-autopairs', config = true },

    -- Add smooth scrolling to avoid jarring jumps
    { 'karb94/neoscroll.nvim', config = true },

    -- Automatically fix indentation when pasting code.
    { import = 'user.plugins.vim-pasta' },

    -- Fuzzy finder
    { import = 'user.plugins.telescope' },

    -- File tree sidebar
    { import = 'user.plugins.neo-tree' },

    -- A Status line.
    { import = 'user.plugins.lualine' },

    -- Display buffers as tabs.
    { import = 'user.plugins.bufferline' },

    -- Display indentation lines.
    { import = 'user.plugins.indent-blankline' },

    -- Add a dashboard.
    { import = 'user.plugins.dashboard-nvim' },

    --- Floating terminal.
    { import = 'user.plugins.floaterm' },

    -- Improved syntax highlighting
    { import = 'user.plugins.treesitter' },
})
