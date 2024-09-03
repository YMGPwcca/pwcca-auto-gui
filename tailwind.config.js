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
        tier4: '#4B4B4B',
        tier5: '#5A5A5A',
      },

      height: {
        px50: '50px',
        px100: '100px',
        px150: '150px',
        px200: '200px',
        px250: '250px',
        px300: '300px',
        px350: '350px',
        px400: '400px',
        px450: '450px',
        px500: '500px',
        px550: '550px',
        px600: '600px',
        px650: '650px',
      },

      animation: {
        'fade-in': 'fadeIn .3s linear',
        'fade-out': 'fadeOut .3s linear',
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