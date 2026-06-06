import { Pipe, PipeTransform } from '@angular/core';
import { formatSize } from '@shared/utils/format.util';

@Pipe({
  name: 'formatSize',
  standalone: true,
  pure: true
})
export class FormatSizePipe implements PipeTransform {
  transform(bytes: number | null | undefined): string {
    if (bytes == null) return '0 B';
    return formatSize(bytes);
  }
}
