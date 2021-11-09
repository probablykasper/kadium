<script lang="ts">
  import { onDestroy } from 'svelte'
  import { tags, totalVideos, videos, viewOptions } from '../lib/data'
  import { checkShortcut } from '../lib/general'
  import { event } from '@tauri-apps/api'

  let show = 0
  $: {
    if ($viewOptions.show_all) {
      show = 2
    } else if ($viewOptions.show_archived) {
      show = 1
    } else {
      show = 0
    }
  }
  function show0() {
    $viewOptions.show_all = false
    $viewOptions.show_archived = false
  }
  function show1() {
    $viewOptions.show_all = false
    $viewOptions.show_archived = true
  }
  function show2() {
    $viewOptions.show_all = true
    $viewOptions.show_archived = false
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

  let filterInput: HTMLInputElement
  const unlistenFuture = event.listen('menu', ({ payload }) => {
    if (payload === 'Find') {
      filterInput.focus()
    }
  })
  onDestroy(async () => {
    const unlisten = await unlistenFuture
    unlisten()
  })
</script>

<header>
  <div class="options-bar">
    <button class="bar-item control-style group" on:keydown={showGroupKeydown} tabindex="0">
      <div class="item" class:selected={show === 0} on:mousedown={show0}>New</div>
      <div class="item" class:selected={show === 1} on:mousedown={show1}>Archived</div>
      <div class="item" class:selected={show === 2} on:mousedown={show2}>All</div>
    </button>
    <input
      bind:this={filterInput}
      class="bar-item control-style"
      type="text"
      placeholder="Channel Filter"
      bind:value={$viewOptions.channel_filter} />
    {#each $tags as tag}
      <button
        class="bar-item control-style tag"
        class:enabled={$viewOptions.tag === tag}
        on:click={() => toggleTag(tag)}>{tag}</button>
    {/each}
  </div>
  <div class="bar-item page-info">
    {#if $totalVideos === null}
      {$videos.length} of ?
    {:else}
      {$videos.length} of {$totalVideos}
    {/if}
  </div>
</header>

<style lang="sass">
  header
    display: flex
    align-items: center
    width: 100%
    box-sizing: border-box
    padding: 0px 20px
    justify-content: space-between
    flex-shrink: 0
  .bar-item
    margin-bottom: 5px
  .page-info
    flex-shrink: 0
    margin-left: 5px
    font-size: 13px
    opacity: 0.7
  .options-bar
    box-sizing: border-box
    align-items: center
    display: flex
    flex-wrap: wrap
  .control-style
    border-radius: 3px
    border: 1px solid hsl(233, 7%, 22%)
    background-color: hsla(223, 33%, 64%, 0.12)
    outline: none
    margin-top: 0px
    margin-left: 0px
    margin-right: 10px
    font-size: 13px
    color: inherit
  .group
    display: flex
    padding: 0px
    cursor: default
    height: 28px
    line-height: 28px
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
    height: 24px
    font-size: 12px
    box-sizing: border-box
    padding: 0px 8px
    border-radius: 10px
    margin-right: 5px
    transition: all 120ms cubic-bezier(0.4, 0.0, 0.2, 1)
    &:hover
      border-color: hsla(0, 0%, 50%, 0.5)
    &:focus
      border-color: hsla(220, 100%, 50%, 1)
      box-shadow: 0px 0px 0px 3px hsla(220, 100%, 50%, 0.5)
    &.enabled
      background-color: hsl(220, 14%, 28%)
      border-color: hsl(220, 10%, 50%)
      color: #ffffff
    &.enabled:focus
      background-color: hsla(220, 100%, 50%, 1)
      border-color: hsla(220, 100%, 60%, 1)
  input
    height: 28px
    box-sizing: border-box
    padding: 0px 6px
    transition: all 120ms cubic-bezier(0.4, 0.0, 0.2, 1)
    &:focus
      border-color: hsla(220, 100%, 50%, 1)
      box-shadow: 0px 0px 0px 3px hsla(220, 100%, 50%, 0.5)
</style>
