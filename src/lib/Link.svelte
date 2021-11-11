<script lang="ts">
  export let href = ''
</script>

{#if href === ''}
  <button type="button" on:click><slot /></button>
{:else}
  <a {href}><button type="button" on:click><slot /></button></a>
{/if}

<style lang="sass">
  button
    font-size: 13px
    margin: 0px
    vertical-align: middle
    color: hsl(210, 100%, 55%)
    background-color: transparent
    padding: 0px
    border: none
    &:hover
      color: hsl(210, 100%, 45%)
  a
    color: inherit
    text-decoration: none
    font-size: inherit
    button
      font-size: inherit
      vertical-align: baseline
</style>
