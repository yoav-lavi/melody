import { editor, languages } from 'monaco-editor';
import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker';
// build from ../crates/melody_wasm included manually due to issues with wasm-bindgen (similar to https://github.com/rustwasm/wasm-bindgen/issues/113)
import init, { compiler } from './wasm/melody_wasm';

declare global {
  interface Window {
    MonacoEnvironment: { getWorker: () => Worker };
  }
}

window.MonacoEnvironment = {
  getWorker: () => new editorWorker(),
};

const MELODY_LANGUAGE_ID = 'melody';

const NORD_THEME_ID = 'nord';

const DEFAULT_EDITOR_SETTINGS: editor.IStandaloneEditorConstructionOptions = {
  theme: NORD_THEME_ID,
  automaticLayout: true,
  minimap: { enabled: false },
  wordWrap: 'on',
  wrappingIndent: 'indent',
  fontFamily: "'Fira Code', monospace",
  fontLigatures: true,
  renderLineHighlight: 'none',
  scrollbar: {
    alwaysConsumeMouseWheel: false,
    vertical: 'hidden',
    horizontal: 'hidden',
  },
};

languages.register({ id: MELODY_LANGUAGE_ID });

languages.setMonarchTokensProvider(MELODY_LANGUAGE_ID, {
  tokenizer: {
    root: [
      [/(of|capture|to|of|some|match|over|option|not|either|any)/, 'keyword'],
      [/\d/, 'digit'],
      [/"(\\"|[^"\n])*"/, 'string'],
      [/'(\\'|[^'\n])*'/, 'string'],
      [/"(\\"|[^"\n])*"/, 'string'],
      [/`(\\`|[^`\n])*`/, 'string'],
      [
        /(<space>|<newline>|<tab>|<return>|<feed>|<null>|<digit>|<word>|<vertical>|<start>|<end>|<char>)/,
        'character',
      ],
      [/[A-Za-z]/, 'character'],
      [/\/\/.*/, 'comment'],
    ],
  },
});

// colors taken from the Nord palette (https://www.nordtheme.com and https://github.com/arcticicestudio/nord-visual-studio-code/)
const nordTheme: editor.IStandaloneThemeData = {
  base: 'vs-dark',
  inherit: false,
  rules: [
    { token: 'keyword', foreground: '#81A1C1' },
    { token: 'digit', foreground: '#EBCB8B' },
    { token: 'string', foreground: '#A3BE8C' },
    { token: 'character', foreground: '#EBCB8B' },
    { token: 'comment', foreground: '#616E88' },
  ],
  colors: {
    foreground: '#D8DEE9',
    'editor.background': '#2E3440',
    'editor.foreground': '#D8DEE9',
    'editorLineNumber.foreground': '#4C566A',
    'editorLineNumber.activeForeground': '#D8DEE9',
    'editorBracketMatch.background': '#2E344000',
    'editorBracketMatch.border': '#88C0D0',
    'editorCursor.foreground': '#D8DEE9',
    'editorWhitespace.foreground': '#4C566AB3',
  },
};

editor.defineTheme(NORD_THEME_ID, nordTheme);
editor.setTheme(NORD_THEME_ID);

const editorInitialValue = `// matches the batman theme tune

16 of "na";

2 of match {
  <space>;
  "batman";
}
`;

const initEditors = async () => {
  const melodyEditorTarget = document.getElementById('editor-container');
  const regexEditorTarget = document.getElementById('output-container');

  if (!melodyEditorTarget || !regexEditorTarget) {
    return;
  }

  const melodyEditor = editor.create(melodyEditorTarget, {
    value: editorInitialValue,
    language: MELODY_LANGUAGE_ID,
    ...DEFAULT_EDITOR_SETTINGS,
  });

  const regexEditor = editor.create(regexEditorTarget, {
    value: '',
    readOnly: true,
    ...DEFAULT_EDITOR_SETTINGS,
  });

  await init();

  const syncEditors = () => {
    try {
      const regex = compiler(melodyEditor.getValue());
      regexEditor.setValue(regex);
    } catch {
      regexEditor.setValue('Parsing error');
    }
  };

  syncEditors();

  melodyEditor.getModel()?.onDidChangeContent(syncEditors);
};

initEditors();
