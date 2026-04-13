import { type DefaultTheme, type LocaleSpecificConfig } from 'vitepress'

export const enConfig: LocaleSpecificConfig<DefaultTheme.Config> = {
  lang: 'en-US',
  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Docs', link: '/docs' },
    ],
    sidebar: [
      {
        text: 'Guide',
        items: [
          { text: 'Introduction', link: '/en/' },
          { text: 'Types', link: '/en/types' },
          { text: 'Variables', link: '/en/variables' },
          { text: 'Functions', link: '/en/functions' },
          { text: 'Expressions', link: '/en/expressions' },
          { text: 'Comments', link: '/en/comments' },
        ]
      },
      {
        text: 'Control Flow',
        items: [
          { text: 'Overview', link: '/en/control-flow' },
          { text: 'If Expressions', link: '/en/control-flow/if-expressions' },
          { text: 'Loops', link: '/en/control-flow/repetition-with-loops' },
        ]
      }
    ],
    editLink: {
      pattern: 'https://github.com/jkearnsl/redstone/edit/main/docs/:path',
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
