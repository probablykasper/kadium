<script lang="ts">
  // @ts-ignore (linter error)
  import Changelog from '../../../CHANGELOG.md'
  import Revealed from './_Revealed.svelte'
</script>

<svelte:head>
  <title>Changelog • Kadium</title>
</svelte:head>

<div class="changelog mx-auto mb-24 max-w-3xl p-1">
  <Revealed options={{ duration: 300, opacity: 0 }}>
    <Changelog />
  </Revealed>
</div>

<style lang="sass">
  .changelog
    :global(h1)
      text-align: center
      margin-top: 5rem
      font-size: 3rem
      font-weight: 800
      background: linear-gradient(130deg,#f844cb 10%,#fd713a 90%)
      background-clip: text
      text-fill-color: transparent
      -webkit-text-fill-color: transparent
    :global(h2)
      font-size: 1.5rem
      font-weight: 500
      margin-top: 3rem
      margin-bottom: 1rem
    :global(li)
      font-size: 1rem
      line-height: 2
      list-style-type: disc
      margin-left: 2rem
</style>
