//! Adapters only ask for text. Execution belongs to the confirmation flow.
use anyhow::{bail, Context, Result};
use async_trait::async_trait;
use serde_json::Value;
use std::{process::Stdio, time::Duration};
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWriteExt},
    process::Command,
};

#[async_trait]
pub trait Agent: Send + Sync {
    fn name(&self) -> &str;
    fn is_available(&self) -> bool;
    async fn run(&self, prompt: &str) -> Result<String>;
}

#[derive(Clone, Debug)]
pub struct CliAgent {
    name: String,
    timeout: Duration,
}

impl CliAgent {
    pub fn new(name: &str, timeout_seconds: u64) -> Result<Self> {
        if !["claude", "codex", "gemini", "opencode"].contains(&name) {
            bail!("unknown agent {name:?}; choose claude, codex, gemini, or opencode");
        }
        Ok(Self {
            name: name.into(),
            timeout: Duration::from_secs(timeout_seconds),
        })
    }
}

pub fn detect(precedence: &[String], forced: Option<&str>, timeout: u64) -> Result<CliAgent> {
    let names = forced
        .map(|s| vec![s.to_owned()])
        .unwrap_or_else(|| precedence.to_vec());
    for name in names {
        let agent = CliAgent::new(&name, timeout)?;
        if agent.is_available() {
            return Ok(agent);
        }
    }
    bail!("no agent found on PATH. Sign in to Claude Code, Codex, Gemini CLI, or opencode, then try again. cantrip uses your existing agent login; no API key is needed. See https://github.com/flockcap/cantrip#usage")
}

#[async_trait]
impl Agent for CliAgent {
    fn name(&self) -> &str {
        &self.name
    }
    fn is_available(&self) -> bool {
        which::which(&self.name).is_ok()
    }

    async fn run(&self, prompt: &str) -> Result<String> {
        let sandbox = tempfile::tempdir().context("create isolated agent directory")?;
        let mut cmd = Command::new(which::which(&self.name)?);
        cmd.current_dir(sandbox.path())
            .kill_on_drop(true)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        match self.name.as_str() {
            "claude" => {
                cmd.args([
                    "-p",
                    "--output-format",
                    "json",
                    "--tools",
                    "",
                    "--strict-mcp-config",
                    "--mcp-config",
                    "{\"mcpServers\":{}}",
                    "--disable-slash-commands",
                    "--setting-sources",
                    "",
                    "--no-session-persistence",
                ]);
                // A nested print invocation is safe: no tools, no project context.
                cmd.env_remove("CLAUDECODE");
            }
            "codex" => {
                cmd.args([
                    "exec",
                    "--skip-git-repo-check",
                    "--sandbox",
                    "read-only",
                    "--ephemeral",
                    "--ignore-user-config",
                    "--ignore-rules",
                    "-c",
                    "features.shell_tool=false",
                    "-c",
                    "features.apply_patch_freeform=false",
                    "-c",
                    "features.multi_agent=false",
                    "--json",
                    "-",
                ]);
            }
            "gemini" => {
                let settings = sandbox.path().join("settings.json");
                std::fs::write(
                    &settings,
                    r#"{"tools":{"core":["__cantrip_no_tools__"],"exclude":["run_shell_command","write_file","replace","read_file","read_many_files","list_directory","glob","search_file_content","google_web_search","web_fetch","save_memory","activate_skill","delegate_to_agent"]},"mcp":{"allowed":["__cantrip_no_servers__"]},"context":{"fileName":"CANTRIP_NO_CONTEXT.md","includeDirectoryTree":false},"hooks":{"enabled":false},"telemetry":{"enabled":false}}"#,
                )?;
                cmd.env("GEMINI_CLI_SYSTEM_SETTINGS_PATH", settings).args([
                    "-p",
                    "Return only the JSON command suggestion requested on stdin. Do not use tools.",
                    "--output-format",
                    "json",
                    "--extensions",
                    "",
                ]);
            }
            "opencode" => {
                cmd.env(
                    "OPENCODE_CONFIG_CONTENT",
                    r#"{"permission":{"*":"deny"},"share":"disabled","autoupdate":false}"#,
                )
                .args(["run", "--format", "json"]);
            }
            _ => unreachable!(),
        }
        let name = &self.name;
        let task = async {
            let mut child = cmd.spawn().with_context(|| format!("start {name}"))?;
            let mut stdin = child.stdin.take().context("agent stdin unavailable")?;
            stdin.write_all(prompt.as_bytes()).await?;
            drop(stdin);
            let stdout = child.stdout.take().context("agent stdout unavailable")?;
            let stderr = child.stderr.take().context("agent stderr unavailable")?;
            let (stdout, stderr, status) = tokio::try_join!(
                read_bounded(stdout, 1_048_576),
                read_bounded(stderr, 65536),
                child.wait()
            )?;
            if !status.success() {
                let error = String::from_utf8_lossy(&stderr);
                bail!(
                    "{name} exited with {}: {}\nCheck `{name} --help` and your agent login.",
                    status,
                    error.chars().take(2000).collect::<String>()
                );
            }
            normalize(name, &String::from_utf8_lossy(&stdout))
        };
        tokio::time::timeout(self.timeout, task)
            .await
            .with_context(|| {
                format!(
                    "{name} timed out after {}s; increase agent_timeout_seconds in config",
                    self.timeout.as_secs()
                )
            })?
    }
}

pub fn normalize(name: &str, raw: &str) -> Result<String> {
    if name == "claude" || name == "gemini" {
        if let Ok(value) = serde_json::from_str::<Value>(raw) {
            if value.get("is_error").and_then(Value::as_bool) == Some(true) {
                bail!("{name}: {}", value["result"]);
            }
            for field in ["result", "response"] {
                if let Some(text) = value.get(field).and_then(Value::as_str) {
                    return Ok(text.into());
                }
            }
        }
    }
    if name == "codex" || name == "opencode" {
        let mut messages = Vec::new();
        for line in raw.lines() {
            if let Ok(v) = serde_json::from_str::<Value>(line) {
                if v["type"] == "error" {
                    bail!("{name}: {}", v.get("message").unwrap_or(&v));
                }
                if v["type"] == "item.completed" && v["item"]["type"] == "agent_message" {
                    if let Some(text) = v["item"]["text"].as_str() {
                        messages.push(text.to_owned());
                    }
                }
                if v["type"] == "text" {
                    if let Some(text) = v["part"]["text"].as_str() {
                        messages.push(text.to_owned());
                    }
                }
            }
        }
        if !messages.is_empty() {
            return Ok(messages.join("\n"));
        }
    }
    Ok(raw.trim().into())
}

async fn read_bounded(reader: impl AsyncRead + Unpin, limit: u64) -> std::io::Result<Vec<u8>> {
    let mut buffer = Vec::new();
    reader.take(limit + 1).read_to_end(&mut buffer).await?;
    if buffer.len() as u64 > limit {
        return Err(std::io::Error::other("agent output exceeded capture limit"));
    }
    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unwraps_agent_envelopes() {
        assert_eq!(
            normalize("claude", r#"{"result":"hello"}"#).unwrap(),
            "hello"
        );
        assert_eq!(
            normalize("gemini", r#"{"response":"hello"}"#).unwrap(),
            "hello"
        );
        assert_eq!(normalize("codex", "{\"type\":\"item.completed\",\"item\":{\"type\":\"agent_message\",\"text\":\"hello\"}}\n").unwrap(), "hello");
        assert_eq!(
            normalize("opencode", r#"{"type":"text","part":{"text":"hello"}}"#).unwrap(),
            "hello"
        );
        assert!(normalize("claude", r#"{"is_error":true,"result":"login required"}"#).is_err());
    }
}
