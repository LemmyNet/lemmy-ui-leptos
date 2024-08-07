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
      keyframes: {
        "color-cycle": {
          "0%, 100%": { color: "#f87171" },
          "6%": { color: "#fb923c" },
          "12%": { color: "#fbbf24" },
          "18%": { color: "#facc15" },
          "24%": { color: "#a3e635" },
          "30%": { color: "#4ade80" },
          "36%": { color: "#34d399" },
          "42%": { color: "#2dd4bf" },
          "48%": { color: "#22d3ee" },
          "54%": { color: "#38bdf8" },
          "60%": { color: "#60a5fa" },
          "66%": { color: "#818cf8" },
          "72%": { color: "#a78bfa" },
          "78%": { color: "#c084fc" },
          "84%": { color: "#e879f9" },
          "90%": { color: "#f472b6" },
          "95%": { color: "#fb7185" },
        },
      },
      animation: {
        "color-cycle": "color-cycle 6s linear infinite",
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
