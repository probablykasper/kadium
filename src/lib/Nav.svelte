<script lang="ts">
  import { active, router } from 'tinro'
  import { settingsOpen } from './data'
  import { runCmd } from './general'
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
    await runCmd('check_now')
  }
  function openSettings() {
    $settingsOpen = true
  }
</script>

<nav>
  <a on:mousedown={go} use:active data-exact href="/"><button>Videos</button></a>
  <a on:mousedown={go} use:active href="/channels"><button>Channels</button></a>
  <div class="spacer" />
  <button class="control-style" on:click={checkNow}>Check Now</button>
  <button class="control-style" on:click={openSettings}>Settings</button>
</nav>

<style lang="sass">
  nav
    padding: 0px 20px
    display: flex
    align-items: center
    height: 54px
    flex-shrink: 0
    background-color: hsl(220, 17%, 9%)
    border-bottom: 1px solid hsla(0, 0%, 50%, 0.07)
  .spacer
    margin-left: auto
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
  button.control-style
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
</style>
