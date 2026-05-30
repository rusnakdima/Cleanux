export interface FilePreviewConfig {
  width?: string;
  height?: string;
  showOpenWith?: boolean;
  showFileInfo?: boolean;
}

export const DEFAULT_PREVIEW_CONFIG: FilePreviewConfig = {
  width: 'max-w-4xl',
  height: 'h-[80vh]',
  showOpenWith: true,
  showFileInfo: true,
};
