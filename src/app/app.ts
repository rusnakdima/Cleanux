import { Component } from '@angular/core';
import { SchemaShellComponent } from '@tauri-front/shared';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [SchemaShellComponent],
  template: '<lib-schema-shell appId="cleanux" />',
})
export class AppComponent {}
