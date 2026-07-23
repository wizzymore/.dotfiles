return {
  "neovim/nvim-lspconfig",
  opts = {
    server = {
      intelephense = {
        init_options = {
          licenceKey = vim.fn.expand("~/intelephense/licence.txt"),
        },
      },
    },
  },
}
