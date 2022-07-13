import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import eslintPlugin from 'vite-plugin-eslint';
import { join } from 'path';
import requireTransform from 'vite-plugin-require-transform';
import svgr from 'vite-plugin-svgr';

export default defineConfig({
  plugins: [
    react(),
    eslintPlugin(),
    requireTransform({
      fileRegex: /.jsx$/,
    }),
    svgr({}),
  ],
  resolve: {
    alias: {
      '@': join(__dirname, 'src'),
    },
  },
});
