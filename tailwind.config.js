/** @type {import('tailwindcss').Config} */
import screen from "./screen.json";
// var xama = {
//       'tabletyyy': '640px',
//       // => @media (min-width: 640px) { ... }

//       'laptop': '1024px',
//       // => @media (min-width: 1024px) { ... }

//       'desktop': '1280px',
//       // => @media (min-width: 1280px) { ... }
//     };
module.exports = {
  theme: {
    screens: screen,
  },
};

