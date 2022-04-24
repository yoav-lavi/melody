#![cfg(test)]
use melody_compiler::compiler;

#[test]
fn quantifier_test() {
    let output = compiler(
        r#"
  5 of "A";
  "#,
    );
    assert_eq!(output.unwrap(), "A{5}");
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
    assert_eq!(output.unwrap(), "(A{5}[0-9])");
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
    assert_eq!(output.unwrap(), "(?<name>A{5}[0-9])");
}

#[test]
fn number_quantifier_range_test() {
    let output = compiler(
        r#"
      1 to 5 of "A";
      "#,
    );
    assert_eq!(output.unwrap(), "A{1,5}");
}

#[test]
fn uppercase_range_test() {
    let output = compiler(
        r#"
      A to Z;
      7 of A to Z;
      "#,
    );
    assert_eq!(output.unwrap(), "[A-Z][A-Z]{7}");
}

#[test]
fn lowercase_range_test() {
    let output = compiler(
        r#"
      a to z;
      8 of a to z;
      "#,
    );
    assert_eq!(output.unwrap(), "[a-z][a-z]{8}");
}

#[test]
fn open_range_expression_test() {
    let output = compiler(
        r#"
      over 4 of "a";
      "#,
    );
    assert_eq!(output.unwrap(), "a{5,}");
}

#[test]
fn start_end_test() {
    let output = compiler(
        r#"
      <start>;
      "a";
      <end>;
      "#,
    );
    assert_eq!(output.unwrap(), "^a$");
}

#[test]
fn symbol_test() {
    let output = compiler(
        r#"
      <start>;
      <char>;
      not <char>;
      <whitespace>;
      not <whitespace>;
      <newline>;
      not <newline>;
      <tab>;
      not <tab>;
      <return>;
      not <return>;
      <feed>;
      not <feed>;
      <null>;
      not <null>;
      <digit>;
      not <digit>;
      <word>;
      not <word>;
      <vertical>;
      not <vertical>;
      <alphabetic>;
      not <alphabetic>;
      <alphanumeric>;
      not <alphanumeric>;
      <space>;
      not <space>;
      <boundary>;
      not <boundary>;
      <backspace>;
      not <backspace>;
      <end>;
      "#,
    );
    assert_eq!(
        output.unwrap(),
        r"^.[^.]\s\S\n[^\n]\t[^\t]\r[^\r]\f[^\f]\0[^\0]\d\D\w\W\v[^\v][a-zA-Z][^a-zA-Z][a-zA-Z0-9][^a-zA-Z0-9] [^ ]\b\B[\b][^\b]$"
    );
}

#[test]
fn symbol_unicode_category_test() {
    let output = compiler(
        r#"
        <category::cased_letter>;
        <category::close_punctuation>;
        <category::connector_punctuation>;
        <category::control>;
        <category::currency_symbol>;
        <category::dash_punctuation>;
        <category::decimal_digit_number>;
        <category::enclosing_mark>;
        <category::final_punctuation>;
        <category::format>;
        <category::initial_punctuation>;
        <category::letter_number>;
        <category::letter>;
        <category::line_separator>;
        <category::lowercase_letter>;
        <category::mark>;
        <category::math_symbol>;
        <category::modifier_letter>;
        <category::modifier_symbol>;
        <category::non_spacing_mark>;
        <category::number>;
        <category::open_punctuation>;
        <category::other_letter>;
        <category::other_number>;
        <category::other_punctuation>;
        <category::other_symbol>;
        <category::other>;
        <category::paragraph_separator>;
        <category::private_use>;
        <category::punctuation>;
        <category::separator>;
        <category::space_separator>;
        <category::spacing_combining_mark>;
        <category::surrogate>;
        <category::symbol>;
        <category::titlecase_letter>;
        <category::unassigned>;
        <category::uppercase_letter>;
        not <category::cased_letter>;
        not <category::close_punctuation>;
        not <category::connector_punctuation>;
        not <category::control>;
        not <category::currency_symbol>;
        not <category::dash_punctuation>;
        not <category::decimal_digit_number>;
        not <category::enclosing_mark>;
        not <category::final_punctuation>;
        not <category::format>;
        not <category::initial_punctuation>;
        not <category::letter_number>;
        not <category::letter>;
        not <category::line_separator>;
        not <category::lowercase_letter>;
        not <category::mark>;
        not <category::math_symbol>;
        not <category::modifier_letter>;
        not <category::modifier_symbol>;
        not <category::non_spacing_mark>;
        not <category::number>;
        not <category::open_punctuation>;
        not <category::other_letter>;
        not <category::other_number>;
        not <category::other_punctuation>;
        not <category::other_symbol>;
        not <category::other>;
        not <category::paragraph_separator>;
        not <category::private_use>;
        not <category::punctuation>;
        not <category::separator>;
        not <category::space_separator>;
        not <category::spacing_combining_mark>;
        not <category::surrogate>;
        not <category::symbol>;
        not <category::titlecase_letter>;
        not <category::unassigned>;
        not <category::uppercase_letter>;
        5 of <category::cased_letter>;
      "#,
    );
    assert_eq!(
        output.unwrap(),
        r"\p{L&}\p{Pe}\p{Pc}\p{Cc}\p{Sc}\p{Pd}\p{Nd}\p{Me}\p{Pf}\p{Cf}\p{Pi}\p{Nl}\p{L}\p{Zl}\p{Ll}\p{M}\p{Sm}\p{Lm}\p{Sk}\p{Mn}\p{N}\p{Ps}\p{Lo}\p{No}\p{Po}\p{So}\p{C}\p{Zp}\p{Co}\p{P}\p{Z}\p{Zs}\p{Mc}\p{Cs}\p{S}\p{Lt}\p{Cn}\p{Lu}\P{L&}\P{Pe}\P{Pc}\P{Cc}\P{Sc}\P{Pd}\P{Nd}\P{Me}\P{Pf}\P{Cf}\P{Pi}\P{Nl}\P{L}\P{Zl}\P{Ll}\P{M}\P{Sm}\P{Lm}\P{Sk}\P{Mn}\P{N}\P{Ps}\P{Lo}\P{No}\P{Po}\P{So}\P{C}\P{Zp}\P{Co}\P{P}\P{Z}\P{Zs}\P{Mc}\P{Cs}\P{S}\P{Lt}\P{Cn}\P{Lu}\p{L&}{5}"
    );
}

#[test]
fn match_test() {
    let output = compiler(
        r#"
3 of match {
  5 of "A";
  0 to 9;
}"#,
    );
    assert_eq!(output.unwrap(), "(?:A{5}[0-9]){3}");
}

#[test]
fn comment_test() {
    let output = compiler(
        r#"/*  a single digit in the range of 0 to 5 */ 
    // other comment
    0 to 5; 
    match { 
      "x";
    } 
    "#,
    );
    assert_eq!(output.unwrap(), "[0-5](?:x)");
}

#[test]
fn char_test() {
    let output = compiler(
        r#"
      3 of <char>;
      "#,
    );
    assert_eq!(output.unwrap(), ".{3}");
}

#[test]
fn negative_range_test() {
    let output = compiler(
        r#"
      not 3 to 5;
      not a to z;
      "#,
    );
    assert_eq!(output.unwrap(), "[^3-5][^a-z]");
}

#[test]
fn some_test() {
    let single_output = compiler(
        r#"
      some of <char>;
      "#,
    );
    assert_eq!(single_output.unwrap(), ".+");
    let multiple_output = compiler(
        r#"
      some of "ABC";
      "#,
    );
    assert_eq!(multiple_output.unwrap(), "(?:ABC)+");
}

#[test]
fn option_test() {
    let single_output = compiler(
        r#"
      option of <char>;
      "#,
    );
    assert_eq!(single_output.unwrap(), ".?");
    let multiple_output = compiler(
        r#"
      option of "ABC";
      "#,
    );
    assert_eq!(multiple_output.unwrap(), "(?:ABC)?");
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
    assert_eq!(output.unwrap(), "(?:first|second|[a-z])(?:first|second)");
}

#[test]
fn any_test() {
    let single_output = compiler(
        r#"
      any of <char>;
      "#,
    );
    assert_eq!(single_output.unwrap(), ".*");
    let multiple_output = compiler(
        r#"
        any of "ABC";
      "#,
    );
    assert_eq!(multiple_output.unwrap(), "(?:ABC)*");
}

#[test]
fn directly_quantifiable() {
    let single_output = compiler(
        r#"
      5 of <word>;
      "#,
    );
    assert_eq!(single_output.unwrap(), r"\w{5}");
}

#[test]
fn raw_test() {
    let output = compiler(
        r#"
      5 of `.*\``;
      "#,
    );
    assert_eq!(output.unwrap(), "(?:.*`){5}");
}

#[test]
fn assertion_test() {
    let output = compiler(
        r#"
      ahead {
        "a";
      }
      behind {
        "a";
      }
      not ahead {
        "a";
      }
      not behind {
        "a";
      }
      "#,
    );
    assert_eq!(output.unwrap(), "(?=a)(?<=a)(?!a)(?<!a)");
}

#[test]
fn negative_char_class_test() {
    let output = compiler(
        r#"
        not abcd;
        5 of not abcd;
        "#,
    );
    assert_eq!(output.unwrap(), "[^abcd][^abcd]{5}");
}

#[test]
fn single_quote_test() {
    let output = compiler(
        r#"
        'hello \'quoted\'';
        "#,
    );
    assert_eq!(output.unwrap(), "hello 'quoted'");
}

#[test]
fn double_quote_test() {
    let output = compiler(
        r#"
        "hello \"quoted\"";
        "#,
    );
    assert_eq!(output.unwrap(), "hello \"quoted\"");
}

#[test]
fn auto_escape_test() {
    let output = compiler(
        r#"
        "[](){}*+?|^$.-\\";
        "#,
    );
    assert_eq!(output.unwrap(), r"\[\]\(\)\{\}\*\+\?\|\^\$\.\-\\\\");
}

#[test]
fn lazy_test() {
    let output = compiler(
        r#"
        lazy any of "A";
        lazy some of "A";
        lazy option of "A";
        lazy 5 of "A";
        lazy over 5 of "A";
        lazy 5 to 6 of "A";
        "#,
    );
    assert_eq!(output.unwrap(), r"A*?A+?A??A{5}?A{6,}?A{5,6}?");
}

#[test]
fn variable_test() {
    let output = compiler(
        r#"
        let .test_variable = {
          "A";
          "B";
        }
        let .second_test_variable = {
          "C";
        }
        .test_variable;
        .second_test_variable;
        "#,
    );
    assert_eq!(output.unwrap(), r"ABC");
}
