/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    screens: {
      pc: { raw: '(hover: hover) and (pointer: fine)' },
      mobile: { raw: '(hover: none), (pointer: coarse)' }
    },

    extend: {
      textColor: {
        tier0: '#AFAFAF',
      },

      colors: {
        tier0: '#0F0F0F',
        tier1: '#1E1E1E',
        tier2: '#2D2D2D',
        tier3: '#3C3C3C',
      },

      animation: {
        'fade-in': 'fadeIn 1s linear',
        'fade-out': 'fadeOut .8s linear',
      },

      keyframes: {
        fadeIn: {
          '0%': { opacity: 0 },
          '100%': { opacity: 1 },
        },
        fadeOut: {
          '0%': { opacity: 1 },
          '100%': { opacity: 0 },
        },
      },
    }
  }
}