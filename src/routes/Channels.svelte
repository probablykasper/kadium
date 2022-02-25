<script lang="ts">
  import Link from '../lib/Link.svelte'
  import { Channel, loadSettings } from '../lib/data'
  import Tags from '../lib/Tags.svelte'
  import { runCmd } from '../lib/general'
  import ChannelModal from '../modals/Channel.svelte'
  import { event } from '@tauri-apps/api'
  import { onDestroy } from 'svelte'
  import { router } from 'tinro'

  export let channels: Channel[]

  $: visibleIndexes = getVisibleIndexes(channels, filter)
  function getVisibleIndexes(channels: Channel[], filter: string) {
    let indexes = []
    let i = 0
    for (const channel of channels) {
      if (filter === '' || channel.name.includes(filter)) {
        indexes.push(i)
      }
      i++
    }
    return indexes
  }

  async function saveChannels() {
    await runCmd('set_channels', { channels })
    await loadSettings()
  }

  let editIndex: null | number = null
  let editVisible = false
  function openEditModal(index: number) {
    editIndex = index
    editVisible = true
  }
  function openAddModal() {
    editIndex = null
    editVisible = true
  }
  $: if ($router.hash === 'add') {
    openAddModal()
  }

  let filter = ''
  let filterInput: HTMLInputElement
  const unlistenFuture = event.listen('menu', ({ payload }) => {
    if (payload === 'Find') {
      filterInput.focus()
    }
  })
  onDestroy(async () => {
    const unlisten = await unlistenFuture
    unlisten()
  })
</script>

<ChannelModal {channels} bind:editIndex bind:visible={editVisible} />

<main>
  <header>
    <button class="control-style" on:click={openAddModal}>Add</button>
    <input
      bind:this={filterInput}
      class="bar-item control-style"
      type="text"
      placeholder="Channel Filter"
      bind:value={filter}
    />
    <div class="page-info">
      {visibleIndexes.length} of {channels.length}
    </div>
  </header>
  <div class="channels">
    {#each channels as channel, i}
      <div class="channel selectable" class:show={filter === '' || channel.name.includes(filter)}>
        <img src={channel.icon} alt="" />
        <div class="details">
          <a href="https://youtube.com/channel/{channel.id}" target="_blank" class="title"
            >{channel.name}</a
          >
          <div class="content">
            <!-- <span>{channel.id}</span> -->
            <span>Check for videos after {new Date(channel.from_time).toLocaleString()}</span>
            <span>Refresh rate: {channel.refresh_rate_ms / 1000 / 60} minutes</span>
          </div>
          <Tags bind:value={channel.tags} on:update={saveChannels} />
        </div>
        <Link on:click={() => openEditModal(i)}>
          <div class="edit">Edit</div>
        </Link>
      </div>
    {/each}
  </div>
</main>

<style lang="sass">
  main
    display: flex
    flex-direction: column
    height: 100%
  header
    display: flex
    align-items: center
    width: 100%
    box-sizing: border-box
    padding: 0px 20px
    justify-content: space-between
    flex-shrink: 0
    border-bottom: 1px solid hsla(0, 0%, 50%, 0.12)
  .page-info
    flex-shrink: 0
    margin-left: auto
    font-size: 13px
    opacity: 0.7
  .control-style
    border-radius: 3px
    border: 1px solid hsl(233, 7%, 22%)
    background-color: hsla(223, 33%, 64%, 0.12)
    outline: none
    margin: 10px 0px
    margin-right: 8px
    font-size: 13px
    color: inherit
  button.control-style
    height: 28px
    padding: 0px 11px
    box-sizing: border-box
    border: 1px solid hsla(0, 0%, 50%, 0.2)
    background-color: hsl(225, 14%, 20%)
    transition: all 120ms cubic-bezier(0.4, 0.0, 0.2, 1)
    &:hover
      border-color: hsla(0, 0%, 50%, 0.5)
    &:focus
      border-color: hsla(220, 100%, 50%, 1)
      box-shadow: 0px 0px 0px 3px hsla(220, 100%, 50%, 0.5)
  .selectable
    user-select: text
  .channels
    flex-wrap: wrap
    padding: 0px var(--page-padding)
    height: 0px
    flex-grow: 1
    overflow-y: auto
  .channel
    flex-grow: 1
    align-items: center
    border-radius: 7px
    transition: border 120ms cubic-bezier(0.4, 0.0, 0.2, 1)
    padding: 15px 5px
    border: 1px solid transparent
    display: none
    &.show
      display: flex
    @media screen and (min-width: 600px)
      padding: 15px
      margin: 15px 0px
      background-color: hsla(223, 33%, 64%, 0.05)
      box-shadow: 0px 4px 8px 0px hsla(0, 0%, 0%, 0.1)
      border: 1px solid hsla(0, 0%, 50%, 0.04)
      &:hover
        border: 1px solid hsla(0, 0%, 50%, 0.2)
  img
    width: 70px
    padding-right: 20px
  .details
    padding-right: 15px
    flex-grow: 1
    a.title
      color: inherit
      font-weight: bold
      font-size: 16px
      text-decoration: none
    .content
      font-size: 13px
      span
        display: block
        color: hsla(231, 20%, 100%, 0.5)
  .edit
    padding: 10px 0px
  input
    height: 28px
    box-sizing: border-box
    padding: 0px 6px
    transition: all 120ms cubic-bezier(0.4, 0.0, 0.2, 1)
    &:focus
      border-color: hsla(220, 100%, 50%, 1)
      box-shadow: 0px 0px 0px 3px hsla(220, 100%, 50%, 0.5)
</style>
