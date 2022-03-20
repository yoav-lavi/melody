// This is a generated file. Not intended for manual editing.
package dev.tigr.melody.plugin.psi.impl;

import java.util.List;
import org.jetbrains.annotations.*;
import com.intellij.lang.ASTNode;
import com.intellij.psi.PsiElement;
import com.intellij.psi.PsiElementVisitor;
import com.intellij.psi.util.PsiTreeUtil;
import static dev.tigr.melody.plugin.psi.MelodyTypes.*;
import com.intellij.extapi.psi.ASTWrapperPsiElement;
import dev.tigr.melody.plugin.psi.*;

public class MelodyExpressionImpl extends ASTWrapperPsiElement implements MelodyExpression {

  public MelodyExpressionImpl(@NotNull ASTNode node) {
    super(node);
  }

  public void accept(@NotNull MelodyVisitor visitor) {
    visitor.visitExpression(this);
  }

  @Override
  public void accept(@NotNull PsiElementVisitor visitor) {
    if (visitor instanceof MelodyVisitor) accept((MelodyVisitor)visitor);
    else super.accept(visitor);
  }

  @Override
  @Nullable
  public MelodyAheadRule getAheadRule() {
    return findChildByClass(MelodyAheadRule.class);
  }

  @Override
  @Nullable
  public MelodyBehindRule getBehindRule() {
    return findChildByClass(MelodyBehindRule.class);
  }

  @Override
  @Nullable
  public MelodyCaptureRule getCaptureRule() {
    return findChildByClass(MelodyCaptureRule.class);
  }

  @Override
  @Nullable
  public MelodyEitherRule getEitherRule() {
    return findChildByClass(MelodyEitherRule.class);
  }

  @Override
  @Nullable
  public MelodyMatchRule getMatchRule() {
    return findChildByClass(MelodyMatchRule.class);
  }

  @Override
  @Nullable
  public MelodyNotRule getNotRule() {
    return findChildByClass(MelodyNotRule.class);
  }

  @Override
  @Nullable
  public MelodyStringRule getStringRule() {
    return findChildByClass(MelodyStringRule.class);
  }

  @Override
  @Nullable
  public MelodySymbolsRule getSymbolsRule() {
    return findChildByClass(MelodySymbolsRule.class);
  }

  @Override
  @Nullable
  public MelodyToRule getToRule() {
    return findChildByClass(MelodyToRule.class);
  }

  @Override
  @Nullable
  public MelodyVariableRule getVariableRule() {
    return findChildByClass(MelodyVariableRule.class);
  }

}
