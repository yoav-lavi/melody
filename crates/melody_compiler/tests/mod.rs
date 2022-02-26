#![cfg(test)]

use melody_compiler::compiler;

#[test]
fn quantifier_test() {
    let output = compiler(
        r#"
  5 of "A";
  "#,
    )
    .unwrap();
    assert_eq!(output, "/A{5}/");
}

#[test]
fn capture_test() {
    let output = compiler(
        r#"
      capture {
        5 of "A";
        0 to 9;
      }
      "#,
    )
    .unwrap();
    assert_eq!(output, "/(A{5}[0-9])/");
}

#[test]
fn named_capture_test() {
    let output = compiler(
        r#"
      capture name {
        5 of "A";
        0 to 9;
      }
      "#,
    )
    .unwrap();
    assert_eq!(output, "/(?<name>A{5}[0-9])/");
}

#[test]
fn number_quantifier_range_test() {
    let output = compiler(
        r#"
      1 to 5 of "A";
      "#,
    )
    .unwrap();
    assert_eq!(output, "/A{1,5}/");
}

#[test]
fn uppercase_range_test() {
    let output = compiler(
        r#"
      A to Z;
      "#,
    )
    .unwrap();
    assert_eq!(output, "/[A-Z]/");
}

#[test]
fn lowercase_range_test() {
    let output = compiler(
        r#"
      a to z;
      "#,
    )
    .unwrap();
    assert_eq!(output, "/[a-z]/");
}

#[test]
fn open_range_expression_test() {
    let output = compiler(
        r#"
      over 4 of "a";
      "#,
    )
    .unwrap();
    assert_eq!(output, "/a{5,}/");
}

#[test]
fn start_end_test() {
    let output = compiler(
        r#"
      <start>;
      "a"
      <end>;
      "#,
    )
    .unwrap();
    assert_eq!(output, "/^a$/");
}

#[test]
fn symbol_test() {
    let output = compiler(
        r#"
      <start>;
      <char>;
      <whitespace>;
      not <whitespace>;
      <newline>;
      <tab>;
      <return>;
      <feed>;
      <null>;
      <digit>;
      not <digit>;
      <word>;
      not <word>;
      <vertical>;
      <alphabet>;
      <space>;
      <end>;
      "#,
    )
    .unwrap();
    assert_eq!(output, r"/^.\s\S\n\t\r\f\0\d\D\w\W\v[a-zA-Z] $/");
}

#[test]
fn single_quote_test() {
    let output = compiler(
        r#"
      'hello';
      "#,
    )
    .unwrap();
    assert_eq!(output, "/hello/");
}

#[test]
fn match_test() {
    let output = compiler(
        r#"
      match {
        5 of "A";
        0 to 9;
      }
      "#,
    )
    .unwrap();
    assert_eq!(output, "/(?:A{5}[0-9])/");
}

#[test]
fn comment_test() {
    let output = compiler(
        r#"
      // a single digit in the range of 0 to 5
      0 to 5;
      "#,
    )
    .unwrap();
    assert_eq!(output, "/[0-5]/");
}

#[test]
fn char_test() {
    let output = compiler(
        r#"
      3 of <char>;
      "#,
    )
    .unwrap();
    assert_eq!(output, "/.{3}/");
}

#[test]
fn some_test() {
    let single_output = compiler(
        r#"
      some of <char>;
      "#,
    )
    .unwrap();
    assert_eq!(single_output, "/.+/");
    let multiple_output = compiler(
        r#"
      some of "ABC";
      "#,
    )
    .unwrap();
    assert_eq!(multiple_output, "/(?:ABC)+/");
}

#[test]
fn option_test() {
    let single_output = compiler(
        r#"
      option of <char>;
      "#,
    )
    .unwrap();
    assert_eq!(single_output, "/.?/");
    let multiple_output = compiler(
        r#"
      option of "ABC";
      "#,
    )
    .unwrap();
    assert_eq!(multiple_output, "/(?:ABC)?/");
}

#[test]
fn either_test() {
    let output = compiler(
        r#"
      either {
        "first";
        "second";
        a to z;
      }
      either {
        "first";
        "second";
      }
      "#,
    )
    .unwrap();
    assert_eq!(output, "/(?:first|second|[a-z])(?:first|second)/");
}

#[test]
fn any_test() {
    let single_output = compiler(
        r#"
      any of <char>;
      "#,
    )
    .unwrap();
    assert_eq!(single_output, "/.*/");
    let multiple_output = compiler(
        r#"
        any of "ABC";
      "#,
    )
    .unwrap();
    assert_eq!(multiple_output, "/(?:ABC)*/");
}

#[test]
fn raw_test() {
    let output = compiler(
        r#"
      5 of `.*`
      "#,
    )
    .unwrap();
    assert_eq!(output, "/(?:.*){5}/");
}

#[test]
fn assertion_test() {
    let output = compiler(
        r#"
      5 of ahead {
        "a";
      }
      5 of behind {
        "a";
      }
      5 of not ahead {
        "a";
      }
      5 of not behind {
        "a";
      }
      "#,
    )
    .unwrap();
    assert_eq!(output, "/(?=a){5}(?<=a){5}(?!a){5}(?<!a){5}/");
}
