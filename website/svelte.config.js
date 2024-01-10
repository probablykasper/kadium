import adapter from '@sveltejs/adapter-auto'
import { vitePreprocess } from '@sveltejs/kit/vite'
import { mdsvex, defineMDSveXConfig } from 'mdsvex'

const mdsvexConfig = defineMDSveXConfig({
	extensions: ['.svelte.md', '.md', '.svx'],

	smartypants: {
		dashes: 'oldschool',
	},

	remarkPlugins: [],
	rehypePlugins: [],
})

/** @type {import('@sveltejs/kit').Config} */
const config = {
	extensions: ['.svelte', ...mdsvexConfig.extensions],
	preprocess: [vitePreprocess(), mdsvex(mdsvexConfig)],

	kit: {
		adapter: adapter(),
	},
}

export default config
