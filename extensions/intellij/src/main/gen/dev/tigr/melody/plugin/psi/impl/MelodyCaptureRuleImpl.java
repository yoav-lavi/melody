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

public class MelodyCaptureRuleImpl extends ASTWrapperPsiElement implements MelodyCaptureRule {

  public MelodyCaptureRuleImpl(@NotNull ASTNode node) {
    super(node);
  }

  public void accept(@NotNull MelodyVisitor visitor) {
    visitor.visitCaptureRule(this);
  }

  @Override
  public void accept(@NotNull PsiElementVisitor visitor) {
    if (visitor instanceof MelodyVisitor) accept((MelodyVisitor)visitor);
    else super.accept(visitor);
  }

  @Override
  @NotNull
  public List<MelodyExpression> getExpressionList() {
    return PsiTreeUtil.getChildrenOfTypeAsList(this, MelodyExpression.class);
  }

  @Override
  @NotNull
  public List<MelodyLetRule> getLetRuleList() {
    return PsiTreeUtil.getChildrenOfTypeAsList(this, MelodyLetRule.class);
  }

  @Override
  @NotNull
  public List<MelodyOfRule> getOfRuleList() {
    return PsiTreeUtil.getChildrenOfTypeAsList(this, MelodyOfRule.class);
  }

  @Override
  @Nullable
  public PsiElement getCharacter() {
    return findChildByType(CHARACTER);
  }

  @Override
  @Nullable
  public PsiElement getIdentifier() {
    return findChildByType(IDENTIFIER);
  }

}
