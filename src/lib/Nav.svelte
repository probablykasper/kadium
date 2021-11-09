<script lang="ts">
  import { active, router } from 'tinro'

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
</script>

<nav>
  <a on:mousedown={go} use:active data-exact href="/"><button>Videos</button></a>
  <a on:mousedown={go} use:active href="/channels"><button>Channels</button></a>
  <a on:mousedown={go} use:active href="/settings"><button>Settings</button></a>
</nav>

<style lang="sass">
  nav
    padding: 0px 20px
    display: flex
    align-items: center
    height: var(--nav-height)
    flex-shrink: 0
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
</style>
