use clap::{Parser, Subcommand};
#[derive(Parser, Debug)]
#[command(
    name = "cantrip",
    version,
    about = "Speak plainly. Get the incantation.",
    long_about = "A beautiful, keyless shell companion by Flock. Uses your existing agent login. Flags go before the wish.",
    trailing_var_arg = true
)]
pub struct Cli {
    /// Skip the first confirmation (high danger still requires confirmation)
    #[arg(long)]
    pub yolo: bool,
    /// Allow high-danger auto-run; requires --yolo
    #[arg(long, requires = "yolo")]
    pub force: bool,
    /// Use a specific installed agent
    #[arg(long, value_parser=["claude","codex","gemini","opencode"])]
    pub agent: Option<String>,
    /// Show the card without executing anything
    #[arg(long)]
    pub dry_run: bool,
    /// Print structured JSON; never execute
    #[arg(long)]
    pub json: bool,
    /// Include a fuller explanation of the command
    #[arg(long)]
    pub explain: bool,
    /// Override the configured theme
    #[arg(long, value_parser=["gruvbox","rose-pine","flock"])]
    pub theme: Option<String>,
    #[command(subcommand)]
    pub subcommand: Option<Commands>,
    /// Describe the command you want
    pub intent: Vec<String>,
}
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Install the latest stable release (uses your package manager when managed)
    Update,
    /// Generate completions for your shell
    Completions { shell: clap_complete::Shell },
    /// Print the default configuration (redirect to ~/.config/cantrip/config.toml)
    Config,
    /// Print an optional Alt-G prompt-buffer widget
    Widget {
        #[arg(value_parser=["zsh","fish"])]
        shell: String,
    },
}
