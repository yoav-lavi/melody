#![cfg(test)]

mod utils;

use std::fs::read_to_string;

use assert_cmd::Command;
use assert_fs::{fixture::FileWriteStr, NamedTempFile};
use tempfile::tempdir;
use unindent::unindent;
use utils::TestResult;

#[test]
fn cli_stdout_test() -> TestResult {
    let mut command = Command::cargo_bin("melody")?;
    let melody_file = NamedTempFile::new("test.mdy")?;

    let source = r#"
      /* comment */

      5 of 'A\'';

      match {
        capture test {
          "x";
          "y";
        }
      }

      `[abc]`;

      // comment
      either {
        "a\"";
        "b";
        <char>;
      }

      some of "a";
    "#;

    let expected_output = "(?:A'){5}(?:(?<test>xy))[abc](?:a\"|b|.)a+";

    melody_file.write_str(&unindent(source))?;

    command
        .arg(melody_file.path())
        .assert()
        .stdout(expected_output);

    Ok(())
}

#[test]
fn cli_file_test() -> TestResult {
    let mut command = Command::cargo_bin("melody")?;
    let melody_file = NamedTempFile::new("test.mdy")?;
    let dir = tempdir()?;

    let source = r#"
    /* comment */

    5 of 'A\'';

    match {
      capture test {
        "x";
        "y";
      }
    }

    `[abc]`;

    // comment
    either {
      "a\"";
      "b";
      <char>;
    }

    some of "a";
  "#;

    let expected_output = "(?:A'){5}(?:(?<test>xy))[abc](?:a\"|b|.)a+";

    melody_file.write_str(&unindent(source))?;

    let file_path = dir.path().join("output.txt");

    command
        .arg(melody_file.path())
        .arg("-o")
        .arg(&file_path)
        .assert()
        .success();

    let output = read_to_string(file_path)?;

    assert_eq!(output, expected_output);

    Ok(())
}
