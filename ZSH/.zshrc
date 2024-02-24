export ZSH="$HOME/.oh-my-zsh"

ZSH_THEME="robbyrussell"

plugins=(git laravel)

source $ZSH/oh-my-zsh.sh

# Sublime Text
export PATH="$PATH:/Applications/Sublime Text.app/Contents/SharedSupport/bin"

export PATH=$PATH:$HOME/go/bin


# Herd injected NVM configuration
export NVM_DIR="/Users/cristianbilu/Library/Application Support/Herd/config/nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"  # This loads nvm

[[ -f "/Applications/Herd.app/Contents/Resources/config/shell/zshrc.zsh" ]] && builtin source "/Applications/Herd.app/Contents/Resources/config/shell/zshrc.zsh"

# Herd injected PHP binary.
export PATH="/Users/cristianbilu/Library/Application Support/Herd/bin/":$PATH


# Herd injected PHP 8.3 configuration.
export HERD_PHP_83_INI_SCAN_DIR="/Users/cristianbilu/Library/Application Support/Herd/config/php/83/"


# Herd injected PHP 8.2 configuration.
export HERD_PHP_82_INI_SCAN_DIR="/Users/cristianbilu/Library/Application Support/Herd/config/php/82/"
