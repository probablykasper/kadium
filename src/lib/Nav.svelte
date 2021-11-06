<script lang="ts">
  import { active, router } from 'tinro'
  import { tags, viewOptions } from './data'
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
  function toggleTag(tag: string) {
    if ($viewOptions.tag === tag) {
      $viewOptions.tag = null
    } else {
      $viewOptions.tag = tag
    }
  }
</script>

<nav>
  <a on:mousedown={go} use:active data-exact href="/"><button>Videos</button></a>
  <a on:mousedown={go} use:active href="/channels"><button>Channels</button></a>
  <a on:mousedown={go} use:active href="/settings"><button>Settings</button></a>
</nav>
<div class="options-bar">
  <button class="bar-item group" on:keydown={showGroupKeydown} tabindex="0">
    <div class="item" class:selected={show === 0} on:mousedown={() => (show = 0)}>New</div>
    <div class="item" class:selected={show === 1} on:mousedown={() => (show = 1)}>Archived</div>
    <div class="item" class:selected={show === 2} on:mousedown={() => (show = 2)}>All</div>
  </button>
  <input
    class="bar-item"
    type="text"
    placeholder="Channel Filter"
    bind:value={$viewOptions.channel_filter} />
  {#each $tags as tag}
    <button
      class="bar-item tag"
      class:enabled={$viewOptions.tag === tag}
      on:click={() => toggleTag(tag)}>{tag}</button>
  {/each}
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
    box-sizing: border-box
    padding: 0px 20px
    padding-bottom: 10px
    align-items: center
    display: flex
    overflow-y: scroll
    overscroll-behavior-y: none
    overscroll-behavior-x: none
    -webkit-overflow-scrolling: touch
  .bar-item
    border-radius: 3px
    border: 1px solid hsl(233, 7%, 22%)
    background-color: hsla(223, 33%, 64%, 0.12)
    outline: none
    margin: 0px
    margin-right: 12px
    font-size: 13px
    color: inherit
  .group
    display: flex
    padding: 0px
    cursor: default
    height: 26px
    line-height: 26px
    box-sizing: border-box
    transition: all 120ms cubic-bezier(0.4, 0.0, 0.2, 1)
    &:focus
      border-color: hsla(220, 100%, 50%, 1)
      box-shadow: 0px 0px 0px 3px hsla(220, 100%, 50%, 0.5)
      .item.selected
        background-color: hsla(220, 100%, 50%, 1)
    .item
      background-color: transparent
      border: none
      margin: 0px
      padding: 0px 12px
      transition: all 120ms cubic-bezier(0.4, 0.0, 0.2, 1)
      &.selected
        background-color: hsl(225, 14%, 28%)
  button.tag
    height: 23px
    font-size: 12px
    box-sizing: border-box
    padding: 0px 8px
    border-radius: 10px
    margin-right: 5px
    transition: all 120ms cubic-bezier(0.4, 0.0, 0.2, 1)
    &:focus
      border-color: hsla(220, 100%, 50%, 1)
      box-shadow: 0px 0px 0px 3px hsla(220, 100%, 50%, 0.5)
    &.enabled
      background-color: hsl(220, 14%, 28%)
      border: 1px solid hsl(220, 10%, 50%)
      color: #ffffff
    &.enabled:focus
      background-color: hsla(220, 100%, 50%, 1)
      border-color: hsla(220, 100%, 60%, 1)
  input
    height: 26px
    box-sizing: border-box
    padding: 0px 6px
    transition: all 120ms cubic-bezier(0.4, 0.0, 0.2, 1)
    &:focus
      border-color: hsla(220, 100%, 50%, 1)
      box-shadow: 0px 0px 0px 3px hsla(220, 100%, 50%, 0.5)
</style>
