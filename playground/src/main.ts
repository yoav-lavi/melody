import { initCopyButton } from './copy-button';

declare global {
  interface Window {
    MonacoEnvironment: { getWorker: () => Worker };
    currentEditorContent: string;
  }
}

initCopyButton();

import('./editors').then((exports) => {
  document.getElementById('editor-loader')?.remove();
  document.getElementById('output-loader')?.remove();
  exports.initEditors();
});
