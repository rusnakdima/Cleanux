import { Component, input, output, signal } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatIconModule } from '@angular/material/icon';

export interface WidgetConfig {
  id: string;
  title: string;
  icon: string;
  enabled: boolean;
  order: number;
}

@Component({
  selector: 'app-widget-container',
  standalone: true,
  imports: [CommonModule, MatIconModule],
  templateUrl: './widget-container.component.html',
  styleUrls: ['./widget-container.component.css'],
})
export class WidgetContainerComponent {
  widgets = input<WidgetConfig[]>([]);
  widgetVisibilityChange = output<{ id: string; enabled: boolean }>();
  widgetOrderChange = output<WidgetConfig[]>();

  draggingWidget = signal<string | null>(null);
  dragOverWidget = signal<string | null>(null);

  onDragStart(event: DragEvent, widgetId: string) {
    this.draggingWidget.set(widgetId);
    if (event.dataTransfer) {
      event.dataTransfer.effectAllowed = 'move';
      event.dataTransfer.setData('text/plain', widgetId);
    }
  }

  onDragEnd() {
    this.draggingWidget.set(null);
    this.dragOverWidget.set(null);
  }

  onDragOver(event: DragEvent, widgetId: string) {
    event.preventDefault();
    if (event.dataTransfer) {
      event.dataTransfer.dropEffect = 'move';
    }
    this.dragOverWidget.set(widgetId);
  }

  onDragLeave() {
    this.dragOverWidget.set(null);
  }

  onDrop(event: DragEvent, targetWidgetId: string) {
    event.preventDefault();
    const draggedWidgetId = this.draggingWidget();

    if (!draggedWidgetId || draggedWidgetId === targetWidgetId) {
      this.dragOverWidget.set(null);
      return;
    }

    const widgets = [...this.widgets()];
    const draggedIndex = widgets.findIndex((w) => w.id === draggedWidgetId);
    const targetIndex = widgets.findIndex((w) => w.id === targetWidgetId);

    if (draggedIndex !== -1 && targetIndex !== -1) {
      const [draggedWidget] = widgets.splice(draggedIndex, 1);
      widgets.splice(targetIndex, 0, draggedWidget);

      const reorderedWidgets = widgets.map((w, index) => ({
        ...w,
        order: index,
      }));

      this.widgetOrderChange.emit(reorderedWidgets);
    }

    this.draggingWidget.set(null);
    this.dragOverWidget.set(null);
  }

  toggleWidget(widgetId: string, enabled: boolean) {
    this.widgetVisibilityChange.emit({ id: widgetId, enabled });
  }
}
