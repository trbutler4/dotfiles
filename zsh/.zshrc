
# OMZ
export ZSH="$HOME/.oh-my-zsh"
ZSH_THEME="lambda"
plugins=(git)
source $ZSH/oh-my-zsh.sh


export PATH=$HOME/.local/bin:$PATH

eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)"

# for android studio
export ANDROID_HOME=/home/$USER/Android/Sdk
export NDK_HOME=$ANDROID_HOME/ndk

# for java
export JAVA_HOME=/usr/lib/jvm/java-21-openjdk-amd64
export PATH=$PATH:$JAVA_HOME/bin

# starkli
export PATH=$PATH:$HOME/.starkli/bin/


# git aliases
alias gs="git status"
alias ga="git add"
alias gc="git commit"
alias gp="git push"
alias gpl="git pull"
alias gco="git checkout"
alias gb="git branch"
alias lg="lazygit"
alias z="zellij"

# ngrok static access easier
alias ngrok-static="ngrok http --url=proud-maggot-initially.ngrok-free.app"

. "$HOME/.cargo/env"

export PATH="$PATH:/home/thomas/.dojo/bin"

fpath+=${ZDOTDIR:-~}/.zsh_functions

. /home/linuxbrew/.linuxbrew/opt/asdf/libexec/asdf.sh

export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"  # This loads nvm
[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"  # This loads nvm bash_completion
export PATH=$PATH:/sbin:/usr/sbin:/usr/local/sbin

# android tools
export PATH="$PWD/platform-tools:$PATH"
export PATH="$HOME/Android/Sdk:$PATH"

# bun completions
[ -s "/home/thomas/.bun/_bun" ] && source "/home/thomas/.bun/_bun"

# bun
export BUN_INSTALL="$HOME/.bun"
export PATH="$BUN_INSTALL/bin:$PATH"

# intelij
export PATH="$HOME/idea/bin:$PATH"

# deno
[ -f ~/.fzf.zsh ] && source ~/.fzf.zsh
. "/home/thomas/.deno/env"

# pnpm
export PNPM_HOME="/home/thomas/.local/share/pnpm"
case ":$PATH:" in
  *":$PNPM_HOME:"*) ;;
  *) export PATH="$PNPM_HOME:$PATH" ;;
esac
# pnpm end
. "/home/thomas/.starkli/env"

export EDITOR="hx"
