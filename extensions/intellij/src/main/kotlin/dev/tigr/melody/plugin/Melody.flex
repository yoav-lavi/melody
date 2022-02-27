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

NUMBER=\p{N}*
COMMENT="//".*
STRING=\"[^\"]*\"|'[^']*'|`[^`]*`
WHITESPACE=[ \t\n\x0B\f\r]+

%%
<YYINITIAL> {
  {WHITE_SPACE}       { return WHITE_SPACE; }

  "of"                { return OF; }
  "to"                { return TO; }
  "capture"           { return CAPTURE; }
  "some"              { return SOME; }
  "match"             { return MATCH; }
  "over"              { return OVER; }
  "option"            { return OPTION; }
  "not"               { return NOT; }
  "either"            { return EITHER; }
  "any"               { return ANY; }
  "ahead"             { return AHEAD; }
  "behind"            { return BEHIND; }
  "<start>"           { return START; }
  "<end>"             { return END; }
  "<char>"            { return CHAR; }
  "<whitespace>"      { return WHITESPACELITERAL; }
  "<space>"           { return SPACE; }
  "<newline>"         { return NEWLINE; }
  "<tab>"             { return TAB; }
  "<return>"          { return RETURN; }
  "<feed>"            { return FEED; }
  "<null>"            { return NULL; }
  "<digit>"           { return DIGIT; }
  "<vertical>"        { return VERTICAL; }
  "<word>"            { return WORD; }
  "<alphabet>"        { return ALPHABET; }
  "{"                 { return OPENBRACE; }
  "}"                 { return CLOSEBRACE; }
  ";"                 { return SEMICOLON; }

  {NUMBER}            { return NUMBER; }
  {COMMENT}           { return COMMENT; }
  {STRING}            { return STRING; }
  {WHITESPACE}        { return WHITESPACE; }

}

[^] { return BAD_CHARACTER; }
