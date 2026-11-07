use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    env, fs,
    io::{Read, Seek, SeekFrom},
    path::PathBuf,
    process::Command,
};

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Richness {
    Minimal,
    #[default]
    Normal,
    Rich,
}

#[derive(Debug, Serialize)]
pub struct LocalContext {
    pub cwd: PathBuf,
    pub os: String,
    pub arch: &'static str,
    pub shell: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<Entry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git: Option<Git>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub tools: BTreeMap<String, bool>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub history: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct Entry {
    pub name: String,
    pub kind: &'static str,
}
#[derive(Debug, Serialize)]
pub struct Git {
    pub branch: String,
    pub dirty: bool,
}

pub fn gather(
    richness: Richness,
    include_history: bool,
    history_lines: usize,
) -> Result<LocalContext> {
    let cwd = env::current_dir()?;
    let mut context = LocalContext {
        cwd,
        os: os_version(),
        arch: env::consts::ARCH,
        shell: shell(),
        entries: Vec::new(),
        git: None,
        tools: BTreeMap::new(),
        history: Vec::new(),
    };
    if richness != Richness::Minimal {
        let cap = if richness == Richness::Rich { 100 } else { 40 };
        if let Ok(entries) = fs::read_dir(&context.cwd) {
            context.entries = entries
                .take(2000)
                .filter_map(|e| e.ok())
                .filter_map(|e| {
                    let ty = e.file_type().ok()?;
                    Some(Entry {
                        name: e.file_name().to_string_lossy().into(),
                        kind: if ty.is_symlink() {
                            "symlink"
                        } else if ty.is_dir() {
                            "directory"
                        } else {
                            "file"
                        },
                    })
                })
                .collect();
            context.entries.sort_by(|a, b| a.name.cmp(&b.name));
            context.entries.truncate(cap);
        }
        if let Some(branch) = git(&["rev-parse", "--abbrev-ref", "HEAD"]) {
            context.git = Some(Git {
                branch,
                dirty: git(&["status", "--porcelain", "--untracked-files=no"])
                    .is_some_and(|s| !s.is_empty()),
            });
        }
        for name in [
            "ffmpeg", "magick", "convert", "cwebp", "rg", "jq", "fd", "find", "docker", "kubectl",
            "tar", "gzip", "git", "rsync", "curl", "python3", "node", "brew", "nix", "pwsh",
        ] {
            context
                .tools
                .insert(name.into(), which::which(name).is_ok());
        }
    }
    if include_history {
        context.history = history(history_lines.min(100));
    }
    Ok(context)
}

pub fn shell() -> String {
    if cfg!(windows) {
        env::var("COMSPEC").unwrap_or_else(|_| "cmd.exe".into())
    } else {
        env::var("SHELL").unwrap_or_else(|_| "/bin/sh".into())
    }
}
fn git(args: &[&str]) -> Option<String> {
    let out = Command::new("git")
        .args(["-c", "core.fsmonitor=false"])
        .args(args)
        .env("GIT_OPTIONAL_LOCKS", "0")
        .output()
        .ok()?;
    out.status
        .success()
        .then(|| String::from_utf8_lossy(&out.stdout).trim().into())
}
fn os_version() -> String {
    if cfg!(target_os = "linux") {
        if let Ok(release) = fs::read_to_string("/etc/os-release") {
            if let Some(name) = release.lines().find_map(|s| s.strip_prefix("PRETTY_NAME=")) {
                return format!("linux ({})", name.trim_matches('"'));
            }
        }
    }
    if cfg!(target_os = "macos") {
        if let Ok(out) = Command::new("sw_vers").arg("-productVersion").output() {
            return format!("macOS {}", String::from_utf8_lossy(&out.stdout).trim());
        }
    }
    env::consts::OS.into()
}
fn history(lines: usize) -> Vec<String> {
    if lines == 0 {
        return Vec::new();
    }
    let path = env::var_os("HISTFILE").map(PathBuf::from).or_else(|| {
        dirs::home_dir().map(|h| {
            h.join(if shell().contains("zsh") {
                ".zsh_history"
            } else {
                ".bash_history"
            })
        })
    });
    let read = || -> Option<String> {
        let mut file = fs::File::open(path?).ok()?;
        let length = file.metadata().ok()?.len();
        file.seek(SeekFrom::Start(length.saturating_sub(65536)))
            .ok()?;
        let mut bytes = Vec::new();
        file.take(65536).read_to_end(&mut bytes).ok()?;
        let text = String::from_utf8_lossy(&bytes).into_owned();
        Some(if length > 65536 {
            text.split_once('\n').map(|(_, s)| s).unwrap_or("").into()
        } else {
            text
        })
    };
    let text = read().unwrap_or_default();
    let mut result: Vec<_> = text
        .lines()
        .rev()
        .take(lines)
        .map(|s| s.chars().take(1000).collect())
        .collect();
    result.reverse();
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimal_is_private_by_default() {
        let c = gather(Richness::Minimal, false, 10).unwrap();
        assert!(
            c.entries.is_empty() && c.git.is_none() && c.tools.is_empty() && c.history.is_empty()
        );
    }
    #[test]
    fn bounded_context_without_contents() {
        let c = gather(Richness::Normal, false, 10).unwrap();
        assert!(c.entries.len() <= 40 && c.history.is_empty());
    }
}
