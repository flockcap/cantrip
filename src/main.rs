use anyhow::{bail, Context, Result};
use cantrip_cli::{
    agent::{self, Agent},
    cli::{Cli, Commands},
    config::Config,
    context, safety,
    suggestion::{self, Danger, Suggestion},
    theme::Theme,
    ui::{self, Action, Card},
};
use clap::{CommandFactory, Parser};
use std::{
    env, fs,
    io::{self, IsTerminal, Write},
    process::{Command, ExitCode, Stdio},
};

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(code) => ExitCode::from(code.clamp(0, 255) as u8),
        Err(error) => {
            eprintln!("cantrip: {}", suggestion::visible(&format!("{error:#}")));
            ExitCode::FAILURE
        }
    }
}
async fn run() -> Result<i32> {
    let args = Cli::parse();
    if let Some(command) = args.subcommand {
        match command {
            Commands::Completions { shell } => {
                clap_complete::generate(shell, &mut Cli::command(), "cantrip", &mut io::stdout())
            }
            Commands::Config => print!("{}", Config::example()?),
            Commands::Widget { shell } => print!(
                "{}",
                if shell == "zsh" {
                    include_str!("../shells/cantrip.zsh")
                } else {
                    include_str!("../shells/cantrip.fish")
                }
            ),
        }
        return Ok(0);
    }
    let config = Config::load()?;
    let intent = if args.intent.is_empty() {
        ui::wish()?
    } else {
        args.intent.join(" ")
    };
    if intent.len() > 65536 {
        bail!("wish exceeds 64 KiB");
    }
    let agent = agent::detect(
        &config.agent_precedence,
        args.agent.as_deref(),
        config.agent_timeout_seconds,
    )?;
    let context = context::gather(
        config.context_richness,
        config.include_history,
        config.history_lines,
    )?;
    let request = suggestion::prompt(&intent, &context, args.explain)?;
    let theme =
        Theme::named(args.theme.as_deref().unwrap_or(&config.theme)).context("unknown theme")?;
    let mut cache = cantrip_cli::cache::Cache::default();
    let mut result = generate(&mut cache, &agent, &request, !args.json, false).await?;
    let mut base_danger = result.danger;
    let mut alternatives = alternatives(&result);
    let mut index = 0;
    let mut status = String::new();
    loop {
        safety::apply(&mut result, base_danger, &context.cwd);
        if args.json {
            println!("{}", serde_json::to_string(&result)?);
            return Ok(if result.executable() { 0 } else { 2 });
        }
        let auto = safety::may_auto_run(
            &result,
            args.yolo || config.auto_run,
            args.yolo && args.force,
            args.dry_run,
            args.json,
        );
        let action = if auto {
            Action::Run
        } else {
            ui::card(&Card {
                suggestion: &result,
                intent: &intent,
                agent: agent.name(),
                theme,
                dry_run: args.dry_run,
                status: &status,
            })?
        };
        status.clear();
        match action {
            Action::Quit => return Ok(0),
            Action::Run => {
                if args.dry_run || !result.executable() {
                    return Ok(0);
                }
                if result.danger == Danger::High && !auto && !ui::confirm_high(&result, theme)? {
                    return Ok(0);
                }
                return execute(&result.command);
            }
            Action::Edit => {
                match edit(&result.command) {
                    Ok(command) if !command.trim().is_empty() => {
                        let edited = Suggestion {
                            command,
                            explanation: "Edited by you. Review before running.".into(),
                            danger: Danger::Low,
                            alternatives: Vec::new(),
                            warnings: Vec::new(),
                            raw: None,
                        };
                        // Use the same validation as agent output, including control characters.
                        result = suggestion::parse(&serde_json::to_string(&edited)?)?;
                        base_danger = Danger::Low;
                        alternatives = self::alternatives(&result);
                        index = 0;
                    }
                    Ok(_) => status = "empty edit discarded".into(),
                    Err(error) => status = format!("editor: {error}"),
                }
            }
            Action::Alternative => {
                if alternatives.len() > 1 {
                    index = (index + 1) % alternatives.len();
                    result.command = alternatives[index].clone();
                    status = format!("alternative {} of {}", index + 1, alternatives.len());
                } else {
                    status = "no alternatives returned".into();
                }
            }
            Action::Copy => {
                status = match copy(&result.command) {
                    Ok(()) => "copied to clipboard".into(),
                    Err(e) => format!("clipboard: {e}"),
                };
            }
            Action::Regenerate => {
                result = generate(&mut cache, &agent, &request, true, true).await?;
                base_danger = result.danger;
                alternatives = self::alternatives(&result);
                index = 0;
            }
        }
    }
}
fn alternatives(s: &Suggestion) -> Vec<String> {
    std::iter::once(s.command.clone())
        .chain(s.alternatives.clone())
        .filter(|s| !s.trim().is_empty())
        .collect()
}

async fn generate(
    cache: &mut cantrip_cli::cache::Cache,
    agent: &dyn Agent,
    prompt: &str,
    progress: bool,
    bypass: bool,
) -> Result<Suggestion> {
    let task = cache.get_or_generate(agent, prompt, bypass);
    tokio::pin!(task);
    let mut spinner = Spinner::new(progress && io::stderr().is_terminal(), agent.name());
    let mut tick = tokio::time::interval(std::time::Duration::from_millis(90));
    loop {
        tokio::select! {
            result=&mut task => return result,
            _=tick.tick() => spinner.tick(),
            _=tokio::signal::ctrl_c() => bail!("cancelled"),
        }
    }
}
struct Spinner {
    enabled: bool,
    frame: usize,
    agent: String,
}
impl Spinner {
    fn new(enabled: bool, agent: &str) -> Self {
        Self {
            enabled,
            frame: 0,
            agent: agent.into(),
        }
    }
    fn tick(&mut self) {
        if self.enabled {
            let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
            eprint!(
                "\r\x1b[2K  \x1b[38;2;142;192;124m{}\x1b[0m asking {} · your login, your agent",
                frames[self.frame % frames.len()],
                self.agent
            );
            let _ = io::stderr().flush();
            self.frame += 1;
        }
    }
}
impl Drop for Spinner {
    fn drop(&mut self) {
        if self.enabled {
            eprint!("\r\x1b[2K");
            let _ = io::stderr().flush();
        }
    }
}

fn edit(command: &str) -> Result<String> {
    if let Some(editor) = env::var_os("VISUAL")
        .or_else(|| env::var_os("EDITOR"))
        .filter(|s| !s.is_empty())
    {
        let words = shell_words::split(&editor.to_string_lossy()).context("parse EDITOR")?;
        let (program, args) = words.split_first().context("EDITOR is empty")?;
        let mut file = tempfile::Builder::new()
            .prefix("cantrip-")
            .suffix(".sh")
            .tempfile()?;
        file.write_all(command.as_bytes())?;
        file.flush()?;
        let status = Command::new(program).args(args).arg(file.path()).status()?;
        if !status.success() {
            bail!("editor exited with {status}");
        }
        Ok(fs::read_to_string(file.path())?.trim_end().into())
    } else {
        println!("\ncurrent: {}\n", suggestion::visible(command));
        print!("edit › ");
        io::stdout().flush()?;
        let mut text = String::new();
        io::stdin().read_line(&mut text)?;
        Ok(text.trim_end().into())
    }
}
fn copy(command: &str) -> Result<()> {
    let programs: &[(&str, &[&str])] = if cfg!(target_os = "macos") {
        &[("pbcopy", &[])]
    } else if cfg!(windows) {
        &[("clip.exe", &[])]
    } else {
        &[
            ("wl-copy", &[]),
            ("xclip", &["-selection", "clipboard"]),
            ("xsel", &["--clipboard", "--input"]),
        ]
    };
    for (program, args) in programs {
        if which::which(program).is_err() {
            continue;
        }
        let mut child = Command::new(program)
            .args(*args)
            .stdin(Stdio::piped())
            .spawn()?;
        child
            .stdin
            .take()
            .context("clipboard stdin")?
            .write_all(command.as_bytes())?;
        if child.wait()?.success() {
            return Ok(());
        }
    }
    bail!("install wl-copy, xclip, or xsel for clipboard support")
}
fn execute(command: &str) -> Result<i32> {
    println!("\n› {}\n", suggestion::visible(command));
    io::stdout().flush()?;
    let shell = context::shell();
    let flag = if cfg!(windows) && shell.to_lowercase().ends_with("cmd.exe") {
        "/C"
    } else {
        "-c"
    };
    let status = Command::new(&shell)
        .arg(flag)
        .arg(command)
        .status()
        .with_context(|| format!("execute through {shell}"))?;
    let code = status.code().unwrap_or(130);
    if io::stdout().is_terminal() {
        if status.success() {
            println!("\n\x1b[38;2;184;187;38m✓ done\x1b[0m");
        } else {
            eprintln!("\ncommand exited with {code}");
        }
    }
    Ok(code)
}
