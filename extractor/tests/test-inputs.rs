use std::path::Path;
use std::process::{Command, Stdio};

#[test]
fn everything_sandwich() -> anyhow::Result<()> {
    run_moonwave("passing/everything_sandwich.lua", 0)
}

#[test]
fn class_with_function() -> anyhow::Result<()> {
    run_moonwave("passing/class_with_function.lua", 0)
}

#[test]
fn fabric() -> anyhow::Result<()> {
    run_moonwave("passing/fabric.lua", 0)
}

#[test]
fn all_tags() -> anyhow::Result<()> {
    run_moonwave("passing/all_tags.lua", 0)
}

#[test]
fn indentation() -> anyhow::Result<()> {
    run_moonwave("passing/indentation.lua", 0)
}

#[test]
fn failing_function() -> anyhow::Result<()> {
    run_moonwave("failing/function.lua", 1)
}

#[test]
fn missing_starting_newline() -> anyhow::Result<()> {
    run_moonwave("failing/missing_starting_newline.lua", 1)
}

#[test]
fn failing_function_no_within() -> anyhow::Result<()> {
    run_moonwave("failing/function_no_within.lua", 1)
}

#[test]
fn class_with_unused_tags() -> anyhow::Result<()> {
    run_moonwave("failing/class_with_unused_tags.lua", 1)
}

#[test]
fn unknown_tags() -> anyhow::Result<()> {
    run_moonwave("failing/unknown_tags.lua", 1)
}

fn run_moonwave(file_name: &str, expected_status: i32) -> anyhow::Result<()> {
    let path = Path::new("test-input").join(file_name);

    let child = Command::new(env!("CARGO_BIN_EXE_moonwave-extractor"))
        .arg("extract")
        .arg(path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env("NO_COLOR", "1")
        .spawn()?;

    let output = child.wait_with_output()?;
    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;

    let stdout_name = format!("{}-stdout", file_name);
    let stderr_name = format!("{}-stderr", file_name);

    let status_code = output.status.code();

    if status_code != Some(expected_status) {
        eprint!("{}", stderr);
        panic!(
            "Expected status code {}, but got {}",
            expected_status,
            status_code.map_or("none".to_owned(), |c| c.to_string())
        )
    }

    insta::assert_snapshot!(stdout_name, stdout);
    insta::assert_snapshot!(stderr_name, stderr);

    Ok(())
}
