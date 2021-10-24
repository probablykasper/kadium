<script lang="ts">
  import Page from './lib/Page.svelte'
  import { runCmd } from './lib/general'
  import type { Settings } from './lib/data'
  import { Route, active } from 'tinro'

  let settings: Promise<Settings> = runCmd('get_settings')
  function sample() {
    settings = new Promise((resolve) => {
      resolve({
        api_key: 'example key',
        from_email: 'who@example.com',
        unread_errors: false,
        max_concurrent_requests: 5,
        groups: [
          {
            name: 'Group',
            minutes_between_refreshes: 60,
            channels: (() => {
              let channels = []
              for (let i = 0; i < 100; i++) {
                channels.push({
                  from_time: 1611870142000,
                  icon: 'https://yt3.ggpht.com/ytc/AAUvwni4bZoon2txFxQCiRVUoabFsxFhth0z5W89mymg=s240-c-k-c0x00ffffff-no-rj',
                  id: 'UCp4csaOD64mSzPxbfuzJcuA',
                  name: 'Chuckle Sandwich ' + i,
                  uploads_playlist_id: 'UUp4csaOD64mSzPxbfuzJcuA',
                })
              }
              return channels
            })(),
          },
        ],
      })
    })
  }
</script>

{#await settings then settings}
  <nav>
    <a use:active data-exact href="/">Videos</a>
    <a use:active data-exact href="/channels">Channels</a>
    <a use:active data-exact href="/settings">Settings</a>
    {#each settings.groups as group, i}
      <a use:active data-exact href="/group/{i}-{group.name}">{group.name}</a>
    {/each}
  </nav>
  <div class="page">
    <Route path="/">Videos page</Route>
    <Route path="/channels"><Page {settings} /></Route>
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
    display: flex
    flex-direction: column
  nav
    cursor: default
    background-color: hsla(0, 0%, 100%, 0.05)
    border-bottom: 1px solid hsla(0, 0%, 100%, 0.05)
    padding: 15px
  .page
    padding: 20px
    padding-top: 0px
    overflow: auto
  a
    font-size: 16px
    margin-right: 15px
    text-decoration: none
    padding: 12px 0px
    vertical-align: middle
    color: hsl(210, 100%, 55%)
    background-color: transparent
    border: none
    &:hover
      color: hsl(210, 100%, 45%)
    &:global(.active)
      color: hsl(216, 30%, 93%)
</style>
