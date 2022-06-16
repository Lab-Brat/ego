use std::fs;
use assert_cmd::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("ego")?;
    cmd.arg("-o").assert().success();
    Ok(())
}

#[test]
fn runs() -> TestResult {
    let mut cmd = Command::cargo_bin("ego")?;
    cmd.arg("hello").assert().success();
    Ok(())
}

#[test] 
fn hello1() -> TestResult {
    let outfile = "tests/expected/hello1.txt";
    let expected = fs::read_to_string(outfile)?;
    let mut cmd = Command::cargo_bin("ego")?;
    cmd.arg("Hello there").assert().success().stdout(expected);
    Ok(())
}

