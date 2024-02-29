return {
  'numToStr/Comment.nvim',
  lazy = false,
  config = function()
    require('Comment').setup()
    -- Maintain cursor position
    vim.keymap.set('n', 'gcap', 'my<cmd>norm vip<bar>gc<cr>`y')
  end,
}
