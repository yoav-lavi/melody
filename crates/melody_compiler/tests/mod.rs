#![cfg(test)]

use melody_compiler::compiler;

#[test]
fn quantifier_test() {
    let output = compiler(
        r#"
  5 of "A";
  "#,
    );
    assert_eq!(output.unwrap(), "/A{5}/");
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
    );
    assert_eq!(output.unwrap(), "/(A{5}[0-9])/");
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
    );
    assert_eq!(output.unwrap(), "/(?<name>A{5}[0-9])/");
}

#[test]
fn number_quantifier_range_test() {
    let output = compiler(
        r#"
      1 to 5 of "A";
      "#,
    );
    assert_eq!(output.unwrap(), "/A{1,5}/");
}

#[test]
fn uppercase_range_test() {
    let output = compiler(
        r#"
      A to Z;
      "#,
    );
    assert_eq!(output.unwrap(), "/[A-Z]/");
}

#[test]
fn lowercase_range_test() {
    let output = compiler(
        r#"
      a to z;
      "#,
    );
    assert_eq!(output.unwrap(), "/[a-z]/");
}

#[test]
fn open_range_expression_test() {
    let output = compiler(
        r#"
      over 4 of "a";
      "#,
    );
    assert_eq!(output.unwrap(), "/a{5,}/");
}

#[test]
fn start_end_test() {
    let output = compiler(
        r#"
      <start>;
      "a"
      <end>;
      "#,
    );
    assert_eq!(output.unwrap(), "/^a$/");
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
    );
    assert_eq!(output.unwrap(), r"/^.\s\S\n\t\r\f\0\d\D\w\W\v[a-zA-Z] $/");
}

#[test]
fn single_quote_test() {
    let output = compiler(
        r#"
      'hello';
      "#,
    );
    assert_eq!(output.unwrap(), "/hello/");
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
    );
    assert_eq!(output.unwrap(), "/(?:A{5}[0-9])/");
}

#[test]
fn comment_test() {
    let output = compiler(
        r#"
      // a single digit in the range of 0 to 5
      0 to 5;
      "#,
    );
    assert_eq!(output.unwrap(), "/[0-5]/");
}

#[test]
fn char_test() {
    let output = compiler(
        r#"
      3 of <char>;
      "#,
    );
    assert_eq!(output.unwrap(), "/.{3}/");
}

#[test]
fn some_test() {
    let single_output = compiler(
        r#"
      some of <char>;
      "#,
    );
    assert_eq!(single_output.unwrap(), "/.+/");
    let multiple_output = compiler(
        r#"
      some of "ABC";
      "#,
    );
    assert_eq!(multiple_output.unwrap(), "/(?:ABC)+/");
}

#[test]
fn option_test() {
    let single_output = compiler(
        r#"
      option of <char>;
      "#,
    );
    assert_eq!(single_output.unwrap(), "/.?/");
    let multiple_output = compiler(
        r#"
      option of "ABC";
      "#,
    );
    assert_eq!(multiple_output.unwrap(), "/(?:ABC)?/");
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
    );
    assert_eq!(output.unwrap(), "/(?:first|second|[a-z])(?:first|second)/");
}

#[test]
fn any_test() {
    let single_output = compiler(
        r#"
      any of <char>;
      "#,
    );
    assert_eq!(single_output.unwrap(), "/.*/");
    let multiple_output = compiler(
        r#"
        any of "ABC";
      "#,
    );
    assert_eq!(multiple_output.unwrap(), "/(?:ABC)*/");
}

#[test]
fn raw_test() {
    let output = compiler(
        r#"
      5 of `.*`
      "#,
    );
    assert_eq!(output.unwrap(), "/(?:.*){5}/");
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
    );
    assert_eq!(output.unwrap(), "/(?=a){5}(?<=a){5}(?!a){5}(?<!a){5}/");
}

#[test]
fn newline_quantifier_test() {
    let output = compiler(
        r#"
        5 of "a"
        "b"
      "#,
    );
    assert_eq!(output.unwrap(), "/a{5}b/");
}
