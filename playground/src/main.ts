import { initCopyButton } from './copy-button';

declare global {
  interface Window {
    MonacoEnvironment: { getWorker: () => Worker };
    currentEditorContent: string;
  }
}

initCopyButton();

import('./editors').then((exports) => {
  exports.initEditors();
});
