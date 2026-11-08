# source <(cantrip widget zsh) — Alt-G converts the current buffer to a command.
# Requires python3 to decode --json. The result stays editable; never executes.
_cantrip_widget() {
  local result
  result=$(cantrip --json -- "$BUFFER" | python3 -c 'import json,sys; s=json.load(sys.stdin); assert not s.get("raw"); print(s["command"],end="")') || return
  BUFFER=$result
  CURSOR=${#BUFFER}
  zle redisplay
}
zle -N _cantrip_widget
bindkey '^[g' _cantrip_widget
