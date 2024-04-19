use assert_cmd::prelude::*; // Add methods on commands
// use assert_fs::prelude::*;
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn init_command() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("adr-rs")?;

    cmd.arg("init");
    cmd.assert()
        .success();
    Ok(())
}

#[test]
fn new_command() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("adr-rs")?;

    cmd.arg("list");
    cmd.assert()
        .success();
        // .stdout(predicate::str::contains("0001-record-architecture-decisions.md"));
    Ok(())
}

#[test]
fn list_command() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("adr-rs")?;

    cmd.arg("list");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("0001-record-architecture-decisions.md"));
    Ok(())
}

// #[test]
// fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
//     let file = assert_fs::NamedTempFile::new("doc/adr/sample.md")?;
//     file.write_str("# 1. 2024-04-19 Record architecture decisions")?;

//     let mut cmd = Command::cargo_bin("adr-rs")?;
//     cmd.arg("list").arg("-d").arg("doc/adr");
//     cmd.assert()
//         .success()
//         .stdout(predicate::str::contains("A test\nAnother test"));

//     Ok(())
// }
