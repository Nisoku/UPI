export default {
  title: "UPI",
  url: "https://nisoku.org/UPI",
  logo: { alt: "UPI", href: "./" },
  favicon: "",
  theme: {
    name: "ruby",
    defaultMode: "system",
    enableModeToggle: true,
    positionMode: "top",
    codeHighlight: true,
    customCss: ['/assets/css/theme.css'],
    copyWidgets: {
      enabled: true,
      raw: true,
      context: true,
    },
  },
  layout: {
    footer: {
      style: "complete",
      description: "One command to install anything, anywhere.",
      branding: true,
      columns: [
        {
          title: "Resources",
          links: [
            { text: "Getting Started", url: "./getting-started/quickstart" },
            { text: "CLI Reference", url: "./cli/" },
            { text: "Architecture", url: "./reference/architecture" },
          ],
        },
        {
          title: "Community",
          links: [
            { text: "GitHub", url: "https://github.com/Nisoku/UPI" },
            { text: "Issues", url: "https://github.com/Nisoku/UPI/issues" },
            { text: "Discussions", url: "https://github.com/Nisoku/UPI/discussions" },
          ],
        },
      ],
    },
  },
  plugins: {
    search: {
      semantic: true,
      showConfidence: true,
    },
    seo: {
      defaultDescription:
        "UPI is a cross-OS macro installer. One command to install any package on any platform.",
      openGraph: { defaultImage: "" },
      twitter: { cardType: "summary_large_image" },
    },
    sitemap: {
      defaultChangefreq: "weekly",
      defaultPriority: 0.8,
    },
    mermaid: {},
    git: {},
    llms: {
      fullContext: true,
    },
  },
  search: true,
  minify: true,
  autoTitleFromH1: true,
  copyCode: true,
  pageNavigation: true,
  navigation: [
    { title: "Home", path: "/", icon: "home" },
    {
      title: "CLI Reference",
      icon: "terminal",
      path: "/cli/",
      collapsible: false,
    },
    {
      title: "Getting Started",
      icon: "rocket",
      collapsible: false,
      children: [
        { title: "Quick Start", path: "/getting-started/quickstart", icon: "play" },
        { title: "Installation", path: "/getting-started/installation", icon: "download" },
        { title: "Core Concepts", path: "/getting-started/concepts", icon: "book" },
      ],
    },
    {
      title: "Guide",
      icon: "book-open",
      collapsible: false,
      children: [
        { title: "Resolution Pipeline", path: "/guide/resolution", icon: "git-merge" },
        { title: "Supported Platforms", path: "/guide/platforms", icon: "globe" },
      ],
    },
    {
      title: "Reference",
      icon: "file-text",
      collapsible: false,
      children: [
        { title: "Database", path: "/reference/database", icon: "database" },
        { title: "Architecture", path: "/reference/architecture", icon: "box" },
      ],
    },
    {
      title: "GitHub",
      path: "https://github.com/Nisoku/UPI",
      icon: "github",
      external: true,
    },
  ],
  footer: "Built with [docmd](https://docmd.io). [View on GitHub](https://github.com/Nisoku/UPI).",
  editLink: {
    enabled: true,
    baseUrl: "https://github.com/Nisoku/UPI/edit/main/Docs/docs",
    text: "Edit this page",
  },
};
