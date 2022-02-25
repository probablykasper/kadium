<script lang="ts">
  export let checked = false
  export let offColor = 'hsl(220, 20%, 31%)'
  export let onColor = 'hsl(220, 100%, 52%)'
  $: color = checked ? onColor : offColor
</script>

<div class="switch" class:checked tabindex="0">
  <input class="hidden" type="checkbox" bind:checked />
  <div
    class="box"
    style:background-color={color}
    style:border-color={color}
    on:click={() => (checked = !checked)}
    tabindex="-1"
  >
    <div class="handle" />
  </div>
</div>

<style lang="sass">
  .hidden
    display: none
  .switch
    outline: none
  .box
    position: relative
    width: 38px
    height: 20px
    border-radius: 100px
    outline: none
    border: 1px solid
    transition: 200ms cubic-bezier(.4,0,.2,1)
    transition-property: background-color, border-color, box-shadow
  .switch:focus .box
    // box-shadow: 0px 0px 2px 0px onColor
    border-color: hsla(220, 100%, 50%, 1)
    box-shadow: 0px 0px 0px 3px hsla(220, 100%, 50%, 0.5)
  .handle
    position: absolute
    top: 2px
    left: 2px
    width: 16px
    height: 16px
    border-radius: 50%
    background-color: #fff
    transition: 200ms cubic-bezier(.4,0,.2,1) transform
  .checked .handle
    transform: translateX(18px)
</style>
