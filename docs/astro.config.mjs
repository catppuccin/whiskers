// @ts-check
import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";
import catppuccin from "@catppuccin/starlight";

// https://astro.build/config
export default defineConfig({
  site: "https://whiskers.catppuccin.com",
  integrations: [
    starlight({
      title: "Catppuccin Whiskers",
      favicon: "/favicon.png",
      logo: {
        src: "/public/favicon.png",
      },
      social: [
        {
          icon: "github",
          label: "GitHub",
          href: "https://github.com/catppuccin/whiskers",
        },
      ],
      sidebar: [
        {
          label: "Getting Started",
          autogenerate: { directory: "getting-started" },
        },
        {
          label: "Guides",
          autogenerate: { directory: "guides" },
        },
        {
          label: "Reference",
          autogenerate: { directory: "reference" },
        },
        {
          label: "Resources",
          autogenerate: { directory: "resources" },
        },
      ],
      plugins: [catppuccin()],
    }),
  ],
});
