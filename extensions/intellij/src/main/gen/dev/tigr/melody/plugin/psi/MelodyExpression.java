// This is a generated file. Not intended for manual editing.
package dev.tigr.melody.plugin.psi;

import java.util.List;
import org.jetbrains.annotations.*;
import com.intellij.psi.PsiElement;

public interface MelodyExpression extends PsiElement {

  @Nullable
  MelodyAheadRule getAheadRule();

  @Nullable
  MelodyBehindRule getBehindRule();

  @Nullable
  MelodyCaptureRule getCaptureRule();

  @Nullable
  MelodyEitherRule getEitherRule();

  @Nullable
  MelodyMatchRule getMatchRule();

  @Nullable
  MelodyNotRule getNotRule();

  @Nullable
  MelodyStringRule getStringRule();

  @Nullable
  MelodySymbolsRule getSymbolsRule();

  @Nullable
  MelodyToRule getToRule();

  @Nullable
  MelodyVariableRule getVariableRule();

}
