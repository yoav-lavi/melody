// This is a generated file. Not intended for manual editing.
package dev.tigr.melody.plugin.parser;

import com.intellij.lang.PsiBuilder;
import com.intellij.lang.PsiBuilder.Marker;
import static dev.tigr.melody.plugin.psi.MelodyTypes.*;
import static com.intellij.lang.parser.GeneratedParserUtilBase.*;
import com.intellij.psi.tree.IElementType;
import com.intellij.lang.ASTNode;
import com.intellij.psi.tree.TokenSet;
import com.intellij.lang.PsiParser;
import com.intellij.lang.LightPsiParser;

@SuppressWarnings({"SimplifiableIfStatement", "UnusedAssignment"})
public class MelodyParser implements PsiParser, LightPsiParser {

  public ASTNode parse(IElementType t, PsiBuilder b) {
    parseLight(t, b);
    return b.getTreeBuilt();
  }

  public void parseLight(IElementType t, PsiBuilder b) {
    boolean r;
    b = adapt_builder_(t, b, this, null);
    Marker m = enter_section_(b, 0, _COLLAPSE_, null);
    r = parse_root_(t, b);
    exit_section_(b, 0, m, t, r, true, TRUE_CONDITION);
  }

  protected boolean parse_root_(IElementType t, PsiBuilder b) {
    return parse_root_(t, b, 0);
  }

  static boolean parse_root_(IElementType t, PsiBuilder b, int l) {
    return root(b, l + 1);
  }

  /* ********************************************************** */
  // ahead block_rule
  public static boolean ahead_rule(PsiBuilder b, int l) {
    if (!recursion_guard_(b, l, "ahead_rule")) return false;
    if (!nextTokenIs(b, AHEAD)) return false;
    boolean r;
    Marker m = enter_section_(b);
    r = consumeToken(b, AHEAD);
    r = r && block_rule(b, l + 1);
    exit_section_(b, m, AHEAD_RULE, r);
    return r;
  }

  /* ********************************************************** */
  // behind block_rule
  public static boolean behind_rule(PsiBuilder b, int l) {
    if (!recursion_guard_(b, l, "behind_rule")) return false;
    if (!nextTokenIs(b, BEHIND)) return false;
    boolean r;
    Marker m = enter_section_(b);
    r = consumeToken(b, BEHIND);
    r = r && block_rule(b, l + 1);
    exit_section_(b, m, BEHIND_RULE, r);
    return r;
  }

  /* ********************************************************** */
  // openbrace rules closebrace | openbrace closebrace
  static boolean block_rule(PsiBuilder b, int l) {
    if (!recursion_guard_(b, l, "block_rule")) return false;
    if (!nextTokenIs(b, OPENBRACE)) return false;
    boolean r;
    Marker m = enter_section_(b);
    r = block_rule_0(b, l + 1);
    if (!r) r = parseTokens(b, 0, OPENBRACE, CLOSEBRACE);
    exit_section_(b, m, null, r);
    return r;
  }

  // openbrace rules closebrace
  private static boolean block_rule_0(PsiBuilder b, int l) {
    if (!recursion_guard_(b, l, "block_rule_0")) return false;
    boolean r;
    Marker m = enter_section_(b);
    r = consumeToken(b, OPENBRACE);
    r = r && rules(b, l + 1);
    r = r && consumeToken(b, CLOSEBRACE);
    exit_section_(b, m, null, r);
    return r;
  }

  /* ********************************************************** */
  // capture block_rule
  public static boolean capture_rule(PsiBuilder b, int l) {
    if (!recursion_guard_(b, l, "capture_rule")) return false;
    if (!nextTokenIs(b, CAPTURE)) return false;
    boolean r;
    Marker m = enter_section_(b);
    r = consumeToken(b, CAPTURE);
    r = r && block_rule(b, l + 1);
    exit_section_(b, m, CAPTURE_RULE, r);
    return r;
  }

  /* ********************************************************** */
  // either block_rule
  public static boolean either_rule(PsiBuilder b, int l) {
    if (!recursion_guard_(b, l, "either_rule")) return false;
    if (!nextTokenIs(b, EITHER)) return false;
    boolean r;
    Marker m = enter_section_(b);
    r = consumeToken(b, EITHER);
    r = r && block_rule(b, l + 1);
    exit_section_(b, m, EITHER_RULE, r);
    return r;
  }

  /* ********************************************************** */
  // comment | string_rule | not_rule | symbols_rule | match_rule | capture_rule | either_rule | ahead_rule | behind_rule
  public static boolean expression(PsiBuilder b, int l) {
    if (!recursion_guard_(b, l, "expression")) return false;
    boolean r;
    Marker m = enter_section_(b, l, _NONE_, EXPRESSION, "<expression>");
    r = consumeToken(b, COMMENT);
    if (!r) r = string_rule(b, l + 1);
    if (!r) r = not_rule(b, l + 1);
    if (!r) r = symbols_rule(b, l + 1);
    if (!r) r = match_rule(b, l + 1);
    if (!r) r = capture_rule(b, l + 1);
    if (!r) r = either_rule(b, l + 1);
    if (!r) r = ahead_rule(b, l + 1);
    if (!r) r = behind_rule(b, l + 1);
    exit_section_(b, l, m, r, false, null);
    return r;
  }

  /* ********************************************************** */
  // match block_rule
  public static boolean match_rule(PsiBuilder b, int l) {
    if (!recursion_guard_(b, l, "match_rule")) return false;
    if (!nextTokenIs(b, MATCH)) return false;
    boolean r;
    Marker m = enter_section_(b);
    r = consumeToken(b, MATCH);
    r = r && block_rule(b, l + 1);
    exit_section_(b, m, MATCH_RULE, r);
    return r;
  }

  /* ********************************************************** */
  // not (ahead_rule | behind_rule | whitespaceliteral semicolon | digit semicolon | word semicolon)
  public static boolean not_rule(PsiBuilder b, int l) {
    if (!recursion_guard_(b, l, "not_rule")) return false;
    if (!nextTokenIs(b, NOT)) return false;
    boolean r;
    Marker m = enter_section_(b);
    r = consumeToken(b, NOT);
    r = r && not_rule_1(b, l + 1);
    exit_section_(b, m, NOT_RULE, r);
    return r;
  }

  // ahead_rule | behind_rule | whitespaceliteral semicolon | digit semicolon | word semicolon
  private static boolean not_rule_1(PsiBuilder b, int l) {
    if (!recursion_guard_(b, l, "not_rule_1")) return false;
    boolean r;
    Marker m = enter_section_(b);
    r = ahead_rule(b, l + 1);
    if (!r) r = behind_rule(b, l + 1);
    if (!r) r = parseTokens(b, 0, WHITESPACELITERAL, SEMICOLON);
    if (!r) r = parseTokens(b, 0, DIGIT, SEMICOLON);
    if (!r) r = parseTokens(b, 0, WORD, SEMICOLON);
    exit_section_(b, m, null, r);
    return r;
  }

  /* ********************************************************** */
  // to_rule | number | some | over_rule | option | any
  static boolean of_first(PsiBuilder b, int l) {
    if (!recursion_guard_(b, l, "of_first")) return false;
    boolean r;
    r = to_rule(b, l + 1);
    if (!r) r = consumeToken(b, NUMBER);
    if (!r) r = consumeToken(b, SOME);
    if (!r) r = over_rule(b, l + 1);
    if (!r) r = consumeToken(b, OPTION);
    if (!r) r = consumeToken(b, ANY);
    return r;
  }

  /* ********************************************************** */
  // of_first of [expression*]
  public static boolean of_rule(PsiBuilder b, int l) {
    if (!recursion_guard_(b, l, "of_rule")) return false;
    boolean r;
    Marker m = enter_section_(b, l, _NONE_, OF_RULE, "<of rule>");
    r = of_first(b, l + 1);
    r = r && consumeToken(b, OF);
    r = r && of_rule_2(b, l + 1);
    exit_section_(b, l, m, r, false, null);
    return r;
  }

  // [expression*]
  private static boolean of_rule_2(PsiBuilder b, int l) {
    if (!recursion_guard_(b, l, "of_rule_2")) return false;
    of_rule_2_0(b, l + 1);
    return true;
  }

  // expression*
  private static boolean of_rule_2_0(PsiBuilder b, int l) {
    if (!recursion_guard_(b, l, "of_rule_2_0")) return false;
    while (true) {
      int c = current_position_(b);
      if (!expression(b, l + 1)) break;
      if (!empty_element_parsed_guard_(b, "of_rule_2_0", c)) break;
    }
    return true;
  }

  /* ********************************************************** */
  // over number
  public static boolean over_rule(PsiBuilder b, int l) {
    if (!recursion_guard_(b, l, "over_rule")) return false;
    if (!nextTokenIs(b, OVER)) return false;
    boolean r;
    Marker m = enter_section_(b);
    r = consumeTokens(b, 0, OVER, NUMBER);
    exit_section_(b, m, OVER_RULE, r);
    return r;
  }

  /* ********************************************************** */
  // [rules*]
  static boolean root(PsiBuilder b, int l) {
    if (!recursion_guard_(b, l, "root")) return false;
    root_0(b, l + 1);
    return true;
  }

  // rules*
  private static boolean root_0(PsiBuilder b, int l) {
    if (!recursion_guard_(b, l, "root_0")) return false;
    while (true) {
      int c = current_position_(b);
      if (!rules(b, l + 1)) break;
      if (!empty_element_parsed_guard_(b, "root_0", c)) break;
    }
    return true;
  }

  /* ********************************************************** */
  // expression | of_rule
  static boolean rules(PsiBuilder b, int l) {
    if (!recursion_guard_(b, l, "rules")) return false;
    boolean r;
    r = expression(b, l + 1);
    if (!r) r = of_rule(b, l + 1);
    return r;
  }

  /* ********************************************************** */
  // string semicolon
  public static boolean string_rule(PsiBuilder b, int l) {
    if (!recursion_guard_(b, l, "string_rule")) return false;
    if (!nextTokenIs(b, STRING)) return false;
    boolean r;
    Marker m = enter_section_(b);
    r = consumeTokens(b, 0, STRING, SEMICOLON);
    exit_section_(b, m, STRING_RULE, r);
    return r;
  }

  /* ********************************************************** */
  // start | end | char | whitespaceliteral | space | newline | tab | return | feed | null | digit | vertical | word | alphabet
  static boolean symbols_(PsiBuilder b, int l) {
    if (!recursion_guard_(b, l, "symbols_")) return false;
    boolean r;
    r = consumeToken(b, START);
    if (!r) r = consumeToken(b, END);
    if (!r) r = consumeToken(b, CHAR);
    if (!r) r = consumeToken(b, WHITESPACELITERAL);
    if (!r) r = consumeToken(b, SPACE);
    if (!r) r = consumeToken(b, NEWLINE);
    if (!r) r = consumeToken(b, TAB);
    if (!r) r = consumeToken(b, RETURN);
    if (!r) r = consumeToken(b, FEED);
    if (!r) r = consumeToken(b, NULL);
    if (!r) r = consumeToken(b, DIGIT);
    if (!r) r = consumeToken(b, VERTICAL);
    if (!r) r = consumeToken(b, WORD);
    if (!r) r = consumeToken(b, ALPHABET);
    return r;
  }

  /* ********************************************************** */
  // symbols_ semicolon
  public static boolean symbols_rule(PsiBuilder b, int l) {
    if (!recursion_guard_(b, l, "symbols_rule")) return false;
    boolean r;
    Marker m = enter_section_(b, l, _NONE_, SYMBOLS_RULE, "<symbols rule>");
    r = symbols_(b, l + 1);
    r = r && consumeToken(b, SEMICOLON);
    exit_section_(b, l, m, r, false, null);
    return r;
  }

  /* ********************************************************** */
  // number to number
  public static boolean to_rule(PsiBuilder b, int l) {
    if (!recursion_guard_(b, l, "to_rule")) return false;
    if (!nextTokenIs(b, NUMBER)) return false;
    boolean r;
    Marker m = enter_section_(b);
    r = consumeTokens(b, 0, NUMBER, TO, NUMBER);
    exit_section_(b, m, TO_RULE, r);
    return r;
  }

}
