<script lang="ts">
  import { checkShortcut } from './general'

  export let checked = false
  export let offColor = 'hsl(220, 20%, 31%)'
  export let onColor = 'hsl(220, 100%, 52%)'
  export let id: string
  $: color = checked ? onColor : offColor

  function keydown(e: KeyboardEvent) {
    if (checkShortcut(e, ' ')) {
      checked = !checked
    }
  }
</script>

<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
<label class="switch" class:checked tabindex="0" on:keydown={keydown}>
  <input class="hidden" type="checkbox" bind:checked {id} />
  <div class="box" style:background-color={color} style:border-color={color}>
    <div class="handle" />
  </div>
</label>

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
  .switch:focus-visible .box
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
