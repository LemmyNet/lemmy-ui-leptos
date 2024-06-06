/** @type {import('tailwindcss').Config} */
module.exports = {
  theme: {
    extend: {
      screens: {
        "3xl": "1920px",
        "4xl": "2560px",
        "5xl": "3840px",
        "6xl": "5120px",
        "7xl": "8640px",
      },
      aria: {
        "current-page": 'current="page"',
      },
    },
  },
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  plugins: [require("daisyui")],
  daisyui: {
    themes: ["light", "dark", "retro"],
  },
};
