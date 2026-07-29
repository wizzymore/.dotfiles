return {
  {
    "mason-org/mason.nvim",
    opts = {
      ensure_installed = {
        "rust-analyzer",
        "prettier",
        "ols",
        "codelldb",
        "stylua",
        "gopls",
        "intelephense",
        "duster",
        "pint",
      },
    },
  },
}
