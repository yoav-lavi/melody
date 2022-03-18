// This is a generated file. Not intended for manual editing.
package dev.tigr.melody.plugin.psi;

import java.util.List;
import org.jetbrains.annotations.*;
import com.intellij.psi.PsiElement;

public interface MelodyOfRule extends PsiElement {

  @NotNull
  MelodyExpression getExpression();

  @Nullable
  MelodyOverRule getOverRule();

  @Nullable
  MelodyRangeRule getRangeRule();

  @Nullable
  PsiElement getNumber();

}
