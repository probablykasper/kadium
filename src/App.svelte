<script lang="ts">
  import Page from './lib/Page.svelte'
  import { runCmd } from './lib/general'
  import type { Settings } from './lib/data'
  import Modal from './lib/Modal.svelte'

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
            email: '1@example.com',
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

<Modal />

{#await settings then settings}
  <Page {settings} />
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
  :global(body)
    overflow: auto
    height: 100%
    margin: 0px
</style>
