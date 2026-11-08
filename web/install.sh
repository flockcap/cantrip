#!/bin/sh
# cantrip installer — macOS and Linux, Intel and ARM. Windows: irm cantrip.fyi/install | iex
set -eu
cantrip_color=0
if [ -t 2 ] && [ -z "${NO_COLOR:-}" ]; then cantrip_color=1; fi
cantrip_aqua=''; cantrip_orange=''; cantrip_muted=''; cantrip_green=''; cantrip_reset=''
if [ "$cantrip_color" = 1 ]; then
  cantrip_aqua=$(printf '\033[38;2;142;192;124m'); cantrip_orange=$(printf '\033[38;2;254;128;25m')
  cantrip_muted=$(printf '\033[38;2;168;153;132m'); cantrip_green=$(printf '\033[38;2;184;187;38m'); cantrip_reset=$(printf '\033[0m')
fi
printf '\n  %s✦%s %scantrip%s\n  %sspeak plainly. get the incantation.%s\n\n' "$cantrip_orange" "$cantrip_reset" "$cantrip_aqua" "$cantrip_reset" "$cantrip_muted" "$cantrip_reset" >&2
case "$(uname -s)" in
  Darwin) cantrip_os='macOS' ;;
  Linux) cantrip_os='Linux' ;;
  MINGW*|MSYS*|CYGWIN*) printf '  Windows: run irm https://cantrip.fyi/install | iex in PowerShell.\n' >&2; exit 1 ;;
  *) printf '  This OS is unsupported. See https://github.com/flockcap/cantrip/releases\n' >&2; exit 1 ;;
esac
case "$(uname -m)" in
  arm64|aarch64) cantrip_arch='ARM64' ;;
  x86_64|amd64) cantrip_arch='Intel / AMD64' ;;
  *) printf '  This architecture is unsupported.\n' >&2; exit 1 ;;
esac
cantrip_tmp=$(mktemp -d "${TMPDIR:-/tmp}/cantrip-install.XXXXXX")
cantrip_pid=''
cantrip_cleanup() {
  if [ -n "$cantrip_pid" ]; then kill "$cantrip_pid" 2>/dev/null || :; fi
  rm -rf "$cantrip_tmp"
}
trap cantrip_cleanup 0
trap 'exit 130' INT
trap 'exit 143' TERM HUP
cantrip_fetch() {
  if command -v curl >/dev/null 2>&1; then curl --proto '=https' --tlsv1.2 -fLsS --retry 3 --connect-timeout 15 --max-time 180 "$1" -o "$2"
  elif command -v wget >/dev/null 2>&1; then wget -q --https-only --timeout=180 -O "$2" "$1"
  else printf '  Install curl or wget, then try again.\n' >&2; return 1; fi
}
# cargo-dist's release-specific installer embeds archive checksums. Supply the
# standard macOS sha256 implementation under its expected name when necessary.
if ! command -v sha256sum >/dev/null 2>&1; then
  if command -v shasum >/dev/null 2>&1; then
    mkdir "$cantrip_tmp/bin"
    printf '#!/bin/sh\nexec shasum -a 256 "$@"\n' > "$cantrip_tmp/bin/sha256sum"
    chmod +x "$cantrip_tmp/bin/sha256sum"
    PATH="$cantrip_tmp/bin:$PATH"; export PATH
  else printf '  SHA-256 verification requires sha256sum or shasum.\n' >&2; exit 1; fi
fi
if [ -n "${CANTRIP_INSTALL_DIR:-}" ]; then
  cantrip_install_dir="$CANTRIP_INSTALL_DIR"
  unset CANTRIP_CLI_INSTALL_DIR CARGO_DIST_FORCE_INSTALL_DIR
  export CANTRIP_CLI_UNMANAGED_INSTALL="$cantrip_install_dir"
else
  cantrip_install_dir="${CANTRIP_CLI_INSTALL_DIR:-${CARGO_DIST_FORCE_INSTALL_DIR:-${CARGO_HOME:-$HOME/.cargo}}}/bin"
fi
printf '  %s%s · %s%s\n  fetching the latest stable incantation…\n' "$cantrip_muted" "$cantrip_os" "$cantrip_arch" "$cantrip_reset" >&2
cantrip_fetch 'https://github.com/flockcap/cantrip/releases/latest/download/cantrip-cli-installer.sh' "$cantrip_tmp/installer.sh"
sh "$cantrip_tmp/installer.sh" > "$cantrip_tmp/install.log" 2>&1 &
cantrip_pid=$!
if [ "$cantrip_color" = 1 ]; then
  cantrip_frame=0
  while kill -0 "$cantrip_pid" 2>/dev/null; do
    case "$cantrip_frame" in 0) cantrip_glyph='⠋';; 1) cantrip_glyph='⠙';; 2) cantrip_glyph='⠹';; *) cantrip_glyph='⠸';; esac
    printf '\r  %s%s%s installing · verifying the release checksum' "$cantrip_aqua" "$cantrip_glyph" "$cantrip_reset" >&2
    cantrip_frame=$(( (cantrip_frame + 1) % 4 )); sleep 0.1
  done
fi
if wait "$cantrip_pid"; then cantrip_pid=''; else cantrip_pid=''; printf '\n' >&2; cat "$cantrip_tmp/install.log" >&2; exit 1; fi
if [ "$cantrip_color" = 1 ]; then printf '\r\033[2K' >&2; fi
cantrip_version=$("$cantrip_install_dir/cantrip" --version) || { cat "$cantrip_tmp/install.log" >&2; exit 1; }
printf '  %s✓%s %s\n' "$cantrip_green" "$cantrip_reset" "$cantrip_version" >&2
printf '  %s%s%s\n\n  try:    cantrip find the largest files here\n  update: cantrip update\n\n  %sby flock · cantrip.fyi%s\n\n' "$cantrip_muted" "$cantrip_install_dir/cantrip" "$cantrip_reset" "$cantrip_muted" "$cantrip_reset" >&2
case ":$PATH:" in *":$cantrip_install_dir:"*) : ;; *) printf '  Open a new terminal, or add %s to PATH.\n\n' "$cantrip_install_dir" >&2 ;; esac
