<script lang="ts">
  import { event } from '@tauri-apps/api'
  import { onDestroy } from 'svelte'
  import { fade } from 'svelte/transition'
  import { active, router } from 'tinro'
  import commands from './commands'
  import { settingsOpen } from './data'
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
  async function checkNow() {
    checking = 0
    await commands.checkNow()
  }
  function openSettings() {
    $settingsOpen = true
  }
  function keydown(e: KeyboardEvent) {
    if (checkShortcut(e, 'Escape')) {
      if (e.target instanceof HTMLElement) {
        e.target?.blur()
        e.preventDefault()
      }
    }
  }
  let checking = 0
  const unlistenChecking = event.listen('checking', () => {
    checking++
  })
  const unlistenDoneChecking = event.listen('doneChecking', () => {
    checking--
  })
  onDestroy(async () => {
    ;(await unlistenChecking)()
    ;(await unlistenDoneChecking)()
  })
</script>

<nav on:keydown={keydown}>
  <a on:mousedown={go} use:active data-exact href="/">Videos</a>
  <a on:mousedown={go} use:active href="/channels">Channels</a>
  <button class="control-style ml-auto" class:checking={checking > 0} on:click={checkNow}>
    Check Now
    {#if checking > 0}
      <div class="loader-container" transition:fade={{ duration: 400 }}>
        <span class="loader" />
      </div>
    {/if}
  </button>
  <button class="control-style" on:click={openSettings}>Settings</button>
</nav>

<style lang="sass">
  nav
    padding: 0px 20px
    display: flex
    align-items: center
    height: 54px
    flex-shrink: 0
    background-color: hsl(220, 17%, 10%)
    border-bottom: 1px solid hsla(0, 0%, 50%, 0.08)
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
  .control-style
    border-radius: 3px
    border: 1px solid hsl(233, 7%, 22%)
    background-color: hsla(223, 33%, 64%, 0.12)
    outline: none
    margin-top: 0px
    margin-left: 10px
    margin-right: 0px
    font-size: 13px
    color: inherit
    &.ml-auto
      margin-left: auto
  button.control-style
    position: relative
    height: 28px
    padding: 0px 11px
    box-sizing: border-box
    border: 1px solid hsla(0, 0%, 50%, 0.2)
    background-color: hsl(225, 14%, 20%)
    transition: all 120ms cubic-bezier(0.4, 0.0, 0.2, 1)
    &:hover
      border-color: hsla(0, 0%, 50%, 0.5)
    &:focus
      border-color: hsla(220, 100%, 50%, 1)
      box-shadow: 0px 0px 0px 3px hsla(220, 100%, 50%, 0.5)
    &.checking
      color: transparent
      transition: color 400ms cubic-bezier(0.4, 0.0, 0.2, 1)
  .loader-container
    position: absolute
    top: 0px
    left: 0px
    right: 0px
    bottom: 0px
    display: flex
    align-items: center
    justify-content: center
  @keyframes rotation
    0%
      transform: rotate(0deg)
    100%
      transform: rotate(360deg)
  .loader
    width: 16px
    height: 16px
    border: 1.5px solid #d9d9d9
    border-bottom-color: transparent
    border-radius: 50%
    display: inline-block
    box-sizing: border-box
    animation: rotation 1s linear infinite
</style>
