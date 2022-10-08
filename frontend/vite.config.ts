import react from '@vitejs/plugin-react';
import { defineConfig } from 'vite';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      // NOTE: yarn add -D @types/node & add node to compilerOptions.types
      '@/': `${__dirname}/src/`,
    },
  },
});
