<script lang="ts">
  import ChannelsPage from './routes/Channels.svelte'
  import SettingsPage from './routes/Settings.svelte'
  import { checkShortcut } from './lib/general'
  import { loadSettings, settings, useSampleSettings } from './lib/data'
  import { Route, active, router } from 'tinro'
  import VideosPage from './routes/Videos.svelte'

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

  loadSettings()

  function keydown(e: KeyboardEvent) {
    if (
      checkShortcut(e, 'Backspace') &&
      !(e.target instanceof HTMLInputElement) &&
      !(e.target instanceof HTMLTextAreaElement)
    ) {
      let el = e.target as { isContentEditable?: () => boolean }
      if (el.isContentEditable && el.isContentEditable()) return
      e.preventDefault()
    }
  }
</script>

<svelte:window on:keydown={keydown} />

{#if $settings === null}
  Error loading.

  <button on:click={useSampleSettings}>Check out sample data?</button>
{:else}
  <nav>
    <a on:mousedown={go} use:active data-exact href="/"><button>Videos</button></a>
    <a on:mousedown={go} use:active href="/channels"><button>Channels</button></a>
    <a on:mousedown={go} use:active href="/settings"><button>Settings</button></a>
  </nav>
  <div class="page">
    <Route path="/"><VideosPage /></Route>
    <Route path="/channels"><ChannelsPage channels={$settings.channels} /></Route>
    <Route path="/settings">
      <SettingsPage
        apiKey={$settings.api_key}
        maxConcurrentRequests={$settings.max_concurrent_requests} />
    </Route>
  </div>
{/if}

<style lang="sass">
  $nav-height: 56px
  :global(html)
    overflow: hidden
    height: 100%
    font-family: -apple-system, BlinkMacSystemFont, Segoe UI, Helvetica, Arial, sans-serif, Apple Color Emoji, Segoe UI Emoji
    background-color: #0F1115
    color: #f2f2f2
    color-scheme: dark
    user-select: none
    -webkit-user-select: none
  :global(body)
    height: 100%
    margin: 0px
  nav
    cursor: default
    display: flex
    align-items: center
    box-sizing: border-box
    padding: 0px 20px
    height: $nav-height
  .page
    overflow: auto
    height: calc(100% - $nav-height)
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
