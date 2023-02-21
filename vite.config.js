import { defineConfig } from 'vite'
import { svelte, vitePreprocess } from '@sveltejs/vite-plugin-svelte'
import autoprefixer from 'autoprefixer'

export default defineConfig({
  root: './src',
  base: './', // use relative paths
  publicDir: '../public',
  clearScreen: false,
  server: {
    port: 9000,
  },
  build: {
    outDir: '../build',
    emptyOutDir: true,
    minify: false,
    sourcemap: true,
    target: ['chrome64', 'edge79', 'firefox62', 'safari11.1'],
  },
  plugins: [
    svelte({
      preprocess: vitePreprocess({
        pug: {
          pretty: true,
        },
        postcss: {
          plugins: autoprefixer,
        },
      }),
    }),
  ],
})
