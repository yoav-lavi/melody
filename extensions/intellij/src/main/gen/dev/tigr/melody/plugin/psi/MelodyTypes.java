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
  IElementType MATCH_RULE = new MelodyElementType("MATCH_RULE");
  IElementType NOT_RULE = new MelodyElementType("NOT_RULE");
  IElementType OF_RULE = new MelodyElementType("OF_RULE");
  IElementType OVER_RULE = new MelodyElementType("OVER_RULE");
  IElementType STRING_RULE = new MelodyElementType("STRING_RULE");
  IElementType SYMBOLS_RULE = new MelodyElementType("SYMBOLS_RULE");
  IElementType TO_RULE = new MelodyElementType("TO_RULE");

  IElementType AHEAD = new MelodyTokenType("ahead");
  IElementType ALPHABET = new MelodyTokenType("<alphabet>");
  IElementType ANY = new MelodyTokenType("any");
  IElementType BEHIND = new MelodyTokenType("behind");
  IElementType CAPTURE = new MelodyTokenType("capture");
  IElementType CHAR = new MelodyTokenType("<char>");
  IElementType CLOSEBRACE = new MelodyTokenType("}");
  IElementType COMMENT = new MelodyTokenType("comment");
  IElementType DIGIT = new MelodyTokenType("<digit>");
  IElementType EITHER = new MelodyTokenType("either");
  IElementType END = new MelodyTokenType("<end>");
  IElementType FEED = new MelodyTokenType("<feed>");
  IElementType MATCH = new MelodyTokenType("match");
  IElementType NEWLINE = new MelodyTokenType("<newline>");
  IElementType NOT = new MelodyTokenType("not");
  IElementType NULL = new MelodyTokenType("<null>");
  IElementType NUMBER = new MelodyTokenType("number");
  IElementType OF = new MelodyTokenType("of");
  IElementType OPENBRACE = new MelodyTokenType("{");
  IElementType OPTION = new MelodyTokenType("option");
  IElementType OVER = new MelodyTokenType("over");
  IElementType RETURN = new MelodyTokenType("<return>");
  IElementType SEMICOLON = new MelodyTokenType(";");
  IElementType SOME = new MelodyTokenType("some");
  IElementType SPACE = new MelodyTokenType("<space>");
  IElementType START = new MelodyTokenType("<start>");
  IElementType STRING = new MelodyTokenType("string");
  IElementType TAB = new MelodyTokenType("<tab>");
  IElementType TO = new MelodyTokenType("to");
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
      else if (type == STRING_RULE) {
        return new MelodyStringRuleImpl(node);
      }
      else if (type == SYMBOLS_RULE) {
        return new MelodySymbolsRuleImpl(node);
      }
      else if (type == TO_RULE) {
        return new MelodyToRuleImpl(node);
      }
      throw new AssertionError("Unknown element type: " + type);
    }
  }
}
