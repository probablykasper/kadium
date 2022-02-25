<script lang="ts">
  import { ViewOptions, viewOptions, Video } from '../lib/data'
  import { listen } from '@tauri-apps/api/event'
  import { onDestroy, tick } from 'svelte'
  import { runCmd } from '../lib/general'
  import VideoBar from './_VideoBar.svelte'

  let videos: Video[] = []
  let allLoaded = false
  $: getVideos($viewOptions)
  let loading = false
  async function getVideos(options: ViewOptions) {
    loading = true

    videos = await runCmd('get_videos', { options })
    allLoaded = videos.length < $viewOptions.limit

    loading = false

    await tick()
    await autoloadHandler()
  }
  async function getMoreVideos() {
    loading = true

    const newVideos = await runCmd('get_videos', {
      options: $viewOptions,
      after: {
        publishTimeMs: videos[videos.length - 1].publishTimeMs,
        id: videos[videos.length - 1].id,
      },
    })
    videos = videos.concat(newVideos)
    allLoaded = videos.length < $viewOptions.limit

    loading = false
    await tick()
    await autoloadHandler()
  }
  async function autoloadHandler() {
    if (!allLoaded && isScrolledToBottom() && !loading) {
      await getMoreVideos()
    }
  }

  let updateCounter = 0
  async function getUpdateCount() {
    const newCount = await runCmd('video_update_counter')
    if (newCount > updateCounter) {
      updateCounter = newCount
      getVideos($viewOptions)
    }
  }
  let updateInterval = setInterval(getUpdateCount, 2000)
  const focusUnlistener = listen('tauri://focus', () => {
    clearInterval(updateInterval)
    getUpdateCount()
    updateInterval = setInterval(getUpdateCount, 2000)
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
    return ts.getDate() + ' ' + months[ts.getMonth()] + ' ' + ts.getFullYear()
  }
  async function archiveToggle(id: string, isArchived: boolean) {
    if (isArchived) {
      await runCmd('unarchive', { id })
    } else {
      await runCmd('archive', { id })
    }
    getVideos($viewOptions)
  }

  let main: HTMLElement | null = null
  const loadThreshold = 200
  function isScrolledToBottom() {
    if (main) {
      const offset = main.scrollHeight - (main.clientHeight + main.scrollTop)
      if (offset <= loadThreshold) {
        return true
      }
    }
    return false
  }
</script>

<svelte:window on:resize={autoloadHandler} />

<VideoBar loadedVideosCount={videos.length} {allLoaded} />

<main bind:this={main} class="selectable" on:scroll={autoloadHandler}>
  <div class="grid">
    {#each videos as video}
      <div class="box">
        <a class="img-box" target="_blank" href="https://youtube.com/watch?v={video.id}">
          <div class="img-box">
            <div class="img-parent">
              <img src="https://i.ytimg.com/vi/{video.id}/hqdefault.jpg" alt="" />
            </div>
          </div>
        </a>
        <button
          class="archive"
          on:click={() => archiveToggle(video.id, video.archived)}
          title="Archive"
          tabindex="-1"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            class:archived={video.archived}
          >
            <path
              class="frame"
              d="M24,3.382c0,-1.866 -1.516,-3.382 -3.382,-3.382l-17.236,0c-1.866,0 -3.382,1.516 -3.382,3.382l0,17.236c0,1.866
          1.516,3.382 3.382,3.382l17.236,-0c1.866,-0 3.382,-1.516 3.382,-3.382l0,-17.236Zm-2.5,0l-0,17.236c-0,0.487
          -0.395,0.882 -0.882,0.882l-17.236,-0c-0.487,-0 -0.882,-0.395 -0.882,-0.882l0,-17.236c0,-0.487
          0.395,-0.882 0.882,-0.882l17.236,0c0.487,0 0.882,0.395 0.882,0.882Z"
            />
            <path
              class="checkmark"
              d="M9.348,14.652l8.839,-8.839l1.768,1.768l-10.607,10.606l-5.303,-5.303l1.768,-1.768l3.535,3.536Z"
            />
          </svg>
        </button>
        <a class="row" target="_blank" href="https://youtube.com/watch?v={video.id}">
          <button>
            <p class="title selectable">{video.title}</p>
          </button>
        </a>
        <a class="row" target="_blank" href="https://www.youtube.com/channel/{video.channelId}">
          <button class="selectable">
            <p class="sub">
              {video.channelName}
            </p>
          </button>
        </a>
        <p class="row sub selectable">{formatDate(video.publishTimeMs)}</p>
      </div>
    {/each}
  </div>
</main>

<style lang="sass">
  .selectable
    user-select: text
  main
    max-width: 100%
    height: 100%
    overflow-y: auto
  .grid
    flex-grow: 0
    box-sizing: border-box
    display: grid
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr))
    grid-gap: 15px
    padding: var(--page-padding)
    padding-top: 15px
    @media screen and (max-width: 450px)
      grid-template-columns: 1fr
      .box
        margin: 0px auto
  .box
    max-width: 280px
    width: 100%
    user-select: none
    position: relative
  .row
    display: block
    width: 100%
  .img-box
    width: 100%
    padding-top: 56.25%
    position: relative
  .img-parent
    position: absolute
    top: 0px
    left: 0px
    width: 100%
    height: 100%
    overflow: hidden
    display: flex
    align-items: center
  img
    width: 100%
    top:-100%
    bottom:-100%
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
      color: inherit
  p.title
    font-size: 13px
    font-weight: 500
    color: #ffffff
  p.sub
    font-size: 12px
    color: hsl(210, 8%, 80%)
    margin-top: 2px
  .box:hover button.archive
    opacity: 1
  .archive
    position: absolute
    cursor: pointer
    top: 0px
    right: 0px
    border-radius: 10px
    box-shadow: inset 0px 0px 10px 8px rgba(0, 0, 0, 0.15), 0px 0px 12px 12px rgba(0, 0, 0, 0.15)
    background-color: transparent
    margin: 0px
    padding: 0px
    border: none
    transform: translate3d(0, 0, 0) // fix glitch after transform/opacity
    opacity: 0
    transition: opacity 120ms cubic-bezier(0.4, 0.0, 0.2, 1)
    svg
      fill: #ffffff
      width: 16px
      height: 16px
      padding-top: 4px
      padding-right: 4px
      padding-bottom: 2px
      padding-left: 2px
      vertical-align: middle
      filter: drop-shadow( 0px 0px 2px rgba(0, 0, 0, 0.4))
      path.frame
        transform: scale(1)
        transform-origin: center
    svg.archived
      path.checkmark
        transform: scale(1)
        transform-origin: 20% 80%
        transition: all 120ms cubic-bezier(0.4, 0.0, 0.2, 1)
      &:active path.checkmark
        transform: scale(0.8)
        opacity: 0
    svg:not(.archived)
      path.checkmark
        transform: scale(0.8)
        opacity: 0
        transition: all 120ms cubic-bezier(0.4, 0.0, 0.2, 1)
      &:active path.checkmark
        opacity: 1
        transform: scale(1)
</style>
