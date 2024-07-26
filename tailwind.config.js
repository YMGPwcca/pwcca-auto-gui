/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      screens: {
        pc: {
          raw: '(min-width: 1366px) and (min-height: 768px) and (hover: hover) and (pointer: fine)'
        },
        mobile: {
          raw: '(max-width: 1365px), (max-height: 767px), (hover: none), (pointer: coarse)'
        }
      },
      colors: {
        gray: {
          900: '#202225',
          800: '#2f3136',
          700: '#36393f',
          600: '#4f545c',
          400: '#d4d7dc',
          300: '#e3e5e8',
          200: '#ebedef',
          100: '#f2f3f5'
        }
      },
      spacing: {
        88: '22rem'
      },
      keyframes: {
        slideRightOut: {
          '0%': {
            opacity: 100,
            transform: 'translateX(0px)',
          },
          '100%': {
            opacity: 0,
            transform: 'translateX(20px)',
          },
        },
      },
      animation: {
        slideRightOut: 'slideRightOut 0.5s ease-in-out',
      },
    }
  }
}