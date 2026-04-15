import {defineConfig, HeadConfig, resolveSiteDataByRoute} from 'vitepress'
import tailwindcss from '@tailwindcss/vite'
import { enConfig } from '../config'


const siteUrl = 'https://redstone-lang.org'

export default defineConfig({
  title: "Redstone",
  vite: {
    plugins: [tailwindcss()],
  },
  description: "A simple yet powerful statically compiled general-purpose programming language",
  lang: 'en-US',
  head: [['link', { rel: 'icon', href: 'logo.svg' }]],
  lastUpdated: true,

  rewrites: {
    'en/:rest*': ':rest*'
  },

  locales: {
    root: { label: 'English', lang: 'en-US', dir: 'ltr', ...enConfig },
    ru: { label: 'Русский', lang: 'ru-RU', dir: 'ltr' },
  },

  themeConfig: {
    logo: 'logo.svg',
    socialLinks: [
      { icon: 'github', link: 'https://github.com/redstone-lang/redstone' }
    ],
    footer: {
      message: 'Released under the MIT License.',
    },
    search: {
      provider: 'local'
    }
  },

  transformPageData: (pageData, ctx) => {
    const ogImage = new URL('assets/og.jpg', siteUrl).href
    const localeToOgLocaleMap: Record<string, string> = {
      root: 'en_US',
      ru: 'ru_RU',
    }

    const url = new URL(pageData.relativePath.replace(/(?:(^|\/)index)?\.md$/, '$1'), siteUrl).href
    const site = resolveSiteDataByRoute(ctx.siteConfig.site, pageData.relativePath)
    const title = pageData.title ? `${pageData.title} | Redstone` : site.title
    const description = pageData.description || site.description
    const locale = localeToOgLocaleMap[site.localeIndex || 'root']

    ;((pageData.frontmatter.head ??= []) as HeadConfig[]).push(
        ['meta', { property: 'og:url', content: url }],
        ['meta', { property: 'og:title', content: title }],
        ['meta', { property: 'og:description', content: description }],
        ['meta', { property: 'og:type', content: 'website' }],
        ['meta', { property: 'og:locale', content: locale }],
        ['meta', { property: 'og:site_name', content: 'Redstone' }],
        ['meta', { property: 'og:image', content: ogImage }],
        ['meta', { property: 'og:image:secure_url', content: ogImage }],
        ['meta', { property: 'og:image:type', content: 'image/jpeg' }],
        ['meta', { property: 'og:image:width', content: '1280' }],
        ['meta', { property: 'og:image:height', content: '640' }],
        ['meta', { property: 'og:image:alt', content: 'Redstone' }],
        ['link', { rel: 'canonical', href: url }]
    )
  }
})
