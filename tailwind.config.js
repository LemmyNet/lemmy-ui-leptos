/** @type {import('tailwindcss').Config} */
module.exports = {
  theme: {
    extend: {
      screens: {
        '3xl': '1920px',
        '4xl': '3840px',
        '5xl': '5120px',
        '8xl': '8640px',
      },
    },
  },
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  plugins: [require("daisyui")],
  daisyui: {
    themes: ["light", "dark", "retro"],
    logs: false, // Need to disable logs in order for build to succeed. See https://github.com/leptos-rs/cargo-leptos/issues/136
  },
};
