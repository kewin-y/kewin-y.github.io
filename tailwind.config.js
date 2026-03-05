/** @type {import('tailwindcss').Config} */

export const theme = {
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
};
