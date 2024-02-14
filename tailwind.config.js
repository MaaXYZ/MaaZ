/** @type {import('tailwindcss').Config} */
export default {
  corePlugins: {
    preflight: false
  },
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}"
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}

