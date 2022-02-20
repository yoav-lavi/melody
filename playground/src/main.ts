import * as monaco from "monaco-editor";
import editorWorker from "monaco-editor/esm/vs/editor/editor.worker?worker";
// build from ../crates/melody_wasm included manually due to issues with wasm-bindgen (similar to https://github.com/rustwasm/wasm-bindgen/issues/113)
import init, { compiler } from "./wasm/melody_wasm";

declare global {
  interface Window {
    MonacoEnvironment: { getWorker: () => Worker };
  }
}

window.MonacoEnvironment = {
  getWorker: () => new editorWorker(),
};

const DEFAULT_EDITOR_SETTINGS: monaco.editor.IStandaloneEditorConstructionOptions =
  {
    theme: "nord",
    automaticLayout: true,
    minimap: { enabled: false },
    wordWrap: "on",
    wrappingIndent: "indent",
    fontFamily: "'Fira Code', monospace",
    fontLigatures: true,
    renderLineHighlight: "none",
    scrollbar: {
      alwaysConsumeMouseWheel: false,
      vertical: "hidden",
      horizontal: "hidden",
    },
  };

// colors taken from the Nord palette (https://www.nordtheme.com)
const nordTheme: monaco.editor.IStandaloneThemeData = {
  base: "vs-dark",
  inherit: false,
  rules: [],
  colors: {
    "editor.background": "#2E3440",
    "editor.foreground": "#ECEFF4",
  },
};

const initEditors = async () => {
  const editorTarget = document.getElementById("editor-container");
  const outputTarget = document.getElementById("output-container");

  if (!editorTarget || !outputTarget) {
    return;
  }

  const initialValue = `16 of "na";

2 of match {
  <space>;
  "batman";
}`;

  monaco.editor.defineTheme("nord", nordTheme);
  monaco.editor.setTheme("nord");

  const editor = monaco.editor.create(editorTarget, {
    value: initialValue,
    ...DEFAULT_EDITOR_SETTINGS,
  });

  const output = monaco.editor.create(outputTarget, {
    value: ``,
    readOnly: true,
    ...DEFAULT_EDITOR_SETTINGS,
  });

  await init();

  const syncEditors = () => {
    try {
      const regex = compiler(editor.getValue());
      output.setValue(regex);
    } catch {
      output.setValue("Parsing error");
    }
  };

  syncEditors();

  editor.getModel()?.onDidChangeContent(syncEditors);
};

initEditors();
