import { editor, languages } from 'monaco-editor';
import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker';
import {
  DEFAULT_EDITOR_VALUE,
  MELODY_LANGUAGE_ID,
  NORD_THEME_ID,
} from './consts';
import { DEFAULT_EDITOR_SETTINGS } from './editor-settings';
import { languageDefinition } from './language-definition';
import { nordTheme } from './nord-theme';
// build from ../crates/melody_wasm included manually due to issues with wasm-bindgen (similar to https://github.com/rustwasm/wasm-bindgen/issues/113)
import init, { compiler } from '../wasm/melody_wasm';

window.MonacoEnvironment = {
  getWorker: () => new editorWorker(),
};

languages.register({ id: MELODY_LANGUAGE_ID });

languages.setMonarchTokensProvider(MELODY_LANGUAGE_ID, languageDefinition);

editor.defineTheme(NORD_THEME_ID, nordTheme);
editor.setTheme(NORD_THEME_ID);

const urlParams = new URLSearchParams(location.search);

const melodyEditorTarget = document.getElementById('editor-container');
const regexEditorTarget = document.getElementById('output-container');

const getEditorInitialValue = () => {
  const urlParamContent = urlParams.get('content');
  if (urlParamContent !== null) {
    return decodeURIComponent(atob(urlParamContent));
  }
  return DEFAULT_EDITOR_VALUE;
};

const initialValue = getEditorInitialValue();

export const initEditors = async () => {
  if (!melodyEditorTarget || !regexEditorTarget) {
    return;
  }

  melodyEditorTarget.style.display = 'block';
  regexEditorTarget.style.display = 'block';

  const melodyEditor = editor.create(melodyEditorTarget, {
    value: initialValue,
    language: MELODY_LANGUAGE_ID,
    ...DEFAULT_EDITOR_SETTINGS,
  });

  const lines = initialValue.split('\n');
  const lineNumber = lines.length;
  const lastLine = lines[lines.length - 1];
  const column = lastLine.length + 1;

  melodyEditor.setPosition({ lineNumber, column });
  melodyEditor.focus();
  melodyEditor.revealLineInCenter(lineNumber);

  const regexEditor = editor.create(regexEditorTarget, {
    value: '',
    readOnly: true,
    ...DEFAULT_EDITOR_SETTINGS,
  });

  await init();

  const syncEditors = () => {
    try {
      window.currentEditorContent = melodyEditor.getValue();
      const regex = compiler(melodyEditor.getValue());
      regexEditor.setValue(regex);
    } catch (error) {
      regexEditor.setValue(error as string);
    }
  };

  syncEditors();

  melodyEditor.getModel()?.onDidChangeContent(syncEditors);
};
