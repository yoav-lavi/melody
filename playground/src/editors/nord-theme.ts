import { editor } from 'monaco-editor/esm/vs/editor/editor.api';

// colors taken from the Nord palette (https://www.nordtheme.com and https://github.com/arcticicestudio/nord-visual-studio-code/)
export const nordTheme: editor.IStandaloneThemeData = {
  base: 'vs-dark',
  inherit: false,
  rules: [
    { token: 'keyword', foreground: '#81A1C1' },
    { token: 'digit', foreground: '#EBCB8B' },
    { token: 'string', foreground: '#A3BE8C' },
    { token: 'character', foreground: '#EBCB8B' },
    { token: 'comment', foreground: '#616E88' },
    { token: 'variable', foreground: '#D8DEE9' },
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
