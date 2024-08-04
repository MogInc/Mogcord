/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./**/*.{html,js,rs}"],
  theme: {
    extend: {
      colors: {
        transparent: "transparent",
        current: "currentColor",
        amber: "#FFBF00",
      },
      animation: {
        shake: "shake 0.15s cubic-bezier(.36,.07,.19,.97) 2",
      },
      keyframes: {
        shake: {
          "10%, 90%": {
            transform: "translate3d(-15px, 0, 0)",
          },
          "20%, 80%": {
            transform: "translate3d(9px, 0, 0)",
          },
          "30%, 50%, 70%": {},
          transform: "translate3d(-13px, 0, 0)",
        },
        "40%, 60%": {
          transform: "translate3d(4px, 0, 0)",
        },
      },
    },
  },
  plugins: [],
};