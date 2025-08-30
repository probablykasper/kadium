import { sveltekit } from '@sveltejs/kit/vite'
import { defineConfig } from 'vite'

export default defineConfig({
	clearScreen: false,
	server: { port: 9000, strictPort: true, fs: { allow: ['./bindings.ts'] } },
	build: { sourcemap: true, target: ['chrome64', 'edge79', 'firefox62', 'safari11.1'] },
	plugins: [sveltekit()],
	css: { preprocessorOptions: { sass: { api: 'modern' } } },
})
