package dev.tigr.melody.plugin

import com.intellij.icons.AllIcons
import com.intellij.lexer.FlexAdapter
import com.intellij.lexer.Lexer
import com.intellij.openapi.editor.DefaultLanguageHighlighterColors
import com.intellij.openapi.editor.HighlighterColors
import com.intellij.openapi.editor.colors.TextAttributesKey
import com.intellij.openapi.editor.colors.TextAttributesKey.createTextAttributesKey
import com.intellij.openapi.fileTypes.SyntaxHighlighter
import com.intellij.openapi.fileTypes.SyntaxHighlighterBase
import com.intellij.openapi.fileTypes.SyntaxHighlighterFactory
import com.intellij.openapi.options.colors.AttributesDescriptor
import com.intellij.openapi.options.colors.ColorDescriptor
import com.intellij.openapi.options.colors.ColorSettingsPage
import com.intellij.openapi.project.Project
import com.intellij.openapi.vfs.VirtualFile
import com.intellij.psi.TokenType
import com.intellij.psi.tree.IElementType
import dev.tigr.melody.plugin.psi.MelodyTypes.*
import javax.swing.Icon

class MelodySyntaxHighlighter: SyntaxHighlighterBase() {
    companion object {
        internal val BAD_CHAR_KEY = createTextAttributesKey("MELODY_BAD_CHARACTER", HighlighterColors.BAD_CHARACTER)
        internal val COMMENT_KEY = createTextAttributesKey("MELODY_COMMENT", DefaultLanguageHighlighterColors.LINE_COMMENT)
        internal val KEYWORD_KEY = createTextAttributesKey("MELODY_KEYWORD", DefaultLanguageHighlighterColors.KEYWORD)
        internal val SYMBOL_KEY = createTextAttributesKey("MELODY_SYMBOL", DefaultLanguageHighlighterColors.FUNCTION_DECLARATION)
        internal val STRING_KEY = createTextAttributesKey("MELODY_STRING", DefaultLanguageHighlighterColors.STRING)
        internal val NUMBER_KEY = createTextAttributesKey("MELODY_NUMBER", DefaultLanguageHighlighterColors.NUMBER)
        internal val SEMICOLON_KEY = createTextAttributesKey("MELODY_SEMICOLON", DefaultLanguageHighlighterColors.SEMICOLON)
        internal val BRACES_KEY = createTextAttributesKey("MELODY_BRACES", DefaultLanguageHighlighterColors.BRACES)
        internal val EQUALS_KEY = createTextAttributesKey("MELODY_OPERATOR", DefaultLanguageHighlighterColors.OPERATION_SIGN)

        private val BAD_CHAR_KEYS = arrayOf(BAD_CHAR_KEY)
        private val COMMENT_KEYS = arrayOf(COMMENT_KEY)
        private val KEYWORD_KEYS = arrayOf(KEYWORD_KEY)
        private val SYMBOL_KEYS = arrayOf(SYMBOL_KEY)
        private val STRING_KEYS = arrayOf(STRING_KEY)
        private val NUMBER_KEYS = arrayOf(NUMBER_KEY)
        private val SEMICOLON_KEYS = arrayOf(SEMICOLON_KEY)
        private val BRACES_KEYS = arrayOf(BRACES_KEY)
        private val EQUALS_KEYS = arrayOf(EQUALS_KEY)
        private val EMPTY_KEYS = arrayOf<TextAttributesKey>()
    }

    override fun getHighlightingLexer(): Lexer = FlexAdapter(_MelodyLexer())

    override fun getTokenHighlights(tokenType: IElementType?): Array<TextAttributesKey> {
        return when(tokenType) {
            COMMENT -> COMMENT_KEYS
            OF, TO, CAPTURE, MATCH, OVER, NOT, EITHER, AHEAD, BEHIND, LET, LAZY
            -> KEYWORD_KEYS
            SOME, OPTION, ANY, START, END, CHAR, WHITESPACELITERAL, SPACE,
            NEWLINE, TAB, RETURN, FEED, NULL, DIGIT, VERTICAL, WORD, ALPHABET,
            ALPHANUMERIC, BOUNDARY, BACKSPACE, CHARACTER, IDENTIFIER,
            LETTER_CATEGORY, LOWERCASE_LETTER_CATEGORY, UPPERCASE_LETTER_CATEGORY, TITLECASE_LETTER_CATEGORY, CASED_LETTER_CATEGORY,
            MODIFIER_LETTER_CATEGORY, OTHER_LETTER_CATEGORY, MARK_CATEGORY, NON_SPACING_MARK_CATEGORY, SPACING_COMBINING_MARK_CATEGORY,
            ENCLOSING_MARK_CATEGORY, SEPARATOR_CATEGORY, SPACE_SEPARATOR_CATEGORY, LINE_SEPARATOR_CATEGORY, PARAGRAPH_SEPARATOR_CATEGORY,
            SYMBOL_CATEGORY, MATH_SYMBOL_CATEGORY, CURRENCY_SYMBOL_CATEGORY, MODIFIER_SYMBOL_CATEGORY, OTHER_SYMBOL_CATEGORY, NUMBER_CATEGORY,
            DECIMAL_DIGIT_NUMBER_CATEGORY, LETTER_NUMBER_CATEGORY, OTHER_NUMBER_CATEGORY, PUNCTUATION_CATEGORY, DASH_PUNCTUATION_CATEGORY,
            OPEN_PUNCTUATION_CATEGORY, CLOSE_PUNCTUATION_CATEGORY, INITIAL_PUNCTUATION_CATEGORY, FINAL_PUNCTUATION_CATEGORY,
            CONNECTOR_PUNCTUATION_CATEGORY, OTHER_PUNCTUATION_CATEGORY, OTHER_CATEGORY, CONTROL_CATEGORY, FORMAT_CATEGORY, PRIVATE_USE_CATEGORY,
            SURROGATE_CATEGORY, UNASSIGNED_CATEGORY
            -> SYMBOL_KEYS
            STRING -> STRING_KEYS
            NUMBER -> NUMBER_KEYS
            SEMICOLON -> SEMICOLON_KEYS
            EQUALS -> EQUALS_KEYS
            OPENBRACE, CLOSEBRACE -> BRACES_KEYS
            TokenType.BAD_CHARACTER -> BAD_CHAR_KEYS
            else -> EMPTY_KEYS
        }
    }
}

class MelodySyntaxHighlighterFactory: SyntaxHighlighterFactory() {
    override fun getSyntaxHighlighter(project: Project?, virtualFile: VirtualFile?): SyntaxHighlighter {
        return MelodySyntaxHighlighter()
    }
}

class MelodyColorSettingsPage: ColorSettingsPage {
    companion object {
        private val DESCRIPTORS = arrayOf(
            AttributesDescriptor("Comment", MelodySyntaxHighlighter.COMMENT_KEY),
            AttributesDescriptor("Keyword", MelodySyntaxHighlighter.KEYWORD_KEY),
            AttributesDescriptor("Symbol", MelodySyntaxHighlighter.SYMBOL_KEY),
            AttributesDescriptor("String", MelodySyntaxHighlighter.STRING_KEY),
            AttributesDescriptor("Number", MelodySyntaxHighlighter.NUMBER_KEY),
            AttributesDescriptor("Semicolon", MelodySyntaxHighlighter.SEMICOLON_KEY),
            AttributesDescriptor("Braces", MelodySyntaxHighlighter.BRACES_KEY),
            AttributesDescriptor("Equals", MelodySyntaxHighlighter.EQUALS_KEY),
            AttributesDescriptor("Bad value", MelodySyntaxHighlighter.BAD_CHAR_KEY)
        )
    }

    override fun getDisplayName(): String = "Melody"
    override fun getIcon(): Icon = AllIcons.Actions.Regex
    override fun getHighlighter(): SyntaxHighlighter = MelodySyntaxHighlighter()
    override fun getAdditionalHighlightingTagToDescriptorMap(): Map<String, TextAttributesKey>? = null
    override fun getAttributeDescriptors(): Array<AttributesDescriptor> = DESCRIPTORS
    override fun getColorDescriptors(): Array<out ColorDescriptor> = ColorDescriptor.EMPTY_ARRAY
    override fun getDemoText(): String = """// you are reading a melody regex file that matches ipv4 addresses
        |<start>;
        |3 of match {
        |    1 to 3 of <digit>;
        |    ".";
        |}
        |1 to 3 of <digit>;
        |<end>;
    """.trimMargin()
}
