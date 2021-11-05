<script lang="ts">
  import { ViewOptions, viewOptions } from '../lib/data'
  import { listen } from '@tauri-apps/api/event'

  import { onDestroy } from 'svelte'
  import { runCmd } from '../lib/general'

  type Video = {
    id: string
    title: string
    description: string
    publishTimeMs: number
    durationMs: number
    thumbnailStandard: boolean
    thumbnailMaxres: boolean
    channelId: string
    channelName: string
    unread: boolean
  }
  let videos: Video[] = []

  async function getVideos(options: ViewOptions) {
    videos = await runCmd('get_videos', { viewOptions: options })
  }
  $: getVideos($viewOptions)

  let updateCounter = 0
  async function getCount() {
    const newCount = await runCmd('video_update_counter')
    console.log('newCount', newCount)
    if (newCount > updateCounter) {
      getVideos($viewOptions)
    }
  }
  let updateInterval = setInterval(getCount, 2000)
  const focusUnlistener = listen('tauri://focus', () => {
    clearInterval(updateInterval)
    getCount()
    updateInterval = setInterval(getCount, 2000)
  })
  const blurUnlistener = listen('tauri://blur', () => {
    clearInterval(updateInterval)
  })
  onDestroy(async () => {
    clearInterval(updateInterval)
    ;(await focusUnlistener)()
    ;(await blurUnlistener)()
  })

  const months = [
    'Jan',
    'Feb',
    'Mar',
    'Apr',
    'May',
    'Jun',
    'Jul',
    'Aug',
    'Sep',
    'Oct',
    'Nov',
    'Dec',
  ]
  function formatDate(timestamp: number) {
    let ts = new Date(timestamp)
    return ts.getDay() + ' ' + months[ts.getMonth()] + ' ' + ts.getFullYear()
  }
</script>

<main class="selectable">
  {#each videos as video}
    <div class="box">
      <!-- <img src="https://i.ytimg.com/vi/{video.id}/hqdefault.jpg" alt="" /> -->
      <!-- <img src="https://i.ytimg.com/vi/{video.id}/sddefault.jpg" alt="" /> -->
      <a target="_blank" href="https://youtube.com/watch?v={video.id}">
        <button>
          <img src="https://i.ytimg.com/vi/{video.id}/maxresdefault.jpg" alt="" />
          <p class="title selectable">{video.title}</p>
        </button>
      </a>
      <p class="channel">
        <a target="_blank" href="https://www.youtube.com/channel/{video.channelId}">
          <button class="selectable">{video.channelName}</button>
        </a>
      </p>
      <p class="channel selectable">{formatDate(video.publishTimeMs)}</p>
    </div>
  {/each}
</main>

<style lang="sass">
  .selectable
    user-select: text
    -webkit-user-select: text
  main
    width: 100%
    box-sizing: border-box
    display: grid
    grid-template-columns: repeat(auto-fit, minmax(180px, 1fr))
    @media screen and (max-width: 430px)
      grid-template-columns: 1fr
      grid-gap: 15px
      .box
        margin: 0px auto
    grid-gap: 15px
    padding: 20px
  .box
    max-width: 280px
    user-select: none
    -webkit-user-select: none
  img
    width: 100%
  p
    margin: 0px
  a
    text-decoration: none
    color: inherit
    &:focus
      border-color: hsl(210, 100%, 55%)
    button
      background-color: transparent
      border: none
      margin: 0px
      padding: 0px
      text-align: left
      cursor: pointer
  p.title
    font-size: 13px
    font-weight: 500
    color: #ffffff
  p.channel
    font-size: 11.5px
    opacity: 0.8
    margin-top: 2px
</style>
