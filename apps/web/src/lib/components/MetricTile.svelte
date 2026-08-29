<script lang="ts">
  export let title: string = 'metric tile';
  export let value: string | number | null = null;
  export let detail = '';
</script>

<section class="panel" aria-label={title}>
  <header><h3>{title}</h3><slot name="action" /></header>
  {#if value !== null}<strong>{value}</strong>{/if}
  {#if detail}<p>{detail}</p>{/if}
  <slot />
</section>

<style>
  .panel { border: 1px solid #24483a; border-radius: 1rem; padding: 1.25rem; background: #0d2119; }
  header { display: flex; justify-content: space-between; gap: 1rem; align-items: start; }
  h3 { margin: 0; color: #dffbed; }
  strong { display: block; margin-top: .75rem; color: #4ade80; font-size: 2rem; }
  p { color: #a9c9bb; line-height: 1.5; }
</style>
