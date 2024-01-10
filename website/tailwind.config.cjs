module.exports = {
	content: ['./src/**/*.{html,js,svelte,ts}'],

	theme: {
		extend: {
			transitionTimingFunction: {
				'out-cubic': 'cubic-bezier(0.33, 1, 0.68, 1)',
			},
		},
	},

	plugins: [require('@tailwindcss/forms')],
}
