return {
  "stevearc/conform.nvim",
  opts = {
    formatters_by_ft = {
      php = { "pint" },
      javascript = { "vp" },
      javascriptreact = { "vp" },
      typescript = { "vp" },
      typescriptreact = { "vp" },
      css = { "vp" },
    },
    formatters = {
      vp = {
        command = "vp",
        args = { "fmt", "--stdin-filepath", "$FILENAME" },
        stdin = true,
      },
    },
  },
}
