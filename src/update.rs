use anyhow::{bail, Context, Result};
use std::{
    io::Write,
    path::Path,
    process::{Command, Stdio},
};
const INSTALL_URL: &str = "https://cantrip.fyi/install";
#[derive(Debug, PartialEq, Eq)]
pub enum Method {
    Homebrew,
    Nix,
    Standalone,
}
pub fn method(path: &Path) -> Method {
    let path = path.to_string_lossy();
    if path.starts_with("/nix/store/") {
        Method::Nix
    } else if path.contains("/Cellar/cantrip/") || path.contains("/Cellar/cantrip-cli/") {
        Method::Homebrew
    } else {
        Method::Standalone
    }
}
pub fn run() -> Result<i32> {
    let executable = std::env::current_exe()
        .context("locate installed cantrip")?
        .canonicalize()?;
    match method(&executable) {
        Method::Nix => {
            println!("cantrip is managed by Nix. Update your cantrip flake input and rebuild your configuration.");
            return Ok(0);
        }
        Method::Homebrew => {
            let status = Command::new("brew")
                .args(["upgrade", "flockcap/tap/cantrip"])
                .status()
                .context("run Homebrew upgrade")?;
            return Ok(status.code().unwrap_or(1));
        }
        Method::Standalone => {}
    }
    let directory = executable
        .parent()
        .context("installed binary has no parent directory")?;
    if cfg!(windows) {
        // Windows locks a running executable. Wait for this process to exit,
        // then let the same verified release installer replace the binary.
        let script = format!("Wait-Process -Id {} -ErrorAction SilentlyContinue; $ErrorActionPreference='Stop'; Invoke-RestMethod -Uri '{INSTALL_URL}?shell=powershell' | Invoke-Expression", std::process::id());
        Command::new("powershell.exe")
            .args(["-NoProfile", "-Command", &script])
            .env("CANTRIP_INSTALL_DIR", directory)
            .env("CANTRIP_CLI_NO_MODIFY_PATH", "1")
            .stdin(Stdio::null())
            .spawn()
            .context("start Windows updater")?;
        println!("cantrip update started. The installer will finish after this process exits.");
        return Ok(0);
    }
    let output = if which::which("curl").is_ok() {
        Command::new("curl")
            .args([
                "--proto",
                "=https",
                "--tlsv1.2",
                "-fLsS",
                "--retry",
                "3",
                "--connect-timeout",
                "15",
                "--max-time",
                "180",
                "--max-filesize",
                "1048576",
                INSTALL_URL,
            ])
            .output()?
    } else if which::which("wget").is_ok() {
        Command::new("wget")
            .args(["-qO-", "--https-only", "--timeout=180", INSTALL_URL])
            .output()?
    } else {
        bail!("self-update requires curl or wget; alternatively reinstall from {INSTALL_URL}");
    };
    if !output.status.success() {
        bail!(
            "could not download the updater from {INSTALL_URL}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    if output.stdout.is_empty() || output.stdout.len() > 1_048_576 {
        bail!("invalid installer response size");
    }
    let mut child = Command::new("sh")
        .env("CANTRIP_INSTALL_DIR", directory)
        .env("CANTRIP_CLI_NO_MODIFY_PATH", "1")
        .stdin(Stdio::piped())
        .spawn()?;
    child
        .stdin
        .take()
        .context("updater stdin unavailable")?
        .write_all(&output.stdout)?;
    let status = child.wait()?;
    Ok(status.code().unwrap_or(1))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn updates_respect_package_ownership() {
        assert_eq!(
            method(Path::new("/nix/store/abc-cantrip/bin/cantrip")),
            Method::Nix
        );
        assert_eq!(
            method(Path::new("/opt/homebrew/Cellar/cantrip/0.1.0/bin/cantrip")),
            Method::Homebrew
        );
        assert_eq!(
            method(Path::new("/home/user/.local/bin/cantrip")),
            Method::Standalone
        );
    }
}
