import { Environment } from 'monaco-editor';
import { initCopyButton } from './copy-button';

declare global {
  interface Window {
    MonacoEnvironment?: Environment;
    currentEditorContent: string;
  }
}

initCopyButton();

import('./editors').then((exports) => {
  document.getElementById('editor-loader')?.remove();
  document.getElementById('output-loader')?.remove();
  exports.initEditors();
});
