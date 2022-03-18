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

public class MelodyOfRuleImpl extends ASTWrapperPsiElement implements MelodyOfRule {

  public MelodyOfRuleImpl(@NotNull ASTNode node) {
    super(node);
  }

  public void accept(@NotNull MelodyVisitor visitor) {
    visitor.visitOfRule(this);
  }

  @Override
  public void accept(@NotNull PsiElementVisitor visitor) {
    if (visitor instanceof MelodyVisitor) accept((MelodyVisitor)visitor);
    else super.accept(visitor);
  }

  @Override
  @NotNull
  public MelodyExpression getExpression() {
    return findNotNullChildByClass(MelodyExpression.class);
  }

  @Override
  @Nullable
  public MelodyOverRule getOverRule() {
    return findChildByClass(MelodyOverRule.class);
  }

  @Override
  @Nullable
  public MelodyRangeRule getRangeRule() {
    return findChildByClass(MelodyRangeRule.class);
  }

  @Override
  @Nullable
  public PsiElement getNumber() {
    return findChildByType(NUMBER);
  }

}
