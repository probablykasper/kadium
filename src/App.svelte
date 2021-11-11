<script lang="ts">
  import { event } from '@tauri-apps/api'
  import ChannelsPage from './routes/Channels.svelte'
  import { checkShortcut, checkModifiers } from './lib/general'
  import SettingsModal from './modals/Settings.svelte'
  import {
    loadSettings,
    settings,
    enableSampleData,
    viewOptions,
    tags,
    settingsOpen,
  } from './lib/data'
  import { Route, router } from 'tinro'
  import VideosPage from './routes/Videos.svelte'
  import VideosBar from './routes/_VideoBar.svelte'
  import Nav from './lib/Nav.svelte'
  import { onDestroy } from 'svelte'
  import GetStarted from './modals/GetStarted.svelte'

  let error = false
  loadSettings().catch(() => {
    error = true
  })

  const numShortcutDigits = ['1', '2', '3', '4', '5', '6', '7', '8', '9']
  function keydown(e: KeyboardEvent) {
    if (
      checkShortcut(e, 'Backspace') &&
      !(e.target instanceof HTMLInputElement) &&
      !(e.target instanceof HTMLTextAreaElement)
    ) {
      let el = e.target as { isContentEditable?: () => boolean }
      if (el.isContentEditable && el.isContentEditable()) return
      e.preventDefault()
    } else if (checkModifiers(e, { cmdOrCtrl: true }) && numShortcutDigits.includes(e.key)) {
      const num = Number(e.key)
      if ($router.path === '/' && $tags[num - 1] === $viewOptions.tag) {
        e.preventDefault()
        $viewOptions.tag = null
      } else if (num <= $tags.length) {
        router.goto('/', true)
        e.preventDefault()
        $viewOptions.tag = $tags[num - 1]
      }
    }
  }

  const unlistenFuture = event.listen('menu', ({ payload }) => {
    if (payload === 'Videos') {
      router.goto('/', true)
    } else if (payload === 'Channels') {
      router.goto('/channels', true)
    } else if (payload === 'Preferences...' || payload === 'Options...') {
      $settingsOpen = true
    } else if (payload === 'Add Channel...') {
      router.goto('/channels#add', true)
    } else if (payload === 'Show New') {
      router.goto('/', true)
      $viewOptions.show_all = false
      $viewOptions.show_archived = false
    } else if (payload === 'Show Archived') {
      router.goto('/', true)
      $viewOptions.show_all = false
      $viewOptions.show_archived = true
    } else if (payload === 'Show All') {
      router.goto('/', true)
      $viewOptions.show_all = true
      $viewOptions.show_archived = false
    }
  })
  onDestroy(async () => {
    const unlisten = await unlistenFuture
    unlisten()
  })
</script>

<svelte:window on:keydown={keydown} />
{#if $settings !== null}
  <Nav />
  <Route path="/">
    <VideosBar />
    <VideosPage />
  </Route>
  <Route path="/channels">
    <ChannelsPage channels={$settings.channels} />
  </Route>

  <SettingsModal
    apiKey={$settings.api_key}
    maxConcurrentRequests={$settings.max_concurrent_requests}
    checkInBackground={$settings.check_in_background}
    bind:visible={$settingsOpen} />

  {#if $settings && $settings.channels.length === 0}
    <GetStarted visible={true} />
  {/if}
{:else if error}
  Error loading.

  <button on:click={enableSampleData}>Check out sample data?</button>
{/if}

<style lang="sass">
  :root
    --options-bar-height: 42px
    --page-padding: 16px
  :global(html), :global(body)
    background-color: hsl(220, 17%, 7%)
  :global(html)
    overflow: hidden
    height: 100%
    font-family: -apple-system, BlinkMacSystemFont, Segoe UI, Helvetica, Arial, sans-serif, Apple Color Emoji, Segoe UI Emoji
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
    margin-right: 1px
</style>
