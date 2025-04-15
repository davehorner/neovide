-- src/config/a_neovide_tui_init.lua

-- Neovide animation settings:
vim.g.neovide_animation_length            = 0.25
vim.g.neovide_floating_opacity            = 0.8
vim.g.neovide_cursor_animate_command_line = true
vim.g.neovide_cursor_animate_in_insert_mode = true

-- On VimEnter, open a floating window:
vim.api.nvim_create_autocmd("VimEnter", {
  once = true,
  callback = function()
    local buf = vim.api.nvim_create_buf(false, true)
    vim.api.nvim_buf_set_lines(buf, 0, -1, false, {
      "╭────────────────────────╮",
      "│  Hello from Neovide!  │",
      "│ Animated TUI Example  │",
      "╰────────────────────────╯",
    })
    vim.api.nvim_open_win(buf, true, {
      relative = "editor",
      row      = 5,
      col      = 10,
      width    = 24,
      height   = 4,
      style    = "minimal",
      border   = "rounded",
    })
  end,
})
