import '@tauri-front/shared';
import { loadStyleVariant } from '@tauri-front/shared';
import { bootstrapApplication } from '@angular/platform-browser';
import { appConfig } from './app/app.config';
import { App } from './app/app';

loadStyleVariant('material-design-v3');
bootstrapApplication(App, appConfig).catch((err) => console.error(err));
