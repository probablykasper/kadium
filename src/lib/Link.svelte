<button on:click><slot /></button>

<style lang="sass">
  button
    font-size: 13px
    vertical-align: middle
    color: hsl(210, 100%, 55%)
    background-color: transparent
    padding: 0px
    border: none
    &:hover
      color: hsl(210, 100%, 45%)
</style>
