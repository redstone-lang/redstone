<template>
  <div class="hero-code-wrap relative rounded-2xl px-4 py-4 sm:px-6 sm:py-5
              bg-white/90 dark:bg-black/50 backdrop-blur-md
              border border-white/10
              shadow-[0_8px_32px_rgba(0,0,0,0.35),inset_0_1px_0_rgba(255,255,255,0.08)]
              w-full max-w-xs sm:max-w-sm mx-auto">
    <div v-if="html" v-html="html" class="text-left" />
    <pre v-else class="m-0 text-xs sm:text-sm leading-6 sm:leading-7 font-mono invisible text-left">{{ code }}</pre>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { redstoneGrammar } from '../redstoneGrammar'
import { createHighlighter } from 'shiki'

const html = ref('')

const code = `fn fibonacci(n: u64) -> u64 {
    let a = 1;
    let b = 0;
    let count = 0;

    while count < n {
        let tmp = a + b;
        b = a;
        a = tmp;
        count += 1;
    }

    b
}

fn main() -> i32 {
    print(fibonacci(32));
    return 0;
}`

onMounted(async () => {
  const highlighter = await createHighlighter({
    themes: ['github-light', 'github-dark'],
    langs: [redstoneGrammar],
  })
  html.value = highlighter.codeToHtml(code, {
    lang: 'red',
    themes: { light: 'github-light', dark: 'github-dark' },
    defaultColor: false,
  })
})
</script>

<style>
/* Apply light/dark token colors for runtime-generated Shiki HTML.
   VitePress switches themes by toggling the `dark` class on <html>.
   defaultColor: false means Shiki only emits --shiki-light/--shiki-dark
   custom properties — we must wire them up ourselves. */
.hero-code-wrap .shiki span {
  color: var(--shiki-light);
}
html.dark .hero-code-wrap .shiki span {
  color: var(--shiki-dark);
}
</style>

<style scoped>
:deep(.shiki) {
  background: transparent !important;
  font-family: inherit;
  font-size: 0.75rem;
  line-height: 1.5rem;
}

@media (min-width: 640px) {
  :deep(.shiki) {
    font-size: 0.875rem;
    line-height: 1.75rem;
  }
}

:deep(pre) {
  margin: 0;
  overflow-x: auto;
}


</style>
