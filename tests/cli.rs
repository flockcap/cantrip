#![cfg(unix)]
use std::{
    fs,
    os::unix::fs::PermissionsExt,
    path::Path,
    process::{Command, Output},
};
fn invoke(root: &Path, args: &[&str], response: &str) -> Output {
    let bin = root.join("bin");
    fs::create_dir_all(&bin).unwrap();
    let path = bin.join("claude");
    fs::write(
        &path,
        format!(
            "#!{}\nwhile IFS= read -r line; do :; done\nprintf '%s' \"$CANTRIP_TEST_RESPONSE\"\n",
            test_shell()
        ),
    )
    .unwrap();
    let script = fs::read_to_string(&path)
        .unwrap()
        .replace("#!/bin/sh", &format!("#!{}", test_shell()))
        .replace("/bin/cat >/dev/null", "while IFS= read -r line; do :; done");
    fs::write(&path, script).unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
    Command::new(env!("CARGO_BIN_EXE_cantrip"))
        .args(args)
        .env("PATH", &bin)
        .env("XDG_CONFIG_HOME", root.join("config"))
        .env("SHELL", test_shell())
        .env("CANTRIP_TEST_RESPONSE", response)
        .current_dir(root)
        .output()
        .unwrap()
}
fn response(command: &str, danger: &str) -> String {
    serde_json::json!({"result":serde_json::json!({"command":command,"explanation":"fixture","danger":danger,"alternatives":[]}).to_string()}).to_string()
}
#[test]
fn dry_run_and_json_never_execute_even_with_force() {
    for mode in ["--dry-run", "--json"] {
        let root = tempfile::tempdir().unwrap();
        let out = invoke(
            root.path(),
            &["--yolo", "--force", mode, "make marker"],
            &response("printf changed > marker", "high"),
        );
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(!root.path().join("marker").exists());
    }
}
#[test]
fn high_danger_yolo_requires_force_and_noninteractive_confirm_fails_closed() {
    let root = tempfile::tempdir().unwrap();
    let json = response("printf changed > marker", "high");
    let out = invoke(root.path(), &["--yolo", "make marker"], &json);
    assert!(!out.status.success());
    assert!(!root.path().join("marker").exists());
    let out = invoke(root.path(), &["--yolo", "--force", "make marker"], &json);
    assert!(out.status.success());
    assert_eq!(
        fs::read_to_string(root.path().join("marker")).unwrap(),
        "changed"
    );
}
#[test]
fn local_safety_overrides_model_and_command_exit_status_is_preserved() {
    let root = tempfile::tempdir().unwrap();
    let out = invoke(
        root.path(),
        &["--json", "delete files"],
        &response("rm -rf /", "low"),
    );
    let value: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(value["danger"], "high");
    assert!(!value["warnings"].as_array().unwrap().is_empty());
    let out = invoke(
        root.path(),
        &["--yolo", "exit seven"],
        &response("exit 7", "low"),
    );
    assert_eq!(out.status.code(), Some(7));
}
#[test]
fn degraded_output_does_not_become_executable_and_force_requires_yolo() {
    let root = tempfile::tempdir().unwrap();
    let out = invoke(root.path(), &["--json", "wish"], "not JSON");
    assert_eq!(out.status.code(), Some(2));
    let value: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(value["command"], "");
    assert_eq!(value["raw"], "not JSON");
    assert!(
        !invoke(root.path(), &["--force", "wish"], &response("true", "low"))
            .status
            .success()
    );
}
#[test]
fn agent_cannot_load_project_configuration_from_the_request_directory() {
    let root = tempfile::tempdir().unwrap();
    let bin = root.path().join("bin");
    fs::create_dir(&bin).unwrap();
    let marker = root.path().join("cwd");
    let path = bin.join("codex");
    fs::write(&path,"#!/bin/sh\npwd > \"$CANTRIP_CWD_LOG\"\nwhile IFS= read -r line; do :; done\nprintf '%s' '{\"type\":\"item.completed\",\"item\":{\"type\":\"agent_message\",\"text\":\"{\\\"command\\\":\\\"echo hello\\\",\\\"explanation\\\":\\\"hello\\\",\\\"danger\\\":\\\"low\\\",\\\"alternatives\\\":[]}\"}}'\n").unwrap();
    let script = fs::read_to_string(&path)
        .unwrap()
        .replace("#!/bin/sh", &format!("#!{}", test_shell()))
        .replace("/bin/cat >/dev/null", "while IFS= read -r line; do :; done");
    fs::write(&path, script).unwrap();
    fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_cantrip"))
        .args(["--agent", "codex", "--json", "hello"])
        .env("PATH", bin)
        .env("CANTRIP_CWD_LOG", &marker)
        .env("XDG_CONFIG_HOME", root.path().join("config"))
        .current_dir(root.path())
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_ne!(
        fs::read_to_string(marker).unwrap().trim(),
        root.path().to_str().unwrap()
    );
}

fn test_shell() -> String {
    std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".into())
}
