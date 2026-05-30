import { defineConfig } from 'vitest/config';
import { resolve } from 'path';

export default defineConfig({
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: ['src/test-setup.ts'],
    include: ['src/**/*.spec.ts'],
    exclude: [
      'src/app/components/**/*.spec.ts',
      'src/app/shared/**/*.spec.ts',
    ],
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'html'],
      include: ['src/app/**/*.ts'],
      exclude: ['src/**/*.module.ts', 'src/**/*.routes.ts'],
    },
  },
  resolve: {
    alias: {
      '@components': resolve(__dirname, 'src/app/components'),
      '@views': resolve(__dirname, 'src/app/views'),
      '@models': resolve(__dirname, 'src/app/models'),
      '@services': resolve(__dirname, 'src/app/services'),
      '@helpers': resolve(__dirname, 'src/app/helpers'),
      '@guards': resolve(__dirname, 'src/app/guards'),
      '@shared': resolve(__dirname, 'src/app/shared'),
      '@app': resolve(__dirname, 'src/app'),
      '@assets': resolve(__dirname, 'src/assets'),
      '@styles': resolve(__dirname, 'src/styles'),
      '@env': resolve(__dirname, 'src/environments'),
      '@api': resolve(__dirname, 'src/app/api'),
      '@stores': resolve(__dirname, 'src/app/stores'),
      '@interceptors': resolve(__dirname, 'src/app/interceptors'),
      '@resolvers': resolve(__dirname, 'src/app/resolvers'),
      '@test-utils': resolve(__dirname, 'src/test-utils'),
    },
  },
});
