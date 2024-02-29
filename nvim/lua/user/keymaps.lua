vim.g.mapleader = ' '
vim.g.maplocalleader = ' '

-- Clear search highlights
vim.keymap.set('n', '<C-n>', ':nohl<CR>')

-- File explorer
vim.keymap.set('n', '<leader>e', ':Explore<CR>')

-- Move to next buffer
vim.keymap.set('n', '<leader>]', ':bn<CR>')

-- Move to the previous buffer
vim.keymap.set('n', '<leader>[', ':bp<CR>')

-- Close current buffer
vim.keymap.set('n', '<leader>w', ':bd<CR>')

-- Close all open buffers.
vim.keymap.set('n', '<leader>Q', ':bufdo bdelete<CR>')

-- Allow gf to open non-existent files.
vim.keymap.set('', 'gf', ':edit <cfile><CR>')

-- Reselect visual selection after indenting.
vim.keymap.set('v', '<', '<gv')
vim.keymap.set('v', '>', '>gv')

-- Maintain the cursor position when yanking a visual selection.
-- http://ddrscott.github.io/blog/2016/yank-without-jank/
vim.keymap.set('v', 'y', 'myy`y')
vim.keymap.set('v', 'Y', 'myY`y')

-- Paste replace visual selection without copying it.
vim.keymap.set('v', 'p', '"_dP')

-- Easy insertion of a trailing ; or , from insert mode.
vim.keymap.set('i', ';;', '<Esc>A;<Esc>')
vim.keymap.set('i', ',,', '<Esc>A,<Esc>')

-- Resize with arrows
vim.keymap.set('n', '<C-Up>', ':resize +2<CR>')
vim.keymap.set('n', '<C-Down>', ':resize -2<CR>')
vim.keymap.set('n', '<C-Left>', ':vertical resize -2<CR>')
vim.keymap.set('n', '<C-Right>', ':vertical resize +2<CR>')

