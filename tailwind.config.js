/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/templates/**/*.html"],
    transform: {
      rs: (content) => content.replace(/(?:^|\s)class:/g, " "),
    },
  },
  theme: {
    extend: {
      backgroundImage: {
        "image-home": "url('/public/images/home-bg.png')",
      },
      boxShadow: {
        "3xl": "0px 0px 2rem 1px #000000;",
      },
      fontSize: {
        "2xs": "0.6rem",
      },
      colors: {
        "vallheru-creme": {
          100: "#fcf7df",
          200: "#dbd3b1",
          300: "#d4c0a2",
          400: "#d0cabd",
        },
        game: {
          text: "#efec95",
          link: "#bfbfbf",
          green: "#53cf29",
          red: "#bf0000",
          border: "#c9af90",
        },
      },
    },
  },
  plugins: [],
};
