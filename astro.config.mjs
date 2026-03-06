// @ts-check
import { defineConfig } from "astro/config";

import mdx from "@astrojs/mdx";

import tailwindcss from "@tailwindcss/vite";

// https://astro.build/config
export default defineConfig({
    site: "https://kewin-y.github.io/",
    integrations: [mdx()],
    markdown: {
        syntaxHighlight: "shiki",
        shikiConfig: {
            theme: "material-theme-lighter",
            wrap: true,
        },
    },

    vite: {
        plugins: [tailwindcss()],
    },
});
