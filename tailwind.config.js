/** @type {import('tailwindcss').Config} */
const round = (num) =>
    num
        .toFixed(7)
        .replace(/(\.[0-9]+?)0+$/, "$1")
        .replace(/\.0$/, "");
const rem = (px) => `${round(px / 16)}rem`;
const em = (px, base) => `${round(px / base)}em`;

module.exports = {
    theme: {
        extend: {
            typography: {
                base: {
                    css: {
                        h1: {
                            fontSize: "var(--text-2xl)",
                            lineHeight: "var(--text-2xl--line-height)",
                            fontWeight: "700",
                        },
                        h2: {
                            fontSize: "var(--text-xl)",
                            lineHeight: "var(--text-xl--line-height)",
                            fontWeight: "700",
                        },
                        h3: {
                            fontSize: "var(--text-lg)",
                            lineHeight: "var(--text-lg--line-height)",
                            fontWeight: "700",
                        },
                    },
                },
            },
        },
    },
};
