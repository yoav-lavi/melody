#![cfg(test)]

use assert_cmd::Command;
use assert_fs::{fixture::FileWriteStr, NamedTempFile};
use std::fs::read_to_string;
use tempfile::tempdir;
use unindent::unindent;

#[test]
#[cfg_attr(miri, ignore)]
fn cli_file_stdout_test() -> anyhow::Result<()> {
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
#[cfg_attr(miri, ignore)]
fn cli_stdin_stdout_test() -> anyhow::Result<()> {
    let mut command = Command::cargo_bin("melody")?;

    let source = r#"
    some of "a";
    some of "b";
    "#;

    let expected_output = "a+b+";

    command
        .write_stdin(source)
        .arg("-")
        .assert()
        .stdout(expected_output);

    Ok(())
}

#[test]
#[cfg_attr(miri, ignore)]
fn cli_stdin_stdout_no_hyphen_test() -> anyhow::Result<()> {
    let mut command = Command::cargo_bin("melody")?;

    let source = r#"
    some of "a";
    some of "b";
    "#;

    let expected_output = "a+b+";

    command.write_stdin(source).assert().stdout(expected_output);

    Ok(())
}

#[test]
#[cfg_attr(miri, ignore)]
fn cli_file_test() -> anyhow::Result<()> {
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
