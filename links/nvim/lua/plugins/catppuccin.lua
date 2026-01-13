return {
  "catppuccin/nvim",
  name = "catppuccin",
  lazy = false,
  priority = 1000,
  config = function()
    require("catppuccin").setup({
      flavour = "frappe", -- latte, frappe, macchiato, mocha
      background = { -- map the background to Neovim's background option
        light = "latte",
        dark = "frappe",
      },
      integrations = {
        -- enable integrations if you use plugins
        cmp = true,
        gitsigns = true,
        nvimtree = true,
        telescope = true,
        lsp_trouble = true,
        notify = true,
      },
    })
    vim.cmd.colorscheme("catppuccin")
  end,
}
