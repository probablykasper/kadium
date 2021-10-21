<script lang="ts">
  import Page from './lib/Page.svelte'
  import { runCmd } from './lib/general'
  import type { Settings } from './lib/data'

  let settings: Promise<Settings> = runCmd('get_settings')
  function sample() {
    settings = new Promise((resolve, reject) => {
      resolve({
        api_key: 'example key',
        from_email: 'who@example.com',
        unread_errors: false,
        max_concurrent_requests: 5,
        groups: [
          {
            email: '1@example.com',
            minutes_between_refreshes: 60,
            channels: [
              {
                from_time: 1611870142000,
                icon: 'https://yt3.ggpht.com/ytc/AAUvwni4bZoon2txFxQCiRVUoabFsxFhth0z5W89mymg=s240-c-k-c0x00ffffff-no-rj',
                id: 'UCp4csaOD64mSzPxbfuzJcuA',
                name: 'Chuckle Sandwich',
                uploads_playlist_id: 'UUp4csaOD64mSzPxbfuzJcuA',
              },
              {
                from_time: 1597330800000,
                icon: 'https://yt3.ggpht.com/a/AATXAJzWhYxcPZ9eSKkC6euMnB_x84TCayZB0EUEIECrxQ=s240-c-k-c0xffffffff-no-rj-mo',
                id: 'UC9RM-iSvTu1uPJb8X5yp3EQ',
                name: 'Wendover Productions',
                uploads_playlist_id: 'UU9RM-iSvTu1uPJb8X5yp3EQ',
              },
            ],
          },
        ],
      })
    })
  }
</script>

{#await settings then settings}
  <Page {settings} />
{:catch e}
  Error loading.

  <button on:click={sample}>Check out sample data?</button>
{/await}
