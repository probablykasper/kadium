import { defineConfig } from 'vite'
import { svelte, vitePreprocess } from '@sveltejs/vite-plugin-svelte'
import autoprefixer from 'autoprefixer'

export default defineConfig({
  clearScreen: false,
  server: {
    port: 9000,
    strictPort: true,
  },
  build: {
    sourcemap: true,
    target: ['chrome64', 'edge79', 'firefox62', 'safari11.1'],
  },
  plugins: [
    svelte({
      preprocess: vitePreprocess({
        postcss: {
          plugins: autoprefixer,
        },
      }),
    }),
  ],
})
