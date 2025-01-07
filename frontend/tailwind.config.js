/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
    transform: {
      rs: (content) => content.replace(/(?:^|\s)class:/g, " "),
    },
  },
  theme: {
    extend: {
      backgroundImage: {
        "home-dragon": "url('/images/background-home.jpg')",
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
        },
        "home-bg": {
          100: "#67536c",
          200: "#68546d",
          300: "#645069",
          400: "#604e68",
          500: "#54445e",
          600: "#4a3d56",
          700: "#3f344c",
          800: "#393048",
          900: "#3d324a",
        },
        "home-content-bg": {
          100: "#544e72",
          200: "#2c243b",
          300: "#282039",
          400: "#2e2541",
          500: "#362944",
          600: "#3d2846",
          700: "#3e2647",
          800: "#443752",
          900: "#492f4e",
        },
      },
    },
  },
  plugins: [],
};
