use assert_cmd::Command;
use insta::{assert_snapshot, glob, with_settings};
use lazy_static::lazy_static;
use std::{env, path::PathBuf};

#[test]
fn test() {
    glob!("samples/*/Cargo.toml", |path| {
        let path = path
            .parent()
            .expect("failed to unwrap parent of Cargo.toml");

        let mut cmd = Command::new(BIN.as_os_str());
        let assert = cmd
            .arg("feature-matrix")
            .arg("--color=never")
            .arg("--dry-run")
            .arg("clippy")
            .current_dir(path)
            .assert()
            .success();
        let output = assert.get_output();
        let output = std::str::from_utf8(&output.stdout)
            .expect("child process's output included invalid unicode");

        with_settings!({snapshot_suffix => path.file_name().unwrap().to_str().unwrap()},
        {
            assert_snapshot!(output);
        });
    });
}

#[test]
fn test_no_default_features() {
    glob!("samples/seed/Cargo.toml", |path| {
        let path = path
            .parent()
            .expect("failed to unwrap parent of Cargo.toml");

        let mut cmd = Command::new(BIN.as_os_str());
        let assert = cmd
            .arg("feature-matrix")
            .arg("--color=never")
            .arg("test")
            .current_dir(path)
            .assert()
            .success();
        let output = assert.get_output();
        let output = std::str::from_utf8(&output.stdout)
            .expect("child process's output included invalid unicode");

        with_settings!({snapshot_suffix => path.file_name().unwrap().to_str().unwrap()},
        {
            assert_snapshot!(output);
        });
    });
}

lazy_static! {
    static ref BIN: PathBuf =
        assert_cmd::cargo::cargo_bin("cargo-feature-matrix");
}
