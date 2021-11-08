<script lang="ts">
  import ChannelsPage from './routes/Channels.svelte'
  import SettingsPage from './routes/Settings.svelte'
  import { checkShortcut } from './lib/general'
  import { loadSettings, settings, enableSampleData } from './lib/data'
  import { Route } from 'tinro'
  import VideosPage from './routes/Videos.svelte'
  import Nav from './lib/Nav.svelte'

  let error = false
  loadSettings().catch(() => {
    error = true
  })

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

{#if $settings !== null}
  <Nav />
  <main>
    <Route path="/"><VideosPage /></Route>
    <Route path="/channels"><ChannelsPage channels={$settings.channels} /></Route>
    <Route path="/settings"
      ><SettingsPage
        apiKey={$settings.api_key}
        maxConcurrentRequests={$settings.max_concurrent_requests} /></Route>
  </main>
{:else if error}
  Error loading.

  <button on:click={enableSampleData}>Check out sample data?</button>}
{/if}

<style lang="sass">
  :root
    --options-bar-height: 42px
    --nav-height: 56px
    --header-height: (var(--options-bar-height) + var(--nav-height))
    --page-padding: 15px
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
    color-scheme: dark
    display: flex
    flex-direction: column
  main
    height: 0px
    flex-grow: 1
    background-color: #0F1115 // so scrollbars are light
    margin-right: 1px
</style>
