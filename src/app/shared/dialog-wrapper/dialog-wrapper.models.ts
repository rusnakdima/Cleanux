export interface DialogService {
  isOpen(): boolean;
  getData(): unknown;
  close(): void;
}

export abstract class DialogWrapperBase {
  abstract isOpen(): boolean;
  abstract getData(): unknown;
  abstract close(): void;
}
