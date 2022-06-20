<script lang="ts">
  export let href = ''
  let target = '_self'
  $: if (href.startsWith('/') || href.startsWith('./')) {
    target = '_self'
  } else {
    target = '_blank'
  }
</script>

{#if href === ''}
  <button type="button" on:click><slot /></button>
{:else}
  <a {href} {target}><button type="button" on:click><slot /></button></a>
{/if}

<style lang="sass">
  button
    font-size: 13px
    margin: 0px
    color: hsl(210, 100%, 55%)
    background-color: transparent
    padding: 0px
    border: none
    user-select: none
    cursor: default
    &:hover
      color: hsl(210, 100%, 45%)
  a
    color: inherit
    text-decoration: none
    font-size: inherit
    button
      font-size: inherit
</style>
