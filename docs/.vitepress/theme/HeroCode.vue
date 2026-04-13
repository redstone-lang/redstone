<template>
  <div class="hero-code">
    <pre><code><span class="kw">fn</span> <span class="name">fibonacci</span>(<span class="param">n</span>: <span class="type">u64</span>) -&gt; <span class="type">u64</span> {
    <span class="kw">let</span> <span class="param">a</span> = <span class="num">1</span>;
    <span class="kw">let</span> <span class="param">b</span> = <span class="num">0</span>;
    <span class="kw">let</span> <span class="param">count</span> = <span class="num">0</span>;

    <span class="kw">while</span> <span class="param">count</span> &lt; <span class="param">n</span> {
        <span class="kw">let</span> <span class="param">tmp</span> = <span class="param">a</span> + <span class="param">b</span>;
        <span class="param">b</span> = <span class="param">a</span>;
        <span class="param">a</span> = <span class="param">tmp</span>;
        <span class="param">count</span> += <span class="num">1</span>;
    }

    <span class="param">b</span>
}

<span class="kw">fn</span> <span class="name">main</span>() -&gt; <span class="type">i32</span> {
    <span class="name">print</span>(<span class="name">fibonacci</span>(<span class="num">32</span>));
    <span class="kw">return</span> <span class="num">0</span>;
}</code></pre>
  </div>
</template>

<style scoped>
.hero-code {
  z-index: 10;
  border-radius: 12px;
  background: var(--vp-code-block-bg);
  padding: 24px;
  font-size: 14px;
  line-height: 1.7;
  box-shadow: 0 4px 32px rgba(0, 0, 0, 0.2);
  min-width: 320px;
}

pre {
  margin: 0;
  overflow-x: auto;
}

code {
  font-family: var(--vp-font-family-mono);
}

.kw   { color: #ac6a6a; }
.dark .kw   { color: #ea9292; }
.name { color: #546ea5; }
.dark .name { color: #82aaff; }
.type { color: #ae8a48; }
.dark .type { color: #ffcb6b; }
.param{ color: currentColor; }
.num  { color: #af5b42; }
.dark .num  { color: #f78c6c; }
</style>
