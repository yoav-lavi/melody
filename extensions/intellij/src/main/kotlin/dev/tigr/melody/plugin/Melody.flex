package dev.tigr.melody.plugin;

import com.intellij.lexer.FlexLexer;
import com.intellij.psi.tree.IElementType;

import static com.intellij.psi.TokenType.BAD_CHARACTER;
import static com.intellij.psi.TokenType.WHITE_SPACE;
import static dev.tigr.melody.plugin.psi.MelodyTypes.*;

%%

%{
  public _MelodyLexer() {
    this((java.io.Reader)null);
  }
%}

%public
%class _MelodyLexer
%implements FlexLexer
%function advance
%type IElementType
%unicode

EOL=\R
WHITE_SPACE=\s+

NUMBER=[0-9]+
COMMENT="//".*|"/"\*!(\*"/")*\*"/"|"/"\*\*"/"
STRING=\"[^\"]*\"|'[^']*'|`[^`]*`
CHARACTER=[a-zA-Z0-9_\\@*$#&\^!%]
IDENTIFIER=[a-zA-Z_0-9]+
VARIABLE=\.[a-zA-Z_0-9]+
WHITESPACE=[ \t\n\x0B\f\r]+

%%
<YYINITIAL> {
  {WHITE_SPACE}                             { return WHITE_SPACE; }

  "of"                                      { return OF; }
  "to"                                      { return TO; }
  "capture"                                 { return CAPTURE; }
  "some"                                    { return SOME; }
  "match"                                   { return MATCH; }
  "over"                                    { return OVER; }
  "option"                                  { return OPTION; }
  "not"                                     { return NOT; }
  "either"                                  { return EITHER; }
  "any"                                     { return ANY; }
  "ahead"                                   { return AHEAD; }
  "behind"                                  { return BEHIND; }
  "let"                                     { return LET; }
  "lazy"                                    { return LAZY; }
  "<start>"                                 { return START; }
  "<end>"                                   { return END; }
  "<char>"                                  { return CHAR; }
  "<whitespace>"                            { return WHITESPACELITERAL; }
  "<space>"                                 { return SPACE; }
  "<newline>"                               { return NEWLINE; }
  "<tab>"                                   { return TAB; }
  "<return>"                                { return RETURN; }
  "<feed>"                                  { return FEED; }
  "<null>"                                  { return NULL; }
  "<digit>"                                 { return DIGIT; }
  "<vertical>"                              { return VERTICAL; }
  "<word>"                                  { return WORD; }
  "<alphabetic>"                            { return ALPHABET; }
  "<alphanumeric>"                          { return ALPHANUMERIC; }
  "<boundary>"                              { return BOUNDARY; }
  "<backspace>"                             { return BACKSPACE; }
  "<category::letter>"                      { return LETTER_CATEGORY; }
  "<category::lowercase_letter>"            { return LOWERCASE_LETTER_CATEGORY; }
  "<category::uppercase_letter>"            { return UPPERCASE_LETTER_CATEGORY; }
  "<category::titlecase_letter>"            { return TITLECASE_LETTER_CATEGORY; }
  "<category::cased_letter>"                { return CASED_LETTER_CATEGORY; }
  "<category::modifier_letter>"             { return MODIFIER_LETTER_CATEGORY; }
  "<category::other_letter>"                { return OTHER_LETTER_CATEGORY; }
  "<category::mark>"                        { return MARK_CATEGORY; }
  "<category::non_spacing_mark>"            { return NON_SPACING_MARK_CATEGORY; }
  "<category::spacing_combining_mark>"      { return SPACING_COMBINING_MARK_CATEGORY; }
  "<category::enclosing_mark>"              { return ENCLOSING_MARK_CATEGORY; }
  "<category::separator>"                   { return SEPARATOR_CATEGORY; }
  "<category::space_separator>"             { return SPACE_SEPARATOR_CATEGORY; }
  "<category::line_separator>"              { return LINE_SEPARATOR_CATEGORY; }
  "<category::paragraph_separator>"         { return PARAGRAPH_SEPARATOR_CATEGORY; }
  "<category::symbol>"                      { return SYMBOL_CATEGORY; }
  "<category::math_symbol>"                 { return MATH_SYMBOL_CATEGORY; }
  "<category::currency_symbol>"             { return CURRENCY_SYMBOL_CATEGORY; }
  "<category::modifier_symbol>"             { return MODIFIER_SYMBOL_CATEGORY; }
  "<category::other_symbol>"                { return OTHER_SYMBOL_CATEGORY; }
  "<category::number>"                      { return NUMBER_CATEGORY; }
  "<category::decimal_digit_number>"        { return DECIMAL_DIGIT_NUMBER_CATEGORY; }
  "<category::letter_number>"               { return LETTER_NUMBER_CATEGORY; }
  "<category::other_number>"                { return OTHER_NUMBER_CATEGORY; }
  "<category::punctuation>"                 { return PUNCTUATION_CATEGORY; }
  "<category::dash_punctuation>"            { return DASH_PUNCTUATION_CATEGORY; }
  "<category::open_punctuation>"            { return OPEN_PUNCTUATION_CATEGORY; }
  "<category::close_punctuation>"           { return CLOSE_PUNCTUATION_CATEGORY; }
  "<category::initial_punctuation>"         { return INITIAL_PUNCTUATION_CATEGORY; }
  "<category::final_punctuation>"           { return FINAL_PUNCTUATION_CATEGORY; }
  "<category::connector_punctuation>"       { return CONNECTOR_PUNCTUATION_CATEGORY; }
  "<category::other_punctuation>"           { return OTHER_PUNCTUATION_CATEGORY; }
  "<category::other>"                       { return OTHER_CATEGORY; }
  "<category::control>"                     { return CONTROL_CATEGORY; }
  "<category::format>"                      { return FORMAT_CATEGORY; }
  "<category::private_use>"                 { return PRIVATE_USE_CATEGORY; }
  "<category::surrogate>"                   { return SURROGATE_CATEGORY; }
  "<category::unassigned>"                  { return UNASSIGNED_CATEGORY; }
  "{"                                       { return OPENBRACE; }
  "}"                                       { return CLOSEBRACE; }
  "="                                       { return EQUALS; }
  ";"                                       { return SEMICOLON; }

  {NUMBER}                                  { return NUMBER; }
  {COMMENT}                                 { return COMMENT; }
  {STRING}                                  { return STRING; }
  {CHARACTER}                               { return CHARACTER; }
  {IDENTIFIER}                              { return IDENTIFIER; }
  {VARIABLE}                                { return VARIABLE; }
  {WHITESPACE}                              { return WHITESPACE; }

}

[^] { return BAD_CHARACTER; }
