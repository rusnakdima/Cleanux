import '@tauri-front/shared';
import { loadStyleVariantNoop } from '@tauri-front/shared';
import { bootstrapApplication } from '@angular/platform-browser';
import { appConfig } from './app/app.config';
import { App } from './app/app';

loadStyleVariantNoop();
bootstrapApplication(App, appConfig).catch((err) => console.error(err));
