-- src/config/a_neovide_blink_cmp.lua
-- dave horner 4/25 MIT

-- DEBUG: show stdpath('data') and the first few runtimepath entries
vim.notify("stdpath('data') = " .. vim.fn.stdpath('data'))
for i, p in ipairs(vim.opt.runtimepath:get()) do
  if i > 5 then break end
  vim.notify(string.format("rtp[%d] = %s", i, p))
end

-- ─── 1) MANUALLY ADD blink.cmp TO RUNTIMEPATH ──────────────────────────────
local data_dir    = vim.fn.stdpath('data')            -- e.g. ~/.local/share/nvim
local blink_path  = data_dir .. '/blink.cmp'           -- you must clone here
if vim.fn.isdirectory(blink_path) == 1 then
  vim.opt.runtimepath:prepend(blink_path)
else
  vim.notify('⚠ blink.cmp not found at '..blink_path, vim.log.levels.WARN)
end

-- ─── 2) INLINE DEMO SOURCE ─────────────────────────────────────────────────
package.preload['blink.cmp.sources.demo'] = function()
  local types = require('blink.cmp.types')
  local source = {}
  source.__index = source

  function source.new(opts) return setmetatable({ opts = opts }, source) end
  function source:get_trigger_characters() return {} end

  function source:get_completions(ctx, callback)
    callback({
      items = {
{ label = 'apple',        kind = types.CompletionItemKind.Text, insertText = 'apple' },
{ label = 'apricot',      kind = types.CompletionItemKind.Text, insertText = 'apricot' },
{ label = 'avocado',      kind = types.CompletionItemKind.Text, insertText = 'avocado' },
{ label = 'banana',       kind = types.CompletionItemKind.Text, insertText = 'banana' },
{ label = 'blackberry',   kind = types.CompletionItemKind.Text, insertText = 'blackberry' },
{ label = 'blueberry',    kind = types.CompletionItemKind.Text, insertText = 'blueberry' },
{ label = 'boysenberry',  kind = types.CompletionItemKind.Text, insertText = 'boysenberry' },
{ label = 'cantaloupe',   kind = types.CompletionItemKind.Text, insertText = 'cantaloupe' },
{ label = 'cherry',       kind = types.CompletionItemKind.Text, insertText = 'cherry' },
{ label = 'clementine',   kind = types.CompletionItemKind.Text, insertText = 'clementine' },
{ label = 'coconut',      kind = types.CompletionItemKind.Text, insertText = 'coconut' },
{ label = 'cranberry',    kind = types.CompletionItemKind.Text, insertText = 'cranberry' },
{ label = 'date',         kind = types.CompletionItemKind.Text, insertText = 'date' },
{ label = 'dragonfruit',  kind = types.CompletionItemKind.Text, insertText = 'dragonfruit' },
{ label = 'durian',       kind = types.CompletionItemKind.Text, insertText = 'durian' },
{ label = 'elderberry',   kind = types.CompletionItemKind.Text, insertText = 'elderberry' },
{ label = 'fig',          kind = types.CompletionItemKind.Text, insertText = 'fig' },
{ label = 'gooseberry',   kind = types.CompletionItemKind.Text, insertText = 'gooseberry' },
{ label = 'grape',        kind = types.CompletionItemKind.Text, insertText = 'grape' },
{ label = 'grapefruit',   kind = types.CompletionItemKind.Text, insertText = 'grapefruit' },
{ label = 'guava',        kind = types.CompletionItemKind.Text, insertText = 'guava' },
{ label = 'honeydew',     kind = types.CompletionItemKind.Text, insertText = 'honeydew' },
{ label = 'jackfruit',    kind = types.CompletionItemKind.Text, insertText = 'jackfruit' },
{ label = 'kiwi',         kind = types.CompletionItemKind.Text, insertText = 'kiwi' },
{ label = 'kumquat',      kind = types.CompletionItemKind.Text, insertText = 'kumquat' },
{ label = 'lemon',        kind = types.CompletionItemKind.Text, insertText = 'lemon' },
{ label = 'lime',         kind = types.CompletionItemKind.Text, insertText = 'lime' },
{ label = 'lychee',       kind = types.CompletionItemKind.Text, insertText = 'lychee' },
{ label = 'mango',        kind = types.CompletionItemKind.Text, insertText = 'mango' },
{ label = 'nectarine',    kind = types.CompletionItemKind.Text, insertText = 'nectarine' },
{ label = 'orange',       kind = types.CompletionItemKind.Text, insertText = 'orange' },
{ label = 'papaya',       kind = types.CompletionItemKind.Text, insertText = 'papaya' },
{ label = 'passionfruit', kind = types.CompletionItemKind.Text, insertText = 'passionfruit' },
{ label = 'peach',        kind = types.CompletionItemKind.Text, insertText = 'peach' },
{ label = 'pear',         kind = types.CompletionItemKind.Text, insertText = 'pear' },
{ label = 'pineapple',    kind = types.CompletionItemKind.Text, insertText = 'pineapple' },
{ label = 'plum',         kind = types.CompletionItemKind.Text, insertText = 'plum' },
{ label = 'pomegranate',  kind = types.CompletionItemKind.Text, insertText = 'pomegranate' },
{ label = 'persimmon',    kind = types.CompletionItemKind.Text, insertText = 'persimmon' },
{ label = 'quince',       kind = types.CompletionItemKind.Text, insertText = 'quince' },
{ label = 'raspberry',    kind = types.CompletionItemKind.Text, insertText = 'raspberry' },
{ label = 'starfruit',    kind = types.CompletionItemKind.Text, insertText = 'starfruit' },
{ label = 'strawberry',   kind = types.CompletionItemKind.Text, insertText = 'strawberry' },
{ label = 'tangerine',    kind = types.CompletionItemKind.Text, insertText = 'tangerine' },
{ label = 'tamarind',     kind = types.CompletionItemKind.Text, insertText = 'tamarind' },
{ label = 'tomato',       kind = types.CompletionItemKind.Text, insertText = 'tomato' },
{ label = 'ugli fruit',   kind = types.CompletionItemKind.Text, insertText = 'ugli fruit' },
{ label = 'watermelon',   kind = types.CompletionItemKind.Text, insertText = 'watermelon' },
{ label = 'yuzu',         kind = types.CompletionItemKind.Text, insertText = 'yuzu' },
{ label = 'zucchini',     kind = types.CompletionItemKind.Text, insertText = 'zucchini' },

      },
      is_incomplete_backward = false,
      is_incomplete_forward  = false,
    })
  end

  return source
end

-- ─── 3) CONFIGURE blink.cmp ────────────────────────────────────────────────
require('blink.cmp').setup({
  -- auto‑show menu on every keystroke
  completion = {
    menu               = { border = 'single' },
    documentation      = { window = { border = 'single' } },
    trigger            = { show_on_blocked_trigger_characters = {} },
    list = {selection          = {  preselect = true,
  auto_insert = true,},
  },},

  -- keybindings
  keymap = {
    preset      = 'none',
    ['<C-Space>']= { 'accept' },
    ['<Tab>']   = { 'insert_next', 'fallback' },
    ['<S-Tab>'] = { 'insert_prev' },
    ['<CR>']    = { 'accept' },
  },

  -- sources: demo first, then buffer & path
  sources = {
    providers = {
      demo   = { module = 'blink.cmp.sources.demo', enabled = true },
      buffer = { enabled = true },
      path   = { enabled = true },
    },
    default = function() return { 'demo', 'buffer', 'path' } end,
   min_keyword_length = 0, 
  },

              fuzzy = {
          implementation = "lua",
          sorts = {
            'exact',
            function(a,b)
              if a.client_name and b.client_name and a.client_name ~= b.client_name then
                return b.client_name == 'emmet_ls'
              end
            end,
            'score','sort_text',
          },
        },
})

-- ─── 4) OPTIONAL: Neovide animations ───────────────────────────────────────
vim.g.neovide_animation_length              = 0.25
vim.g.neovide_floating_opacity              = 0.8
vim.g.neovide_cursor_animate_command_line   = true
vim.g.neovide_cursor_animate_in_insert_mode = true

-- ─── 5) DEMO FLOATING WINDOW ───────────────────────────────────────────────
vim.api.nvim_create_autocmd('VimEnter', {
  once   = true,
  nested = true,
  callback = function()
    -- 1) Your demo box (unchanged)
    local demo_buf = vim.api.nvim_create_buf(false, true)
    vim.api.nvim_buf_set_option(demo_buf, 'filetype', 'text')
    vim.api.nvim_buf_set_lines(demo_buf, 0, -1, false, {
      '',
    })
    vim.api.nvim_open_win(demo_buf, true, {
      relative = 'editor', row = 5, col = 10,
      width    = 36, height = 1,
      style    = 'minimal', border = 'rounded',
    })

    -- 2) Instruction box below it
    local inst_buf = vim.api.nvim_create_buf(false, true)
    -- write lines *first*
    vim.api.nvim_buf_set_lines(inst_buf, 0, -1, false, {
      '<Tab>/<S-Tab> to cycle, <CR> to accept, <C-Space> to open, or just start typing.',
    })
    -- now make it read‑only (optional)
    vim.api.nvim_buf_set_option(inst_buf, 'modifiable', false)
    vim.api.nvim_buf_set_option(inst_buf, 'bufhidden', 'wipe')

    vim.api.nvim_open_win(inst_buf, false, {
      relative = 'editor',
      row      = 1,
      col      = 1,
      width    = 79,
      height   = 1,
      style    = 'minimal',
      border   = 'rounded',
    })
    vim.cmd 'startinsert'
    vim.schedule(function()
      require('blink.cmp').show()
    end)    
  end,
})