export interface FilePreviewData {
  name: string;
  path: string;
  type: 'image' | 'text' | 'binary' | 'unknown' | 'error';
  content?: string;
  imageUrl?: string;
  error?: string;
}
