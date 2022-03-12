import { editor } from 'monaco-editor/esm/vs/editor/editor.api';
import { NORD_THEME_ID } from './consts';

export const DEFAULT_EDITOR_SETTINGS: editor.IStandaloneEditorConstructionOptions =
  {
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
