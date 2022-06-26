module.exports = {
  content: ['./src/**/*.{html,js,svelte,ts}'],

  theme: {
    extend: {
      transitionTimingFunction: {
        md: 'cubic-bezier(0.4, 0.0, 0.2, 1)',
      },
    },
  },

  plugins: [require('@tailwindcss/forms')],
}
