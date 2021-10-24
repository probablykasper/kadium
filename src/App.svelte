<script lang="ts">
  import Channels from './lib/Channels.svelte'
  import { checkShortcut, runCmd } from './lib/general'
  import type { Settings } from './lib/data'
  import { Route, active, router } from 'tinro'

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

  let settings: Promise<Settings> = runCmd('get_settings')
  function sample() {
    settings = new Promise((resolve) => {
      resolve({
        api_key: 'example key',
        max_concurrent_requests: 5,
        channels: (() => {
          let channels = []
          for (let i = 0; i < 100; i++) {
            channels.push({
              from_time: 1611870142000,
              icon: 'https://yt3.ggpht.com/ytc/AAUvwni4bZoon2txFxQCiRVUoabFsxFhth0z5W89mymg=s240-c-k-c0x00ffffff-no-rj',
              id: 'UCp4csaOD64mSzPxbfuzJcuA',
              name: 'Chuckle Sandwich ' + i,
              uploads_playlist_id: 'UUp4csaOD64mSzPxbfuzJcuA',
              minutes_between_refreshes: 60,
              tags: ['Chungus'],
            })
          }
          return channels
        })(),
      })
    })
  }
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

{#await settings then settings}
  <nav>
    <a on:mousedown={go} use:active data-exact href="/">Videos</a>
    <a on:mousedown={go} use:active href="/channels">Channels</a>
    <a on:mousedown={go} use:active href="/settings">Settings</a>
  </nav>
  <div class="page">
    <Route path="/">Videos page</Route>
    <Route path="/channels"><Channels channels={settings.channels} /></Route>
    <Route path="/settings">Settings</Route>
  </div>
{:catch e}
  Error loading.

  <button on:click={sample}>Check out sample data?</button>
{/await}

<style lang="sass">
  :global(html)
    overflow: hidden
    height: 100%
    font-family: -apple-system, BlinkMacSystemFont, Segoe UI, Helvetica, Arial, sans-serif, Apple Color Emoji, Segoe UI Emoji
    background-color: hsl(220, 25%, 8%)
    color: #f2f2f2
    color-scheme: dark
    user-select: none
    -webkit-user-select: none
  :global(body)
    height: 100%
    margin: 0px
    --nav-height: 54px
  nav
    cursor: default
    display: flex
    align-items: center
    background-color: hsla(0, 0%, 100%, 0.05)
    border-bottom: 1px solid hsla(0, 0%, 100%, 0.05)
    box-sizing: border-box
    padding: 0px 20px
    height: var(--nav-height)
  .page
    overflow: auto
    height: calc(100% - var(--nav-height))
  a
    display: inline-block
    font-size: 16px
    margin-right: 15px
    text-decoration: none
    padding: 12px 0px
    color: hsl(210, 100%, 55%)
    &:hover
      color: hsl(210, 100%, 45%)
    &:global(.active)
      color: hsl(216, 30%, 93%)
</style>
