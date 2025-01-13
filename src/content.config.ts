import { z, defineCollection } from "astro:content";
import { glob } from "astro/loaders";

const blogs = defineCollection({
  loader: glob({ pattern: "**/*.{md,mdx}", base: "./src/content/blogs" }),
  schema: z.object({
    title: z.string(),
    permalink: z.string().optional(),
  }),
});

// Expose your defined collection to Astro
// with the `collections` export
export const collections = { blogs };
