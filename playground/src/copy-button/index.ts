enum CopyButtonTextState {
  Copied = 'Copied',
  Error = 'Error',
  Default = 'Copy Link to Source',
}

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
  } catch (error) {
    copyButton.textContent = CopyButtonTextState.Error;
  } finally {
    setTimeout(() => {
      copyButton.textContent = originalText;
    }, 3000);
  }
};

export const initCopyButton = () => {
  copyButton?.addEventListener('click', () => onCopyButtonClick(copyButton));
};
