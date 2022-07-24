// This is a generated file. Not intended for manual editing.
package dev.tigr.melody.plugin.psi;

import com.intellij.psi.tree.IElementType;
import com.intellij.psi.PsiElement;
import com.intellij.lang.ASTNode;
import dev.tigr.melody.plugin.psi.impl.*;

public interface MelodyTypes {

  IElementType AHEAD_RULE = new MelodyElementType("AHEAD_RULE");
  IElementType BEHIND_RULE = new MelodyElementType("BEHIND_RULE");
  IElementType CAPTURE_RULE = new MelodyElementType("CAPTURE_RULE");
  IElementType EITHER_RULE = new MelodyElementType("EITHER_RULE");
  IElementType EXPRESSION = new MelodyElementType("EXPRESSION");
  IElementType LET_RULE = new MelodyElementType("LET_RULE");
  IElementType MATCH_RULE = new MelodyElementType("MATCH_RULE");
  IElementType NOT_RULE = new MelodyElementType("NOT_RULE");
  IElementType OF_RULE = new MelodyElementType("OF_RULE");
  IElementType OVER_RULE = new MelodyElementType("OVER_RULE");
  IElementType RANGE_RULE = new MelodyElementType("RANGE_RULE");
  IElementType STRING_RULE = new MelodyElementType("STRING_RULE");
  IElementType SYMBOLS_RULE = new MelodyElementType("SYMBOLS_RULE");
  IElementType TO_RULE = new MelodyElementType("TO_RULE");
  IElementType VARIABLE_RULE = new MelodyElementType("VARIABLE_RULE");

  IElementType AHEAD = new MelodyTokenType("ahead");
  IElementType ALPHABET = new MelodyTokenType("<alphabetic>");
  IElementType ALPHANUMERIC = new MelodyTokenType("<alphanumeric>");
  IElementType ANY = new MelodyTokenType("any");
  IElementType BACKSPACE = new MelodyTokenType("<backspace>");
  IElementType BEHIND = new MelodyTokenType("behind");
  IElementType BOUNDARY = new MelodyTokenType("<boundary>");
  IElementType CAPTURE = new MelodyTokenType("capture");
  IElementType CASED_LETTER_CATEGORY = new MelodyTokenType("<category::cased_letter>");
  IElementType CHAR = new MelodyTokenType("<char>");
  IElementType CHARACTER = new MelodyTokenType("character");
  IElementType CLOSEBRACE = new MelodyTokenType("}");
  IElementType CLOSE_PUNCTUATION_CATEGORY = new MelodyTokenType("<category::close_punctuation>");
  IElementType COMMENT = new MelodyTokenType("comment");
  IElementType CONNECTOR_PUNCTUATION_CATEGORY = new MelodyTokenType("<category::connector_punctuation>");
  IElementType CONTROL_CATEGORY = new MelodyTokenType("<category::control>");
  IElementType CURRENCY_SYMBOL_CATEGORY = new MelodyTokenType("<category::currency_symbol>");
  IElementType DASH_PUNCTUATION_CATEGORY = new MelodyTokenType("<category::dash_punctuation>");
  IElementType DECIMAL_DIGIT_NUMBER_CATEGORY = new MelodyTokenType("<category::decimal_digit_number>");
  IElementType DIGIT = new MelodyTokenType("<digit>");
  IElementType EITHER = new MelodyTokenType("either");
  IElementType ENCLOSING_MARK_CATEGORY = new MelodyTokenType("<category::enclosing_mark>");
  IElementType END = new MelodyTokenType("<end>");
  IElementType EQUALS = new MelodyTokenType("=");
  IElementType FEED = new MelodyTokenType("<feed>");
  IElementType FINAL_PUNCTUATION_CATEGORY = new MelodyTokenType("<category::final_punctuation>");
  IElementType FORMAT_CATEGORY = new MelodyTokenType("<category::format>");
  IElementType IDENTIFIER = new MelodyTokenType("identifier");
  IElementType INITIAL_PUNCTUATION_CATEGORY = new MelodyTokenType("<category::initial_punctuation>");
  IElementType LAZY = new MelodyTokenType("lazy");
  IElementType LET = new MelodyTokenType("let");
  IElementType LETTER_CATEGORY = new MelodyTokenType("<category::letter>");
  IElementType LETTER_NUMBER_CATEGORY = new MelodyTokenType("<category::letter_number>");
  IElementType LINE_SEPARATOR_CATEGORY = new MelodyTokenType("<category::line_separator>");
  IElementType LOWERCASE_LETTER_CATEGORY = new MelodyTokenType("<category::lowercase_letter>");
  IElementType MARK_CATEGORY = new MelodyTokenType("<category::mark>");
  IElementType MATCH = new MelodyTokenType("match");
  IElementType MATH_SYMBOL_CATEGORY = new MelodyTokenType("<category::math_symbol>");
  IElementType MODIFIER_LETTER_CATEGORY = new MelodyTokenType("<category::modifier_letter>");
  IElementType MODIFIER_SYMBOL_CATEGORY = new MelodyTokenType("<category::modifier_symbol>");
  IElementType NEWLINE = new MelodyTokenType("<newline>");
  IElementType NON_SPACING_MARK_CATEGORY = new MelodyTokenType("<category::non_spacing_mark>");
  IElementType NOT = new MelodyTokenType("not");
  IElementType NULL = new MelodyTokenType("<null>");
  IElementType NUMBER = new MelodyTokenType("number");
  IElementType NUMBER_CATEGORY = new MelodyTokenType("<category::number>");
  IElementType OF = new MelodyTokenType("of");
  IElementType OPENBRACE = new MelodyTokenType("{");
  IElementType OPEN_PUNCTUATION_CATEGORY = new MelodyTokenType("<category::open_punctuation>");
  IElementType OPTION = new MelodyTokenType("option");
  IElementType OTHER_CATEGORY = new MelodyTokenType("<category::other>");
  IElementType OTHER_LETTER_CATEGORY = new MelodyTokenType("<category::other_letter>");
  IElementType OTHER_NUMBER_CATEGORY = new MelodyTokenType("<category::other_number>");
  IElementType OTHER_PUNCTUATION_CATEGORY = new MelodyTokenType("<category::other_punctuation>");
  IElementType OTHER_SYMBOL_CATEGORY = new MelodyTokenType("<category::other_symbol>");
  IElementType OVER = new MelodyTokenType("over");
  IElementType PARAGRAPH_SEPARATOR_CATEGORY = new MelodyTokenType("<category::paragraph_separator>");
  IElementType PRIVATE_USE_CATEGORY = new MelodyTokenType("<category::private_use>");
  IElementType PUNCTUATION_CATEGORY = new MelodyTokenType("<category::punctuation>");
  IElementType RETURN = new MelodyTokenType("<return>");
  IElementType SEMICOLON = new MelodyTokenType(";");
  IElementType SEPARATOR_CATEGORY = new MelodyTokenType("<category::separator>");
  IElementType SOME = new MelodyTokenType("some");
  IElementType SPACE = new MelodyTokenType("<space>");
  IElementType SPACE_SEPARATOR_CATEGORY = new MelodyTokenType("<category::space_separator>");
  IElementType SPACING_COMBINING_MARK_CATEGORY = new MelodyTokenType("<category::spacing_combining_mark>");
  IElementType START = new MelodyTokenType("<start>");
  IElementType STRING = new MelodyTokenType("string");
  IElementType SURROGATE_CATEGORY = new MelodyTokenType("<category::surrogate>");
  IElementType SYMBOL_CATEGORY = new MelodyTokenType("<category::symbol>");
  IElementType TAB = new MelodyTokenType("<tab>");
  IElementType TITLECASE_LETTER_CATEGORY = new MelodyTokenType("<category::titlecase_letter>");
  IElementType TO = new MelodyTokenType("to");
  IElementType UNASSIGNED_CATEGORY = new MelodyTokenType("<category::unassigned>");
  IElementType UPPERCASE_LETTER_CATEGORY = new MelodyTokenType("<category::uppercase_letter>");
  IElementType VARIABLE = new MelodyTokenType("variable");
  IElementType VERTICAL = new MelodyTokenType("<vertical>");
  IElementType WHITESPACELITERAL = new MelodyTokenType("<whitespace>");
  IElementType WORD = new MelodyTokenType("<word>");

  class Factory {
    public static PsiElement createElement(ASTNode node) {
      IElementType type = node.getElementType();
      if (type == AHEAD_RULE) {
        return new MelodyAheadRuleImpl(node);
      }
      else if (type == BEHIND_RULE) {
        return new MelodyBehindRuleImpl(node);
      }
      else if (type == CAPTURE_RULE) {
        return new MelodyCaptureRuleImpl(node);
      }
      else if (type == EITHER_RULE) {
        return new MelodyEitherRuleImpl(node);
      }
      else if (type == EXPRESSION) {
        return new MelodyExpressionImpl(node);
      }
      else if (type == LET_RULE) {
        return new MelodyLetRuleImpl(node);
      }
      else if (type == MATCH_RULE) {
        return new MelodyMatchRuleImpl(node);
      }
      else if (type == NOT_RULE) {
        return new MelodyNotRuleImpl(node);
      }
      else if (type == OF_RULE) {
        return new MelodyOfRuleImpl(node);
      }
      else if (type == OVER_RULE) {
        return new MelodyOverRuleImpl(node);
      }
      else if (type == RANGE_RULE) {
        return new MelodyRangeRuleImpl(node);
      }
      else if (type == STRING_RULE) {
        return new MelodyStringRuleImpl(node);
      }
      else if (type == SYMBOLS_RULE) {
        return new MelodySymbolsRuleImpl(node);
      }
      else if (type == TO_RULE) {
        return new MelodyToRuleImpl(node);
      }
      else if (type == VARIABLE_RULE) {
        return new MelodyVariableRuleImpl(node);
      }
      throw new AssertionError("Unknown element type: " + type);
    }
  }
}
