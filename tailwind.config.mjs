/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}"],
  theme: {
    extend: {
      typography: {
        DEFAULT: {
          css: {
            p: {
              color: "#000000",
            },
          },
        },
      },
    },
  },
  plugins: [require("@tailwindcss/typography")],
};
