/** @type {import('tailwindcss').Config} */
module.exports = {
  theme: {
    extend: {
      gridTemplateAreas: {
        "post-listing-list": [
          "vote thumbnail title",
          "vote thumbnail to",
          "vote thumbnail actions",
        ],
        "post-listing-list-mobile": [
          "title title title thumbnail",
          "to to to thumbnail",
          "vote actions actions thumbnail",
        ],
      },
      gridTemplateColumns: {
        "post-listing-list": "1fr 3fr auto",
        "post-listing-list-mobile": "1fr 2fr auto min-content",
      },
      gridAutoRows: {
        "post-listing-list": "1fr 2fr 1fr",
        "post-listing-list-mobile": "1fr 1fr 1fr",
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
  plugins: [require("daisyui"), require("@savvywombat/tailwindcss-grid-areas")],
  daisyui: {
    themes: ["light", "dark", "retro"],
  },
};
