<script lang="ts">
  import Link from '../lib/Link.svelte'
  import { Channel, defaultChannel, loadSettings } from '../lib/data'
  import Tags from '../lib/Tags.svelte'
  import { runCmd } from '../lib/general'
  import ChannelModal from '../modals/Channel.svelte'

  export let channels: Channel[]

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
</script>

{#if editIndex !== null}
  <ChannelModal {channels} index={editIndex} bind:visible={editVisible} />
{/if}

<div class="channels">
  {#each channels as channel, i}
    <div class="channel selectable">
      <img src={channel.icon} alt="" />
      <div class="details">
        <a href="https://youtube.com/channel/{channel.id}" target="_blank" class="title"
          >{channel.name}</a>
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

<style lang="sass">
  .selectable
    user-select: text
    -webkit-user-select: text
  .channels
    flex-wrap: wrap
    padding: 0px var(--page-padding)
    height: 100%
    overflow-y: auto
  .channel
    display: flex
    flex-grow: 1
    align-items: center
    border-radius: 7px
    transition: border 120ms cubic-bezier(0.4, 0.0, 0.2, 1)
    padding: 15px 5px
    border: 1px solid transparent
    @media screen and (min-width: 600px)
      padding: 15px
      margin: 15px 0px
      background-color: hsla(223, 33%, 64%, 0.05)
      box-shadow: 0px 4px 8px 0px hsla(0, 0%, 0%, 0.1)
      border: 1px solid hsla(0, 0%, 50%, 0.04)
      // border: 1px solid transparent
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
</style>
