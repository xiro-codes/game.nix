{vimPlugins, ...}: {...}: {
  config.vim = {
    assistant.copilot = {
      enable = true;
      cmp.enable = true;
    };
    lsp = {
      enable = true;
      lightbulb.enable = true;
      lspSignature.enable = true;
      lspkind.enable = true;
      trouble.enable = true;
      nvimCodeActionMenu.enable = true;
      formatOnSave = true;
    };
    languages = {
      rust = {
        enable = true;
        crates.enable = true;
        lsp.enable = true;
        treesitter.enable = true;
      };
      enableFormat = true;
      nix = {
        enable = true;
        extraDiagnostics.enable = true;
        format.enable = true;
        lsp.enable = true;
        treesitter.enable = true;
      };
    };
    dashboard.startify = {
      enable = true;
      sessionDir = "~/.config/nvim/sessions";
      sessionPersistence = true;
      sessionSort = true;
      changeToVCRoot = true;
    };
    session.nvim-session-manager.enable = true;
    comments.comment-nvim.enable = true;
    statusline.lualine = {
      enable = true;
      theme = "gruvbox_light";
    };
    theme = {
      enable = true;
      name = "gruvbox";
      style = "dark";
    };
    visuals = {
      enable = true;
      nvimWebDevicons.enable = true;
    };
    minimap.codewindow.enable = true;
    luaConfigRC = {
      lspkeybinds = ''
        local cmp = require('cmp')
        vim.opt.spellsuggest = "best,9"
        vim.opt.smartindent = true
        vim.opt.autoindent = true
        vim.api.nvim_create_autocmd('LspAttach', {
          group = vim.api.nvim_create_augroup('UserLspConfig', {}),
          callback = function(ev)
            -- Buffer local mappings.
            -- See `:help vim.lsp.*` for documentation on any of the below functions
            local opts = { buffer = ev.buf }
            vim.keymap.set('n', 'gD', vim.lsp.buf.declaration, opts)
            vim.keymap.set('n', 'gd', vim.lsp.buf.definition, opts)
            vim.keymap.set('n', 'K', vim.lsp.buf.hover, opts)
            vim.keymap.set('n', 'gi', vim.lsp.buf.implementation, opts)
            vim.keymap.set('n', '<leader>s', vim.lsp.buf.signature_help, opts)
            vim.keymap.set('n', '<leader>d', vim.lsp.buf.type_definition, opts)
            vim.keymap.set('n', '<leader>r', vim.lsp.buf.rename, opts)
            vim.keymap.set({ 'n', 'v' }, '<space>ca', vim.lsp.buf.code_action, opts)
            vim.keymap.set('n', 'gr', vim.lsp.buf.references, opts)
            vim.keymap.set('n', '<space>f', function()
              vim.lsp.buf.format { async = true }
            end, opts)
          end,
        })
        cmp.setup({
        	mapping = cmp.mapping.preset.insert({
        		['<C-u>'] = cmp.mapping.scroll_docs(-4), -- Up
        		['<C-d>'] = cmp.mapping.scroll_docs(4), -- Down
        		-- C-b (back) C-f (forward) for snippet placeholder navigation.
        		['<C-Space>'] = cmp.mapping.complete(),
        		['<CR>'] = cmp.mapping.confirm {
        			behavior = cmp.ConfirmBehavior.Replace,
        			select = true,
        		},
        		['<C-P>'] = cmp.mapping(function(fallback)
        			if cmp.visible() then
        				cmp.select_next_item()
        			else
        				fallback()
        			end
        		end, { 'i', 's' }),
        		['<C-S-P>'] = cmp.mapping(function(fallback)
        			if cmp.visible() then
        				cmp.select_prev_item()
        			else
        				fallback()
        			end
        		end, { 'i', 's' }),
        	}),
        	sources = {
        		{name = "nvim_lsp"},
        	},
        })

      '';
    };
    configRC = {
      keybinds = ''
        let mapleader = ";"
        tnoremap <Esc> <C-\><C-n>
        nnoremap o o<Esc>
        nnoremap O O<Esc>

        nnoremap <C-J> <C-W><C-J>
        nnoremap <C-K> <C-W><C-K>
        nnoremap <C-L> <C-W><C-L>
        nnoremap <C-H> <C-W><C-H>
        nnoremap <C-S-J> <C-W><C-S-J>
        nnoremap <C-S-K> <C-W><C-S-K>
        nnoremap <C-S-L> <C-W><C-S-L>
        nnoremap <C-S-H> <C-W><C-S-H>
      '';
      nvimtree_config = ''
        nnoremap <leader>f :NvimTreeFocus<CR>
        nnoremap <leader>v :Vista<CR>
      '';
      usefull_shit = ''
        set nowrap
        set iskeyword-=_
        set formatoptions-=cro
        autocmd BufWritePre * %s/\s\+$//e
        autocmd FocusGained,BufEnter * :checktime
        autocmd FocusGained,BufWritePost * :syntax sync fromstart
        autocmd FileChangedShellPost * echohl WarningMsg | echo "File changed on disk. Buffer reloaded." | echohl None
        autocmd BufNewFile,BufRead *.tera :set filetype=html
      '';
    };

    binds.cheatsheet.enable = true;
    filetree.nvimTree = {
      enable = true;
      actions.openFile.resizeWindow = true;
      filters = {
        exclude = [
          ".direnv"
          "target"
        ];
        dotfiles = true;
        gitClean = false;
        gitIgnored = true;
        noBuffer = true;
      };
    };
    autocomplete = {
      enable = true;
      type = "nvim-cmp";
    };
    tabline.nvimBufferline.enable = false;
    telescope = {
      enable = true;
    };
    treesitter = {
      enable = true;
      context.enable = true;
    };
    git = {
      enable = true;
      gitsigns.enable = true;
    };
    tabWidth = 2;

    extraPlugins = {
      rustaceanvim = {
        package = vimPlugins.rustaceanvim;
      };
      vista = {
        package = vimPlugins.vista-vim;
      };
    };
  };
}
