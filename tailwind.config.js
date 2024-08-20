/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    backgroundColor: {
      tier0: '#0F0F0F',
      tier1: '#1E1E1E',
      tier2: '#2D2D2D',
      tier3: '#3C3C3C',
    }
  }
}