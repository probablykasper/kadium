import adapter from '@sveltejs/adapter-static'
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte'
import * as props_preprocess from './props_preprocess.js'

/** @type {import('@sveltejs/kit').Config} */
export default {
	preprocess: [
		props_preprocess.props_preprocess(),
		{
			script(input) {
				if (input.filename.includes('ButtonTest')) {
					// console.log('\n\n\n------script\n', input)
				}
			},
		},
		vitePreprocess({
			script: true,
		}),
	],

	kit: { adapter: adapter() },
}
