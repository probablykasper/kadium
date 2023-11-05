<script lang="ts">
  import Revealed from './Revealed.svelte'
  import { page } from '$app/stores'
</script>

<svelte:head>
  <title>Changelog • Kadium</title>
</svelte:head>

<main class="relative mx-auto mb-24 max-w-3xl p-6">
  <Revealed
    class="mt-20 mb-4 text-center transition-all ease-out"
    options={{ opacity: 0, scale: 0.9, duration: 1000 }}
  >
    <h1 class="text-7xl font-extrabold">{$page.status}</h1>
  </Revealed>
  <Revealed
    class="mb-12 text-center transition-all ease-out"
    options={{ opacity: 0, y: -10, duration: 750, delay: 500 }}
  >
    <p class="text-xl text-green-100 opacity-60 transition-all duration-500 ease-out">
      {#if $page.status === 404}
        This page did not survive the nuclear fallout
      {:else}
        {$page.error?.message || 'Something went wrong'}
      {/if}
    </p>
  </Revealed>
</main>

<style lang="sass">
  h1
    margin: auto
    font-size: 3rem
    font-weight: 800
    background: linear-gradient(130deg,#44bff8 10%,#3afd7b 90%)
    background-clip: text
    text-fill-color: transparent
    -webkit-text-fill-color: transparent
    position: relative

  h1::before
    content: ''
    pointer-events: none
    position: absolute
    inset: 0
    will-change: contents
    animation: show-gradient 1s ease-out forwards
    opacity: 0
    animation-delay: 300ms
    @keyframes show-gradient
      0%
        opacity: 0
      100%
        opacity: 0.7
    width: 100%
    height: 100%
    filter: blur(80px)
    transform: scale(1.2)
    background: conic-gradient(from 150deg at 50% 50%,#00ff4d 0deg,#19e7f5 90deg,#1fa2ff 180deg,#00ffee 270deg,#1fffbf 360deg)
</style>
