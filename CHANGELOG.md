# Changelog

## Unreleased

- Add `border = false` for a frameless confirmation card; high-danger confirmations keep a red rule.
- Add the `glass` theme, which leaves the terminal background untouched so translucent terminals show through.
- Size the card for wrapped explanations so they are no longer clipped.

## 0.1.2

- Use PowerShell 7 when available for Windows updates.

## 0.1.1

- Add cantrip.fyi, a shared installer URL for macOS, Linux, and PowerShell.
- Add `cantrip update`, respecting Homebrew and Nix package ownership.
- Fix blank marketing renders by waiting for drawing completion and rejecting empty output.
- Keep long explanations and dangerous-command confirmation prompts visible with scrolling.

## 0.1.0

Speak plainly. Get the incantation.

- Keyless one-shot command suggestions through Claude Code, Codex, Gemini, and opencode.
- Gruvbox confirmation cards, editing, alternatives, clipboard, and regeneration.
- Local danger checks, explicit high-danger confirmation, and execution-free JSON/dry-run modes.
- Bounded context with opt-in history, process-local caching, and no telemetry.
- macOS, Linux, and Windows binaries; shell and PowerShell installers; Homebrew and Nix packaging.
