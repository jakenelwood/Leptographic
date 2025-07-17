/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "*.html",
    "./src/**/*.rs", // Scans all your Rust files
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}
