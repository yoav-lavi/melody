enum CopyButtonTextState {
  Copied = 'Copied',
  Error = 'Error',
  Default = 'Copy Link to Source',
}

const COPY_BUTTON_HIGHLIGHT = '#3B4252CC';
const COPY_BUTTON_COLOR = '#2E3440';

const copyButton = document.getElementById('copy-button');
const originalText = copyButton?.textContent ?? CopyButtonTextState.Default;

const onCopyButtonClick = async (copyButton: HTMLElement) => {
  try {
    await navigator.clipboard.writeText(
      `${location.origin}?content=${btoa(
        encodeURIComponent(window.currentEditorContent ?? ''),
      )}`,
    );
    copyButton.textContent = CopyButtonTextState.Copied;
    copyButton.style.background = COPY_BUTTON_HIGHLIGHT;
  } catch (error) {
    copyButton.textContent = CopyButtonTextState.Error;
  } finally {
    setTimeout(() => {
      copyButton.textContent = originalText;
      copyButton.style.background = COPY_BUTTON_COLOR;
    }, 3000);
  }
};

export const initCopyButton = () => {
  copyButton?.addEventListener('click', () => onCopyButtonClick(copyButton));
};
