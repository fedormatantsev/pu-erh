use std::process::Command;

use pu_erh_core::Session;
use tempfile::tempdir;

#[test]
fn cli_smoke_test() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("kb.json");
    let file_arg = file.to_string_lossy().into_owned();

    let mut session = Session::open(&file).unwrap();
    session.save().unwrap();
    let root = session.root_id().unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_pu-erh"))
        .args(["--file", &file_arg, "create", "--parent", &root.to_string()])
        .output()
        .unwrap();
    assert!(output.status.success(), "{:?}", output);
    let child = String::from_utf8(output.stdout).unwrap().trim().to_string();

    let output = Command::new(env!("CARGO_BIN_EXE_pu-erh"))
        .args([
            "--file",
            &file_arg,
            "query",
            &format!("children:{root}"),
        ])
        .output()
        .unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(&child));

    let output = Command::new(env!("CARGO_BIN_EXE_pu-erh"))
        .args([
            "--file",
            &file_arg,
            "query",
            &format!("parent:{child}"),
        ])
        .output()
        .unwrap();
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(&root.to_string()));

    let output = Command::new(env!("CARGO_BIN_EXE_pu-erh"))
        .args([
            "--file",
            &file_arg,
            "move",
            &child,
            "--parent",
            &root.to_string(),
        ])
        .output()
        .unwrap();
    assert!(output.status.success(), "{:?}", output);

    let output = Command::new(env!("CARGO_BIN_EXE_pu-erh"))
        .args(["--file", &file_arg, "delete", &child])
        .output()
        .unwrap();
    assert!(output.status.success(), "{:?}", output);
}
