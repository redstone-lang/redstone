<template>
  <div class="relative rounded-2xl px-6 py-5 text-sm leading-7 font-mono
              bg-white/90 dark:bg-black/50 backdrop-blur-md
              border border-white/10
              shadow-[0_8px_32px_rgba(0,0,0,0.35),inset_0_1px_0_rgba(255,255,255,0.08)]
              min-w-100">
    <pre class="m-0 overflow-x-auto"><code><span class="kw">fn</span> <span class="name">fibonacci</span>(<span>n</span>: <span class="type">u64</span>) -&gt; <span class="type">u64</span> {
    <span class="kw">let</span> <span>a</span> = <span class="num">1</span>;
    <span class="kw">let</span> <span>b</span> = <span class="num">0</span>;
    <span class="kw">let</span> <span>count</span> = <span class="num">0</span>;

    <span class="kw">while</span> <span>count</span> &lt; <span>n</span> {
        <span class="kw">let</span> <span>tmp</span> = <span>a</span> + <span>b</span>;
        <span>b</span> = <span>a</span>;
        <span>a</span> = <span>tmp</span>;
        <span>count</span> += <span class="num">1</span>;
    }

    <span>b</span>
}

<span class="kw">fn</span> <span class="name">main</span>() -&gt; <span class="type">i32</span> {
    <span class="name">print</span>(<span class="name">fibonacci</span>(<span class="num">32</span>));
    <span class="kw">return</span> <span class="num">0</span>;
}</code></pre>
  </div>
</template>

<style scoped>
.kw   { color: #ac6a6a; }
.dark .kw   { color: #ea9292; }
.name { color: #546ea5; }
.dark .name { color: #82aaff; }
.type { color: #ae8a48; }
.dark .type { color: #ffcb6b; }
.num  { color: #af5b42; }
.dark .num  { color: #f78c6c; }
</style>
