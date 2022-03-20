package dev.tigr.melody.plugin.psi

import com.intellij.psi.tree.IElementType
import dev.tigr.melody.plugin.MelodyLanguage

class MelodyTokenType(debugName: String): IElementType(debugName, MelodyLanguage)
class MelodyElementType(debugName: String): IElementType(debugName, MelodyLanguage)
