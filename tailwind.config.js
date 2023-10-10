/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  // theme: {
  //   extend: {},
  // },
  plugins: [require("daisyui")],
  daisyui: {
    themes: ["light", "dark", "retro"],
    logs: false, // Need to disable logs in order for build to succeed. See https://github.com/leptos-rs/cargo-leptos/issues/136
  },
};
