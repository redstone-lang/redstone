import { type DefaultTheme, type LocaleSpecificConfig } from 'vitepress'

export const enConfig: LocaleSpecificConfig<DefaultTheme.Config> = {
  lang: 'en-US',
  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Docs', link: '/docs/' },
    ],
    sidebar: [
      {
        text: 'Guide',
        base: "docs",
        items: [
          { text: 'Introduction', link: '/index' },
          { text: 'Types', link: '/types' },
          { text: 'Variables', link: '/variables' },
          { text: 'Functions', link: '/functions' },
          { text: 'Expressions', link: '/expressions' },
          { text: 'Comments', link: '/comments' },
        ]
      },
      {
        text: 'Control Flow',
        base: "docs/control-flow",
        items: [
          { text: 'Overview', link: '/index' },
          { text: 'If Expressions', link: '/if-expressions' },
          { text: 'Loops', link: '/repetition-with-loops' },
        ]
      }
    ],
    editLink: {
      pattern: 'https://github.com/redstone-lang/redstone/edit/main/docs/:path',
      text: 'Edit this page on GitHub'
    },
    lastUpdated: {
      text: 'Last updated',
    },
    docFooter: {
      prev: 'Previous',
      next: 'Next'
    },
    outline: {
      label: 'On this page'
    },
  }
}
