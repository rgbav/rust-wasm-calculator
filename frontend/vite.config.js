import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
  plugins: [react()],
  server: {
    fs: {
      // Permit dev-serving the wasm pkg that lives outside this project root.
      allow: ['..'],
    },
  },
});
