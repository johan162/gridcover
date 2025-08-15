# shellcheck disable=SC2148
# ====================================================================================
# Container specific bashrc with colored git-prompt 
# ====================================================================================
# Source global definitions
if [ -f /etc/bashrc ]; then
	# shellcheck disable=SC1091
	. /etc/bashrc
fi

# User specific environment
if [[ ":$PATH:" != *":$HOME/.local/bin:$HOME/bin:"* ]]; then
    # not present
     PATH="$HOME/.local/bin:$HOME/bin:$PATH"
fi
export PATH
 
alias rm='rm -i'
alias cp='cp -i'
alias mv='mv -i'

__bash_prompt() {
    # shellcheck disable=SC2016
    local userpart='`export XIT=$? \
        && [ ! -z "${GITHUB_USER:-}" ] && echo -n "\[\033[0;32m\]@${GITHUB_USER:-} " || echo -n "\[\033[0;32m\]\u " \
        && [ "$XIT" -ne "0" ] && echo -n "\[\033[1;31m\]➜" || echo -n "\[\033[0m\]➜"`'
   
    # shellcheck disable=SC2016
    local gitbranch='`\
    if [ -n "$(git rev-parse --show-toplevel 2>/dev/null)" ]; then
        if [ "$(git config --get devcontainers-theme.hide-status 2>/dev/null)" != 1 ] && [ "$(git config --get codespaces-theme.hide-
status 2>/dev/null)" != 1 ]; then \
            export BRANCH="$(git --no-optional-locks symbolic-ref --short HEAD 2>/dev/null || git --no-optional-locks rev-parse --sho
rt HEAD 2>/dev/null)"; \
            if [ "${BRANCH:-}" != "" ]; then \
                echo -n "\[\033[0;36m\](\[\033[1;31m\]${BRANCH:-}" \
                && if [ "$(git config --get devcontainers-theme.show-dirty 2>/dev/null)" = 1 ] && \
                    git --no-optional-locks ls-files --error-unmatch -m --directory --no-empty-directory -o --exclude-standard ":/*" 
> /dev/null 2>&1; then \
                        echo -n " \[\033[1;33m\]✗"; \
                fi \
                && echo -n "\[\033[0;36m\]) "; \
            fi; \
        fi\
    fi`'

    local lightblue='\[\033[1;34m\]'
    local removecolor='\[\033[0m\]'
    PS1="${userpart} ${lightblue}\w ${gitbranch}${removecolor}\$ "
    unset -f __bash_prompt
}
__bash_prompt
# Keep the last four components of the current directory in the prompt
export PROMPT_DIRTRIM=4

