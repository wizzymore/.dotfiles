# Add deno completions to search path
if [[ ":$FPATH:" != *":/Users/cristianbilu/.zsh/completions:"* ]]; then export FPATH="/Users/cristianbilu/.zsh/completions:$FPATH"; fi
export ZSH="$HOME/.oh-my-zsh"

ZSH_THEME="robbyrussell"

plugins=(git laravel)

source $ZSH/oh-my-zsh.sh

export PATH=$PATH:$HOME/go/bin

# Sublime Text

export PATH="$PATH:/Applications/Sublime Text.app/Contents/SharedSupport/bin"

# Herd injected PHP binary.
export PATH="/Users/cristianbilu/Library/Application Support/Herd/bin/":$PATH


# Herd injected PHP 8.3 configuration.
export HERD_PHP_83_INI_SCAN_DIR="/Users/cristianbilu/Library/Application Support/Herd/config/php/83/"


# Herd injected PHP 8.2 configuration.
export HERD_PHP_82_INI_SCAN_DIR="/Users/cristianbilu/Library/Application Support/Herd/config/php/82/"

# bun completions
[ -s "/Users/cristianbilu/.bun/_bun" ] && source "/Users/cristianbilu/.bun/_bun"

# bun
export BUN_INSTALL="$HOME/.bun"
export PATH="$BUN_INSTALL/bin:$PATH"

export PATH="/user/local/share/dotnet:$PATH"


# Herd injected NVM configuration
export NVM_DIR="/Users/cristianbilu/Library/Application Support/Herd/config/nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"  # This loads nvm

[[ -f "/Applications/Herd.app/Contents/Resources/config/shell/zshrc.zsh" ]] && builtin source "/Applications/Herd.app/Contents/Resources/config/shell/zshrc.zsh"

. "/Users/cristianbilu/.deno/env"
# Initialize zsh completions (added by deno install script)
autoload -Uz compinit
compinit

export LDFLAGS="-L/opt/homebrew/lib"
export CPPFLAGS="-I/opt/homebrew/include"

export ODIN_ROOT=$HOME/odin
export PATH=$PATH:$ODIN_ROOT

# pnpm
export PNPM_HOME="/Users/cristianbilu/Library/pnpm"
case ":$PATH:" in
  *":$PNPM_HOME:"*) ;;
  *) export PATH="$PNPM_HOME:$PATH" ;;
esac
# pnpm end


# Herd injected PHP 8.4 configuration.
export HERD_PHP_84_INI_SCAN_DIR="/Users/cristianbilu/Library/Application Support/Herd/config/php/84/"

alias wip="git add .; git commit -m \"WIP\""
alias gitp="git push"
alias vim ="nvim"
