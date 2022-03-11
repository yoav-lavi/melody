import { initCopyButton } from './copy-button';
import { initEditors } from './editors';

declare global {
  interface Window {
    MonacoEnvironment: { getWorker: () => Worker };
    currentEditorContent: string;
  }
}

initCopyButton();
initEditors();
