<p align="center">
  <img src="assets/banner.png" alt="cantrip — Speak plainly. Get the incantation." width="100%">
</p>

<p align="center">
  <a href="https://crates.io/crates/cantrip-cli"><img src="https://img.shields.io/crates/v/cantrip-cli?style=flat-square&color=8ec07c&labelColor=282828" alt="crates.io"></a>
  <a href="https://github.com/flockcap/cantrip/actions/workflows/ci.yml"><img src="https://github.com/flockcap/cantrip/actions/workflows/ci.yml/badge.svg" alt="CI"></a>
  <a href="LICENSE-MIT"><img src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-ebdbb2?style=flat-square&labelColor=282828" alt="MIT OR Apache-2.0"></a>
  <a href="https://github.com/flockcap/cantrip/releases"><img src="https://img.shields.io/github/downloads/flockcap/cantrip/total?style=flat-square&color=fe8019&labelColor=282828" alt="downloads"></a>
</p>

<p align="center"><img src="assets/demo.gif" alt="A plain-English wish becomes a highlighted command, then converts three PNGs to WebP." width="100%"></p>

A small, beautiful terminal tool that turns a wish into the exact shell command.
**Uses the Claude Code / Codex / Gemini / opencode you already have — no API key, no new subscription.**

## Install

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/flockcap/cantrip/releases/latest/download/cantrip-cli-installer.sh | sh
```

```sh
brew install flockcap/tap/cantrip
```

```powershell
irm https://github.com/flockcap/cantrip/releases/latest/download/cantrip-cli-installer.ps1 | iex
```

```sh
cargo binstall cantrip-cli       # once the crate is published
cargo install cantrip-cli        # once the crate is published
nix run github:flockcap/cantrip -- "find the largest files here"
```

Or grab a [prebuilt binary](https://github.com/flockcap/cantrip/releases).

## Usage

```sh
cantrip resize every png here to 800px webp
cantrip find files larger than 100mb
cantrip --agent codex undo the last commit but keep my changes
cantrip                          # a quiet input line
```

`enter run · e edit · a alternatives · c copy · r regenerate · q quit`

`cantrip --json "find listening ports"` returns structured JSON and never executes. Flags go before the wish; `--dry-run` previews, `--explain` adds detail, and `--yolo` skips the first confirmation.

Gruvbox by default. `--theme rose-pine` or `--theme flock` for a change of scenery.

Dangerous shapes are flagged locally and require a second, default-No confirmation; only explicit `--yolo --force` bypasses it. These checks are heuristics: review commands before running them.

No telemetry, analytics, or phone-home. Nothing about your machine leaves the box except through your own agent CLI; that agent's provider and privacy settings still apply. History is off by default.

<details>
<summary>Configuration, shell widgets, and Nix</summary>

```sh
mkdir -p ~/.config/cantrip
cantrip config > ~/.config/cantrip/config.toml
cantrip completions zsh          # bash / zsh / fish / powershell / elvish
source <(cantrip widget zsh)      # Alt-G turns your buffer into an editable command
# fish: cantrip widget fish | source
```

```toml
agent_precedence = ["claude", "codex", "gemini", "opencode"]
theme = "gruvbox"
auto_run = false
include_history = false
context_richness = "normal"      # minimal / normal / rich
history_lines = 10
agent_timeout_seconds = 120
```

`$XDG_CONFIG_HOME` is honored. Minimal context sends cwd, OS, architecture, and shell only. Normal adds a capped filename/type listing, git branch/state, and tool availability. Rich raises the listing cap. File contents are never collected. Opt-in history adds at most `history_lines` entries from `$HISTFILE` or the shell's history file. Suggestions are cached in process memory only.

Sign in to an installed agent first. Selection is `claude → codex → gemini → opencode`; `--agent` overrides it. Current agent CLIs are required; invocation details and their tool restrictions are in [the adapter source](src/agent.rs). A forced agent's authentication failure is reported directly.

Nix: the flake exports `packages.<system>.cantrip`, `overlays.default`, and `homeManagerModules.default` for nix-darwin and home-manager:

```nix
inputs.cantrip.url = "github:flockcap/cantrip";
# in your home-manager configuration:
imports = [ inputs.cantrip.homeManagerModules.default ];
programs.cantrip = {
  enable = true;
  settings.theme = "gruvbox";
  enableZshIntegration = true;
};
```

Shell widgets require python3. Unix commands run in `$SHELL`; Windows uses `%COMSPEC%`. Use `VISUAL` or `EDITOR` for editing, otherwise enter a replacement inline. Clipboard uses pbcopy, clip.exe, wl-copy, xclip, or xsel.

</details>

<details>
<summary>Build and reproduce the artwork</summary>

```sh
cargo test --locked
cargo clippy --all-targets -- -D warnings
cargo build --release --locked
node scripts/render-art.mjs       # Chrome; bundled Iosevka fonts
vhs tapes/demo.tape               # VHS, cwebp, python3; recorded response, real execution
sh scripts/generate-release.sh    # cargo-dist 0.34
```

The demo uses a recorded response to keep timing repeatable. Art follows Flock's seeded dither and quiet monochrome identity. The runtime stays gruvbox.

</details>

---

by **[Flock](https://flock.holdings)** · [MIT](LICENSE-MIT) OR [Apache-2.0](LICENSE-APACHE)
