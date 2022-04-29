package dev.tigr.melody.plugin

import com.intellij.icons.AllIcons
import com.intellij.openapi.Disposable
import com.intellij.openapi.fileEditor.FileEditorManager
import com.intellij.openapi.fileEditor.FileEditorManagerListener
import com.intellij.openapi.project.Project
import com.intellij.openapi.startup.StartupActivity
import com.intellij.openapi.util.Key
import com.intellij.openapi.vfs.VirtualFile
import com.intellij.psi.*
import java.awt.*
import java.awt.datatransfer.StringSelection
import javax.swing.JButton
import javax.swing.JPanel
import javax.swing.JTextField

class MelodyStartup: StartupActivity, Disposable {
    override fun runActivity(project: Project) {
        PsiManager.getInstance(project).addPsiTreeChangeListener(MelodyPsiListener(), this)
    }

    override fun dispose() {
    }
}

class MelodyOutputPanel private constructor(): JPanel(BorderLayout()) {
    companion object {
        val DATA_KEY = Key<MelodyOutputPanel>("Melody::OutputPanel")
        private val map = hashMapOf<VirtualFile, ArrayList<MelodyOutputPanel>>()

        fun add(virtualFile: VirtualFile): MelodyOutputPanel {
            val outputPanel = MelodyOutputPanel()
            map.getOrPut(virtualFile) { arrayListOf() }.add(outputPanel)
            return outputPanel
        }

        fun remove(virtualFile: VirtualFile, melodyOutputPanel: MelodyOutputPanel) {
            val list = map[virtualFile] ?: return
            list.remove(melodyOutputPanel)
            if(list.size == 0) map.remove(virtualFile)
        }

        fun get(virtualFile: VirtualFile): List<MelodyOutputPanel> {
            return map.getOrDefault(virtualFile, emptyList())
        }
    }

    internal val text = JTextField()
    private val copy = JButton(AllIcons.Actions.Copy)

    init {
        text.isEditable = false
        add(text, BorderLayout.CENTER)

        copy.addActionListener {
            Toolkit.getDefaultToolkit().systemClipboard.setContents(StringSelection(text.text), null)
        }
        add(copy, BorderLayout.EAST)
    }
}

class MelodyFileListener: FileEditorManagerListener, FileEditorManagerListener.Before {
    override fun fileOpened(source: FileEditorManager, file: VirtualFile) {
        if(file.fileType == MelodyLanguageFileType) {
            source.getSelectedEditor(file)?.let { editor ->
                val panel = MelodyOutputPanel.add(file)
                source.addBottomComponent(editor, panel)
                editor.putUserData(MelodyOutputPanel.DATA_KEY, panel)
            }

            update(file.toPsiFile(source.project))
        }
    }

    override fun beforeFileClosed(source: FileEditorManager, file: VirtualFile) {
        if(file.fileType == MelodyLanguageFileType) {
            source.getEditors(file).forEach { editor ->
                editor.getUserData(MelodyOutputPanel.DATA_KEY)?.let { panel ->
                    if(panel.isValid) MelodyOutputPanel.remove(file, panel)
                }
            }
        }
    }
}

class MelodyPsiListener: PsiTreeChangeAdapter() {
    override fun childrenChanged(event: PsiTreeChangeEvent) = update(event.file)
}

private fun update(psiFile: PsiFile?) {
    if(psiFile?.virtualFile?.fileType == MelodyLanguageFileType) {
        val compiled = MelodyCompiler.compile(psiFile.children)
        MelodyOutputPanel.get(psiFile.virtualFile).forEach { panel ->
            panel.text.text = compiled ?: "--- COMPILER ERROR ---"
        }
    }
}

private fun VirtualFile.toPsiFile(project: Project): PsiFile? {
    return SingleRootFileViewProvider(PsiManager.getInstance(project), this,
        true, MelodyLanguageFileType).getPsi(MelodyLanguage)
}
