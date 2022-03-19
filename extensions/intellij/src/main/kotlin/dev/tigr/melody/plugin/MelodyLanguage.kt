package dev.tigr.melody.plugin

import com.intellij.extapi.psi.PsiFileBase
import com.intellij.icons.AllIcons
import com.intellij.lang.ASTNode
import com.intellij.lang.Language
import com.intellij.lang.ParserDefinition
import com.intellij.lang.PsiParser
import com.intellij.lexer.FlexAdapter
import com.intellij.lexer.Lexer
import com.intellij.openapi.fileTypes.FileType
import com.intellij.openapi.fileTypes.LanguageFileType
import com.intellij.openapi.project.Project
import com.intellij.psi.FileViewProvider
import com.intellij.psi.PsiElement
import com.intellij.psi.PsiFile
import com.intellij.psi.tree.IFileElementType
import com.intellij.psi.tree.TokenSet
import dev.tigr.melody.plugin.parser.MelodyParser
import dev.tigr.melody.plugin.psi.MelodyTypes
import javax.swing.Icon

object MelodyLanguage: Language("Melody")

class MelodyFile(fileViewProvider: FileViewProvider): PsiFileBase(fileViewProvider, MelodyLanguage) {
    override fun getFileType(): FileType = MelodyLanguageFileType
    override fun toString(): String = "Melody File"
}

object MelodyLanguageFileType: LanguageFileType(MelodyLanguage) {
    override fun getName(): String = "Melody"
    override fun getDescription(): String = "Melody regex language"
    override fun getDefaultExtension(): String = "mdy"
    override fun getIcon(): Icon = AllIcons.FileTypes.Regexp
}

class MelodyParserDefinition: ParserDefinition {
    companion object {
        private val FILE = IFileElementType(MelodyLanguage)
        private val COMMENT = TokenSet.create(MelodyTypes.COMMENT)
        private val STRING = TokenSet.create(MelodyTypes.STRING)
    }

    override fun createLexer(project: Project?): Lexer = FlexAdapter(_MelodyLexer())
    override fun createParser(project: Project?): PsiParser = MelodyParser()
    override fun getFileNodeType(): IFileElementType = FILE
    override fun getCommentTokens(): TokenSet = COMMENT
    override fun getStringLiteralElements(): TokenSet = STRING
    override fun createElement(node: ASTNode?): PsiElement = MelodyTypes.Factory.createElement(node)
    override fun createFile(viewProvider: FileViewProvider): PsiFile = MelodyFile(viewProvider)
}
