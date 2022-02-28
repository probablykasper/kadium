<script lang="ts">
  import { ViewOptions, viewOptions, Video } from '../lib/data'
  import { event, shell } from '@tauri-apps/api'
  import { listen } from '@tauri-apps/api/event'
  import { onDestroy, tick } from 'svelte'
  import { checkModifiers, checkShortcut, runCmd } from '../lib/general'
  import VideoBar from './_VideoBar.svelte'

  let videos: Video[] = []
  let allLoaded = false
  $: getVideos($viewOptions)
  let loading = false
  async function getVideos(options: ViewOptions) {
    loading = true

    const newVideos = await runCmd('get_videos', { options })
    allLoaded = videos.length < $viewOptions.limit
    videos = newVideos
    selectedIndex = 0
    selectionVisible = false

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
    allLoaded = newVideos.length < $viewOptions.limit
    videos = videos.concat(newVideos)

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
  async function archive(id: string) {
    await runCmd('archive', { id })
    getVideos($viewOptions)
  }
  async function unarchive(id: string) {
    await runCmd('unarchive', { id })
    getVideos($viewOptions)
  }
  async function archiveToggle(id: string, isArchived: boolean) {
    if (isArchived) unarchive(id)
    else archive(id)
  }

  let scrollDiv: HTMLElement | null = null
  const loadThreshold = 200
  function isScrolledToBottom() {
    if (scrollDiv) {
      const offset = scrollDiv.scrollHeight - (scrollDiv.clientHeight + scrollDiv.scrollTop)
      if (offset <= loadThreshold) {
        return true
      }
    }
    return false
  }

  let grid: HTMLDivElement

  let selectionVisible = false
  let selectedIndex = 0
  function select(index: number) {
    selectedIndex = index
    selectionVisible = true
  }

  function openVideo(index: number) {
    shell.open('https://youtube.com/watch?v=' + videos[index].id)
  }
  function openChannel(index: number) {
    shell.open('https://www.youtube.com/channel/' + videos[index].channelId)
  }
  function getColumnCount() {
    const gridStyle = window.getComputedStyle(grid)
    const gridTemplateCols = gridStyle.getPropertyValue('grid-template-columns')
    return gridTemplateCols.split(' ').length
  }

  function keydown(e: KeyboardEvent) {
    let target = e.target as HTMLElement
    if (target.nodeName === 'INPUT') {
      return
    }

    if (selectionVisible) {
      if (checkShortcut(e, 'ArrowLeft')) {
        if (selectedIndex % getColumnCount() !== 0) {
          selectedIndex--
          selectedIndex = Math.max(0, selectedIndex)
          e.preventDefault()
        }
      } else if (checkShortcut(e, 'ArrowRight')) {
        if ((selectedIndex + 1) % getColumnCount() !== 0) {
          selectedIndex++
          selectedIndex = Math.min(selectedIndex, videos.length - 1)
        }
        e.preventDefault()
      } else if (checkShortcut(e, 'ArrowUp')) {
        const columnCount = getColumnCount()
        if (selectedIndex - columnCount >= 0) {
          selectedIndex -= columnCount
        }
        e.preventDefault()
      } else if (checkShortcut(e, 'ArrowDown')) {
        const columnCount = getColumnCount()
        if (selectedIndex + columnCount <= videos.length - 1) {
          selectedIndex += columnCount
        }
        e.preventDefault()
      } else if (checkShortcut(e, 'Escape')) {
        selectionVisible = false
        e.preventDefault()
      } else if (checkShortcut(e, 'Enter')) {
        openVideo(selectedIndex)
        e.preventDefault()
      } else if (checkShortcut(e, 'Enter', { shift: true })) {
        openChannel(selectedIndex)
        e.preventDefault()
      }
    } else {
      if (
        checkShortcut(e, 'ArrowLeft') ||
        checkShortcut(e, 'ArrowRight') ||
        checkShortcut(e, 'ArrowUp') ||
        checkShortcut(e, 'ArrowDown')
      ) {
        selectionVisible = true
        e.preventDefault()
      }
    }
  }
  function videoClick(e: MouseEvent, index: number) {
    if (checkModifiers(e, {})) {
      selectedIndex = index
    } else if (checkModifiers(e, { cmdOrCtrl: true })) {
      openVideo(index)
    }
  }
  function channelClick(e: MouseEvent, index: number) {
    if (checkModifiers(e, { cmdOrCtrl: true })) {
      openChannel(index)
    }
  }

  const unlistenFuture = event.listen('tauri://menu', async ({ payload }) => {
    if (payload === 'Open Selected Video' && selectionVisible) {
      openVideo(selectedIndex)
    } else if (payload === 'Open Selected Channel' && selectionVisible) {
      openChannel(selectedIndex)
    } else if (payload === 'Archive') {
      archive(videos[selectedIndex].id)
    } else if (payload === 'Unarchive') {
      unarchive(videos[selectedIndex].id)
    }
  })
  onDestroy(async () => {
    const unlisten = await unlistenFuture
    unlisten()
  })

  let boxes = [] as HTMLDivElement[]
  $: scrollToBox(selectedIndex)
  function scrollToBox(index: number) {
    if (scrollDiv && boxes[index]) {
      const el = boxes[index].getBoundingClientRect()
      const parent = scrollDiv.getBoundingClientRect()
      const topOffset = el.top - parent.top
      const bottomOffset = el.bottom - parent.bottom
      if (topOffset < 0) {
        scrollDiv.scrollTop += topOffset - 5
      } else if (bottomOffset > 0) {
        scrollDiv.scrollTop += bottomOffset + 5
      }
    }
  }
</script>

<svelte:window on:resize={autoloadHandler} on:keydown={keydown} />

<VideoBar loadedVideosCount={videos.length} {allLoaded} />

<main tabindex="0" on:scroll={autoloadHandler} bind:this={scrollDiv}>
  <div class="grid" tabindex="-1" bind:this={grid}>
    {#each videos as video, i}
      <div
        class="box"
        class:selected={selectionVisible && i === selectedIndex}
        bind:this={boxes[i]}
        on:mousedown={() => select(i)}
        on:dblclick={() => shell.open('https://youtube.com/watch?v=' + videos[i].id)}
        on:click={(e) => videoClick(e, i)}
      >
        <a
          class="img-box"
          href="https://youtube.com/watch?v={video.id}"
          draggable="false"
          on:click|preventDefault
        >
          <div class="img-parent">
            <img src="https://i.ytimg.com/vi/{video.id}/hqdefault.jpg" alt="" />
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
        <a class="row" href="https://youtube.com/watch?v={video.id}" on:click|preventDefault>
          <p class="title selectable">{video.title}</p>
        </a>
        <p class="channel sub">
          <a
            class="row"
            href="https://www.youtube.com/channel/{video.channelId}"
            on:click|preventDefault|stopPropagation={(e) => channelClick(e, i)}
            on:dblclick|stopPropagation={() => openChannel(i)}
          >
            {video.channelName}
          </a>
        </p>
        <p class="row sub selectable">{formatDate(video.publishTimeMs)}</p>
      </div>
    {/each}
  </div>
</main>

<style lang="sass">
  $ease-md: cubic-bezier(0.4, 0.0, 0.2, 1)
  .selectable
    user-select: text
  main
    outline: none
    overflow-y: auto
    max-width: 100%
  .grid
    height: 100%
    max-height: 100%
    flex-grow: 0
    box-sizing: border-box
    display: grid
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr))
    grid-gap: 15px
    padding: var(--page-padding)
    padding-top: 15px
    outline: none
    @media screen and (max-width: 450px)
      grid-template-columns: 1fr
      .box
        margin: 0px auto
  .box
    max-width: 280px
    width: 100%
    user-select: none
    position: relative
    padding: 3px
    border: 1px solid transparent
    border-radius: 3px
  .selected
    background-color: hsla(210, 100%, 95%, 0.07)
    border-color: hsla(210, 100%, 90%, 0.25)
  .row
    display: block
    width: 100%
  .img-box
    display: block
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
    cursor: default
    &:focus
      border-color: hsl(210, 100%, 55%)
  p.title
    font-size: 13px
    font-weight: 500
    color: #ffffff
    opacity: 1
    transition: 100ms opacity $ease-md
    margin-top: 1px
  .channel
    transition: 80ms opacity $ease-md
    a
      display: inline
    a:hover
      color: hsl(210, 8%, 90%)
  p.sub
    font-size: 12px
    cursor: default
    color: hsl(210, 8%, 80%)
    margin-top: 2px
  .box:hover button.archive
    opacity: 1
  button.archive
    position: absolute
    cursor: pointer
    top: 0px
    right: 0px
    border-radius: 10px
    box-shadow: inset 0px 0px 10px 8px rgba(0, 0, 0, 0.15), 0px 0px 12px 12px rgba(0, 0, 0, 0.15)
    background-color: transparent
    margin-right: 2px
    margin-top: 2px
    padding: 0px
    border: none
    transform: translate3d(0, 0, 0) // fix glitch after transform/opacity
    opacity: 0
    transition: opacity 120ms $ease-md
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
        transition: all 120ms $ease-md
      &:active path.checkmark
        transform: scale(0.8)
        opacity: 0
    svg:not(.archived)
      path.checkmark
        transform: scale(0.8)
        opacity: 0
        transition: all 120ms $ease-md
      &:active path.checkmark
        opacity: 1
        transform: scale(1)
</style>
