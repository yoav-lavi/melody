package dev.tigr.melody.plugin

import com.intellij.psi.PsiElement
import com.intellij.psi.PsiErrorElement
import com.intellij.psi.TokenType
import com.intellij.psi.util.elementType
import com.intellij.psi.util.nextLeaf
import com.intellij.psi.util.prevLeaf
import dev.tigr.melody.plugin.psi.MelodyExpression
import dev.tigr.melody.plugin.psi.MelodyTypes

// a tiny melody compiler, with no specific compiler errors or warnings
// everything is contained in one function, recursion is used
object MelodyCompiler {
    fun compile(elements: Array<PsiElement>, negative: Boolean = false,
                variables: HashMap<String, String> = hashMapOf(), grouped: Boolean = false): String? {
        val clear = variables.size == 0 // whether the variables cleared after the end of this function
        val sb = StringBuilder()

        if(elements.any { it is PsiErrorElement }) return null

        elements.forEach { element ->
            when(element) {
                is MelodyExpression -> sb.append(compile(element.children, negative = negative, variables = variables))
                else -> when(element.elementType) {
                    MelodyTypes.SYMBOLS_RULE -> sb.append(if(!negative) when(element.firstChild?.elementType) {
                        MelodyTypes.CHAR -> "."
                        MelodyTypes.WHITESPACELITERAL -> "\\s"
                        MelodyTypes.SPACE -> " "
                        MelodyTypes.NEWLINE -> "\\n"
                        MelodyTypes.TAB -> "\\t"
                        MelodyTypes.RETURN -> "\\r"
                        MelodyTypes.FEED -> "\\f"
                        MelodyTypes.NULL -> "\\0"
                        MelodyTypes.DIGIT -> "\\d"
                        MelodyTypes.VERTICAL -> "\\v"
                        MelodyTypes.WORD -> "\\w"
                        MelodyTypes.ALPHABET -> "[a-zA-Z]"
                        MelodyTypes.ALPHANUMERIC -> "[a-zA-Z0-9]"
                        MelodyTypes.BOUNDARY -> "\\b"
                        MelodyTypes.BACKSPACE -> "[\\b]"
                        MelodyTypes.START -> "^"
                        MelodyTypes.END -> "$"
                        MelodyTypes.CASED_LETTER_CATEGORY -> "\\p{L&}"
                        MelodyTypes.CLOSE_PUNCTUATION_CATEGORY -> "\\p{Pe}"
                        MelodyTypes.CONNECTOR_PUNCTUATION_CATEGORY -> "\\p{Pc}"
                        MelodyTypes.CONTROL_CATEGORY -> "\\p{Cc}"
                        MelodyTypes.CURRENCY_SYMBOL_CATEGORY -> "\\p{Sc}"
                        MelodyTypes.DASH_PUNCTUATION_CATEGORY -> "\\p{Pd}"
                        MelodyTypes.DECIMAL_DIGIT_NUMBER_CATEGORY -> "\\p{Nd}"
                        MelodyTypes.ENCLOSING_MARK_CATEGORY -> "\\p{Me}"
                        MelodyTypes.FINAL_PUNCTUATION_CATEGORY -> "\\p{Pf}"
                        MelodyTypes.FORMAT_CATEGORY -> "\\p{Cf}"
                        MelodyTypes.INITIAL_PUNCTUATION_CATEGORY -> "\\p{Pi}"
                        MelodyTypes.LETTER_NUMBER_CATEGORY -> "\\p{Nl}"
                        MelodyTypes.LETTER_CATEGORY -> "\\p{L}"
                        MelodyTypes.LINE_SEPARATOR_CATEGORY -> "\\p{Zl}"
                        MelodyTypes.LOWERCASE_LETTER_CATEGORY -> "\\p{Ll}"
                        MelodyTypes.MARK_CATEGORY -> "\\p{M}"
                        MelodyTypes.MATH_SYMBOL_CATEGORY -> "\\p{Sm}"
                        MelodyTypes.MODIFIER_LETTER_CATEGORY -> "\\p{Lm}"
                        MelodyTypes.MODIFIER_SYMBOL_CATEGORY -> "\\p{Sk}"
                        MelodyTypes.NON_SPACING_MARK_CATEGORY -> "\\p{Mn}"
                        MelodyTypes.NUMBER_CATEGORY -> "\\p{N}"
                        MelodyTypes.OPEN_PUNCTUATION_CATEGORY -> "\\p{Ps}"
                        MelodyTypes.OTHER_LETTER_CATEGORY -> "\\p{Lo}"
                        MelodyTypes.OTHER_NUMBER_CATEGORY -> "\\p{No}"
                        MelodyTypes.OTHER_PUNCTUATION_CATEGORY -> "\\p{Po}"
                        MelodyTypes.OTHER_SYMBOL_CATEGORY -> "\\p{So}"
                        MelodyTypes.OTHER_CATEGORY -> "\\p{C}"
                        MelodyTypes.PARAGRAPH_SEPARATOR_CATEGORY -> "\\p{Zp}"
                        MelodyTypes.PRIVATE_USE_CATEGORY -> "\\p{Co}"
                        MelodyTypes.PUNCTUATION_CATEGORY -> "\\p{P}"
                        MelodyTypes.SEPARATOR_CATEGORY -> "\\p{Z}"
                        MelodyTypes.SPACE_SEPARATOR_CATEGORY -> "\\p{Zs}"
                        MelodyTypes.SPACING_COMBINING_MARK_CATEGORY -> "\\p{Mc}"
                        MelodyTypes.SURROGATE_CATEGORY -> "\\p{Cs}"
                        MelodyTypes.SYMBOL_CATEGORY -> "\\p{S}"
                        MelodyTypes.TITLECASE_LETTER_CATEGORY -> "\\p{Lt}"
                        MelodyTypes.UNASSIGNED_CATEGORY -> "\\p{Cn}"
                        MelodyTypes.UPPERCASE_LETTER_CATEGORY-> "\\p{Lu}"
                        else -> return null
                    } else when(element.firstChild?.elementType) {
                        // negative symbols
                        MelodyTypes.CHAR -> "[^.]"
                        MelodyTypes.WHITESPACELITERAL -> "\\S"
                        MelodyTypes.SPACE -> "[^ ]"
                        MelodyTypes.NEWLINE -> "[^\\n]"
                        MelodyTypes.TAB -> "[^\\t]"
                        MelodyTypes.RETURN -> "[^\\r]"
                        MelodyTypes.FEED -> "[^\\f]"
                        MelodyTypes.NULL -> "[^\\0]"
                        MelodyTypes.DIGIT -> "\\D"
                        MelodyTypes.VERTICAL -> "[^\\v]"
                        MelodyTypes.WORD -> "\\W"
                        MelodyTypes.ALPHABET -> "[^a-zA-Z]"
                        MelodyTypes.ALPHANUMERIC -> "[^a-zA-Z0-9]"
                        MelodyTypes.BOUNDARY -> "\\B"
                        MelodyTypes.BACKSPACE -> "[^\\b]"
                        MelodyTypes.CASED_LETTER_CATEGORY -> "\\P{L&}"
                        MelodyTypes.CLOSE_PUNCTUATION_CATEGORY -> "\\P{Pe}"
                        MelodyTypes.CONNECTOR_PUNCTUATION_CATEGORY -> "\\P{Pc}"
                        MelodyTypes.CONTROL_CATEGORY -> "\\P{Cc}"
                        MelodyTypes.CURRENCY_SYMBOL_CATEGORY -> "\\P{Sc}"
                        MelodyTypes.DASH_PUNCTUATION_CATEGORY -> "\\P{Pd}"
                        MelodyTypes.DECIMAL_DIGIT_NUMBER_CATEGORY -> "\\P{Nd}"
                        MelodyTypes.ENCLOSING_MARK_CATEGORY -> "\\P{Me}"
                        MelodyTypes.FINAL_PUNCTUATION_CATEGORY -> "\\P{Pf}"
                        MelodyTypes.FORMAT_CATEGORY -> "\\P{Cf}"
                        MelodyTypes.INITIAL_PUNCTUATION_CATEGORY -> "\\P{Pi}"
                        MelodyTypes.LETTER_NUMBER_CATEGORY -> "\\P{Nl}"
                        MelodyTypes.LETTER_CATEGORY -> "\\P{L}"
                        MelodyTypes.LINE_SEPARATOR_CATEGORY -> "\\P{Zl}"
                        MelodyTypes.LOWERCASE_LETTER_CATEGORY -> "\\P{Ll}"
                        MelodyTypes.MARK_CATEGORY -> "\\P{M}"
                        MelodyTypes.MATH_SYMBOL_CATEGORY -> "\\P{Sm}"
                        MelodyTypes.MODIFIER_LETTER_CATEGORY -> "\\P{Lm}"
                        MelodyTypes.MODIFIER_SYMBOL_CATEGORY -> "\\P{Sk}"
                        MelodyTypes.NON_SPACING_MARK_CATEGORY -> "\\P{Mn}"
                        MelodyTypes.NUMBER_CATEGORY -> "\\P{N}"
                        MelodyTypes.OPEN_PUNCTUATION_CATEGORY -> "\\P{Ps}"
                        MelodyTypes.OTHER_LETTER_CATEGORY -> "\\P{Lo}"
                        MelodyTypes.OTHER_NUMBER_CATEGORY -> "\\P{No}"
                        MelodyTypes.OTHER_PUNCTUATION_CATEGORY -> "\\P{Po}"
                        MelodyTypes.OTHER_SYMBOL_CATEGORY -> "\\P{So}"
                        MelodyTypes.OTHER_CATEGORY -> "\\P{C}"
                        MelodyTypes.PARAGRAPH_SEPARATOR_CATEGORY -> "\\P{Zp}"
                        MelodyTypes.PRIVATE_USE_CATEGORY -> "\\P{Co}"
                        MelodyTypes.PUNCTUATION_CATEGORY -> "\\P{P}"
                        MelodyTypes.SEPARATOR_CATEGORY -> "\\P{Z}"
                        MelodyTypes.SPACE_SEPARATOR_CATEGORY -> "\\P{Zs}"
                        MelodyTypes.SPACING_COMBINING_MARK_CATEGORY -> "\\P{Mc}"
                        MelodyTypes.SURROGATE_CATEGORY -> "\\P{Cs}"
                        MelodyTypes.SYMBOL_CATEGORY -> "\\P{S}"
                        MelodyTypes.TITLECASE_LETTER_CATEGORY -> "\\P{Lt}"
                        MelodyTypes.UNASSIGNED_CATEGORY -> "\\P{Cn}"
                        MelodyTypes.UPPERCASE_LETTER_CATEGORY-> "\\P{Lu}"
                        else -> return null
                    })
                    MelodyTypes.NOT_RULE -> sb.append(compile(element.children, negative = true, variables = variables))
                    MelodyTypes.STRING_RULE -> {
                        var raw = false
                        var string = element.firstChild?.text?.let {
                            raw = it[0] == '`'
                            it.substring(1, it.length - 1)
                        } ?: return null

                        // escape non-raw strings
                        if(!raw) {
                            val temp = StringBuilder()
                            for(char in string) {
                                when(char) {
                                    '[', ']', '(', ')', '{', '}', '*', '+', '?', '|', '^', '$', '.',
                                    '-', '\\' -> temp.append("\\").append(char)
                                    else -> temp.append(char)
                                }
                            }
                            string = temp.toString()
                        }

                        if(string.length > 1 && grouped) sb.append("(?:").append(string).append(')')
                        else sb.append(string)
                    }
                    MelodyTypes.TO_RULE -> {
                        val last = element.lastChild?.prevLeaf {
                            it.elementType == MelodyTypes.NUMBER || it.elementType == MelodyTypes.CHARACTER
                        } ?: return null

                        sb.append('[')
                        if(negative) sb.append('^')
                        sb.append(element.firstChild?.text ?: return null)
                            .append('-')
                            .append(last.text)
                            .append(']')
                    }
                    MelodyTypes.MATCH_RULE -> sb.append("(?:").append(compile(element.children, variables = variables)).append(')')
                    MelodyTypes.CAPTURE_RULE -> {
                        val id = element.firstChild?.nextLeaf { it.elementType == MelodyTypes.IDENTIFIER || it.elementType == MelodyTypes.CHARACTER }
                        sb.append('(')
                        if(id != null) sb.append("?<").append(id.text).append('>')
                        sb.append(compile(element.children, variables = variables))
                        .append(')')
                    }
                    MelodyTypes.EITHER_RULE -> {
                        sb.append("(?:")
                        var first = true
                        element.children.forEach {
                            if(!first) sb.append('|')
                            else first = false
                            if(it.elementType == MelodyTypes.EXPRESSION) sb.append(compile(it.children, variables = variables))
                        }
                        sb.append(')')
                    }
                    MelodyTypes.AHEAD_RULE -> sb.append("(?=").append(compile(element.children, variables = variables)).append(')')
                    MelodyTypes.BEHIND_RULE -> sb.append("(?<=").append(compile(element.children, variables = variables)).append(')')
                    MelodyTypes.LET_RULE -> {
                        val name = element.firstChild.nextLeaf { it.elementType == MelodyTypes.VARIABLE } ?: return null
                        variables[name.text] = compile(element.children, variables = variables) ?: return null
                    }
                    MelodyTypes.VARIABLE_RULE -> {
                        val first = element.firstChild ?: return null
                        if(first.elementType != MelodyTypes.VARIABLE) return null
                        val compiled = variables[first.text] ?: return null
                        sb.append(compiled)
                    }
                    MelodyTypes.OF_RULE -> {
                        var first = element.firstChild ?: return null
                        val lazy = first.elementType == MelodyTypes.LAZY
                        val expr = element.lastChild ?: return null

                        if(expr.elementType != MelodyTypes.EXPRESSION) return null
                        sb.append(compile(expr.children, variables = variables, grouped = true))

                        if(lazy) first = first.nextLeaf { it.elementType != TokenType.WHITE_SPACE } ?: return null

                        sb.append(when(first.elementType) {
                            MelodyTypes.SOME -> "+"
                            MelodyTypes.ANY -> "*"
                            MelodyTypes.OPTION -> "?"
                            MelodyTypes.NUMBER -> "{${first.text}}"
                            MelodyTypes.OVER_RULE -> {
                                val num = (first.lastChild?.text?.toIntOrNull() ?: return null) + 1
                                "{$num,}"
                            }
                            MelodyTypes.RANGE_RULE -> "{${first.firstChild?.text ?: return null},${first.lastChild?.text ?: return null}}"
                            else -> return null
                        })

                        // yes, 'lazy option of' is possible, it's also possible in the official compiler
                        if(lazy) sb.append('?')
                    }
                }
            }
        }

        if(clear) variables.clear()

        return sb.toString()
    }
}
