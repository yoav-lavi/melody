// @ts-check

const darkCodeTheme = require("prism-react-renderer/themes/dracula");

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: "Melody",
  tagline:
    "A language that compiles to regular expressions and aims to be more easily readable and maintainable",
  url: "https://yoav-lavi.github.io/",
  baseUrl: "/melody/",
  onBrokenLinks: "throw",
  favicon: "/static/favicon.ico",
  onBrokenMarkdownLinks: "warn",
  organizationName: "yoav-lavi",
  projectName: "melody",
  trailingSlash: true,
  presets: [
    [
      "classic",
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          sidebarPath: require.resolve("./sidebars.js"),
        },
        theme: {
          customCss: require.resolve("./src/css/custom.css"),
        },
      }),
    ],
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      navbar: {
        items: [
          {
            href: "/docs/intro",
            label: "Melody Documentation",
            position: "left",
          },
          {
            href: "https://github.com/yoav-lavi/melody",
            label: "GitHub",
            position: "right",
          },
        ],
      },
      footer: {
        links: [
          {
            html: `<code style="color: #ECEFF4; background-color: #2E3440;">v0.13.3</code>`,
          },
        ],
        style: "dark",
        copyright: `Copyright © ${new Date().getFullYear()} Yoav Lavi`,
      },
      prism: {
        theme: darkCodeTheme,
        darkTheme: darkCodeTheme,
      },
      colorMode: {
        disableSwitch: true,
        defaultMode: "dark",
      },
    }),
};

module.exports = config;
