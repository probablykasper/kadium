<script lang="ts">
  import { active, router } from 'tinro'
  import { viewOptions } from './data'
  import { checkShortcut } from './general'

  function go(e: MouseEvent) {
    if (e.target instanceof HTMLElement) {
      const href = e.target.getAttribute('href')
      if (href !== null) {
        e.preventDefault()
        e.stopPropagation()
        e.stopImmediatePropagation()
        router.goto(href, true)
      }
    }
  }
  let show = 0
  $: {
    if (show === 0) {
      $viewOptions.show_all = false
      $viewOptions.show_archived = false
    } else if (show === 1) {
      $viewOptions.show_all = false
      $viewOptions.show_archived = true
    } else {
      $viewOptions.show_all = true
      $viewOptions.show_archived = false
    }
    $viewOptions = $viewOptions
  }
  function showGroupKeydown(e: KeyboardEvent) {
    if (checkShortcut(e, 'ArrowLeft')) {
      show = Math.max(0, show - 1)
      e.preventDefault()
    } else if (checkShortcut(e, 'ArrowRight')) {
      show = Math.min(2, show + 1)
      e.preventDefault()
    }
  }
</script>

<nav>
  <a on:mousedown={go} use:active data-exact href="/"><button>Videos</button></a>
  <a on:mousedown={go} use:active href="/channels"><button>Channels</button></a>
  <a on:mousedown={go} use:active href="/settings"><button>Settings</button></a>
</nav>
<div class="options-bar">
  <button class="group" on:keydown={showGroupKeydown}>
    <div class="item" class:selected={show === 0} on:click={() => (show = 0)}>New</div>
    <div class="item" class:selected={show === 1} on:click={() => (show = 1)}>Archived</div>
    <div class="item" class:selected={show === 2} on:click={() => (show = 2)}>All</div>
  </button>
</div>

<style lang="sass">
  nav
    padding: 0px 20px
    display: flex
    align-items: center
    height: var(--nav-height)
  a
    background-color: transparent
    border: none
    display: inline-block
    margin-right: 15px
    text-decoration: none
    color: hsl(210, 100%, 55%)
    &:hover
      color: hsl(210, 100%, 45%)
    &:global(.active)
      color: hsl(216, 30%, 93%)
    button
      background-color: transparent
      border: none
      font-size: 16px
      color: inherit
      margin: 0px
      padding: 6px 0px
  .options-bar
    height: var(--options-bar-height)
    padding: 0px 20px
    align-items: center
    display: flex
  button.group
    display: flex
    color: inherit
    border: 1px solid hsl(233, 7%, 22%)
    border-radius: 3px
    background-color: hsla(223, 33%, 64%, 0.12)
    padding: 0px
    margin: 0px
    outline: none
    &:focus
      border-color: hsla(220, 100%, 50%, 1)
      box-shadow: 0px 0px 0px 3px hsla(220, 100%, 50%, 0.5)
      .item.selected
        background-color: hsla(220, 100%, 50%, 1)
    .item
      background-color: transparent
      border: none
      font-size: 13px
      margin: 0px
      padding: 4px 12px
      &.selected
        background-color: hsl(225, 14%, 28%)
</style>
