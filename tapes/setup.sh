# Sourced by VHS. Only the agent response is recorded; UI and conversions are real.
export CANTRIP_ROOT="$PWD"
export CANTRIP_DEMO_RESPONSE="$CANTRIP_ROOT/tapes/fixtures/response.json"
export PATH="$CANTRIP_ROOT/target/release:$CANTRIP_ROOT/tapes/fixtures/bin:$PATH"
export XDG_CONFIG_HOME="$CANTRIP_ROOT/target/demo-config"
export SHELL=/bin/bash
unset NO_COLOR
export COLORTERM=truecolor
export CLICOLOR_FORCE=1
export CANTRIP_DEMO_DIR="$(mktemp -d "${TMPDIR:-/tmp}/cantrip-demo.XXXXXX")"
python3 "$CANTRIP_ROOT/tapes/fixtures/images.py" "$CANTRIP_DEMO_DIR"
cd "$CANTRIP_DEMO_DIR"
export PS1='\[\e[38;2;142;192;124m\]❯\[\e[0m\] '
clear
