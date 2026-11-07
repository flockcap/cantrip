use crate::{agent::CliAgent, context::Richness, theme::Theme};
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf};

#[derive(Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    pub agent_precedence: Vec<String>,
    pub theme: String,
    pub auto_run: bool,
    pub include_history: bool,
    pub context_richness: Richness,
    pub history_lines: usize,
    pub agent_timeout_seconds: u64,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            agent_precedence: ["claude", "codex", "gemini", "opencode"]
                .map(String::from)
                .to_vec(),
            theme: "gruvbox".into(),
            auto_run: false,
            include_history: false,
            context_richness: Richness::Normal,
            history_lines: 10,
            agent_timeout_seconds: 120,
        }
    }
}
impl Config {
    pub fn path() -> Result<PathBuf> {
        let root = env::var_os("XDG_CONFIG_HOME")
            .filter(|s| !s.is_empty())
            .map(PathBuf::from)
            .or_else(|| dirs::home_dir().map(|h| h.join(".config")))
            .context("cannot locate config home")?;
        Ok(root.join("cantrip/config.toml"))
    }
    pub fn load() -> Result<Self> {
        let path = Self::path()?;
        match fs::read_to_string(&path) {
            Ok(text) => Self::from_toml(&text)
                .with_context(|| format!("invalid config at {}", path.display())),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Self::default()),
            Err(e) => Err(e).with_context(|| format!("read config {}", path.display())),
        }
    }
    pub fn from_toml(text: &str) -> Result<Self> {
        let config: Self = toml::from_str(text)?;
        if Theme::named(&config.theme).is_none() {
            bail!("unknown theme; choose gruvbox, rose-pine, or flock");
        }
        if config.agent_precedence.is_empty() {
            bail!("agent_precedence cannot be empty");
        }
        for name in &config.agent_precedence {
            CliAgent::new(name, config.agent_timeout_seconds)?;
        }
        if !(1..=600).contains(&config.agent_timeout_seconds) {
            bail!("agent_timeout_seconds must be 1..600");
        }
        if config.history_lines > 100 {
            bail!("history_lines must be at most 100");
        }
        Ok(config)
    }
    pub fn example() -> Result<String> {
        Ok(toml::to_string_pretty(&Self::default())?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn config_is_safe_and_typo_checked() {
        let c = Config::from_toml("").unwrap();
        assert!(!c.auto_run && !c.include_history);
        assert!(Config::from_toml("include_histroy = true").is_err());
        assert!(Config::from_toml("theme = 'nope'").is_err());
        assert!(Config::from_toml("agent_precedence = ['unknown']").is_err());
        assert!(Config::from_toml("agent_timeout_seconds = 0").is_err());
        assert!(Config::from_toml(&Config::example().unwrap()).is_ok());
    }
}
