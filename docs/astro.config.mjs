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
      expressiveCode: {
        themes: ["catppuccin-mocha", "catppuccin-latte"],
        // Stop it from auto-correcting colour contrast
        minSyntaxHighlightingColorContrast: 0,
        styleOverrides: {
          frames: {
            tooltipSuccessBackground: "var(--green)",
            tooltipSuccessForeground: "var(--base)",
          },
          textMarkers: {
            insBackground:
              "color-mix(in oklab, var(--sl-color-green-high) 25%, var(--sl-color-gray-6));",
            insBorderColor: "var(--sl-color-gray-5)",
            delBackground:
              "color-mix(in oklab, var(--sl-color-red-high) 25%, var(--sl-color-gray-6));",
            delBorderColor: "var(--sl-color-gray-5)",
          },
          codeBackground: "var(--sl-color-gray-6)",
        },
      },
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
