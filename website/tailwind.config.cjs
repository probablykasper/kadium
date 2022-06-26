const defaultTheme = require('tailwindcss/defaultTheme')

const config = {
  content: ['./src/**/*.{html,js,svelte,ts}'],

  theme: {
    extend: {
      transitionTimingFunction: {
        md: 'cubic-bezier(0.4, 0.0, 0.2, 1)',
      },
      screens: {
        xs: '440px',
        ...defaultTheme.screens,
      },
    },
  },

  plugins: [require('@tailwindcss/forms')],
}

module.exports = config
