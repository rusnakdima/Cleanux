import { Injectable, inject } from '@angular/core';
import { SchemaRouterService } from '@tauri-front/shared';
import { TauriApiService } from '@api/tauri-api.service';
import type { UiSchema } from '@tauri-front/shared';

const SCHEMA_ID = 'cleanux-v1.0';

@Injectable({
  providedIn: 'root',
})
export class SchemaService {
  private readonly schemaRouter = inject(SchemaRouterService);
  private readonly tauriApi = inject(TauriApiService);

  async loadSchema(): Promise<void> {
    try {
      const response = await this.tauriApi.invoke<{ data: UiSchema }>('getSchema', {
        id: SCHEMA_ID,
      });

      if (response?.data) {
        this.schemaRouter.setSchema(response.data);
        console.log('Schema loaded successfully:', SCHEMA_ID);
      }
    } catch (error) {
      console.warn('Failed to load schema from DB, schema will be loaded on-demand:', error);
    }
  }

  async saveSchema(schema: UiSchema): Promise<void> {
    await this.tauriApi.invoke('saveSchema', { schema });
  }

  async getSchema(id: string = SCHEMA_ID): Promise<UiSchema | null> {
    try {
      const response = await this.tauriApi.invoke<{ data: UiSchema }>('getSchema', { id });
      return response?.data ?? null;
    } catch {
      return null;
    }
  }
}
