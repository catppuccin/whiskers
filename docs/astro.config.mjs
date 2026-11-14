// @ts-check
import { defineConfig } from "astro/config";
import { unified } from "@astrojs/markdown-remark";
import starlight from "@astrojs/starlight";
import catppuccin from "@catppuccin/starlight";
import { remarkHeadingId } from "remark-custom-heading-id";
import starlightLinksValidator from "starlight-links-validator";
import starlightChangelogs from "starlight-changelogs";

// https://astro.build/config
export default defineConfig({
  site: "https://whiskers.catppuccin.com",
  markdown: {
    processor: unified({ remarkPlugins: [remarkHeadingId] }),
  },
  integrations: [
    starlight({
      title: "Catppuccin Whiskers",
      favicon: "/favicon.png",
      logo: {
        dark: "/public/pepperjack-dark.png",
        light: "/public/pepperjack-light.png",
      },
      social: [
        {
          icon: "github",
          label: "GitHub",
          href: "https://github.com/catppuccin/whiskers",
        },
      ],
      editLink: {
        baseUrl: "https://github.com/catppuccin/whiskers/edit/main/docs",
      },
      expressiveCode: {
        themes: ["catppuccin-mocha", "catppuccin-latte"],
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
          items: [{ autogenerate: { directory: "getting-started" } }],
        },
        {
          label: "Concepts",
          items: [{ autogenerate: { directory: "concepts" } }],
        },
        {
          label: "Guides",
          items: [{ autogenerate: { directory: "guides" } }],
        },
        {
          label: "Reference",
          items: [{ autogenerate: { directory: "reference" } }],
        },
        {
          label: "Cookbook",
          items: [{ autogenerate: { directory: "cookbook" } }],
        },
        {
          label: "Resources",
          items: [{ autogenerate: { directory: "resources" } }],
        },
        {
          link: "/changelog/",
          label: "Changelog",
        },
      ],
      plugins: [catppuccin(), starlightLinksValidator(), starlightChangelogs()],
    }),
  ],
});
