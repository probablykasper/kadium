<script lang="ts">
  import { Channel, loadSettings } from '../lib/data'
  import Modal from '../lib/Modal.svelte'
  import { DateInput } from 'date-picker-svelte'
  import Button from '../lib/Button.svelte'
  import { runCmd } from '../lib/general'

  async function saveChannels() {
    await runCmd('set_channels', { channels })
    await loadSettings()
  }

  export let channels: Channel[]
  export let visible = false

  let url = ''
  let fromTime: Date
  let refreshRateMinutes = 60

  export let editIndex: null | number
  function get(channels: Channel[], index: number) {
    url = ''
    fromTime = new Date(channels[index].from_time)
    refreshRateMinutes = channels[index].refresh_rate_ms / 1000 / 60
  }
  $: if (visible && editIndex === null) {
    console.log('editIndex === null')
    url = ''
    fromTime = new Date()
    refreshRateMinutes = 60
  }
  $: if (visible && editIndex !== null) {
    get(channels, editIndex)
  }

  async function submit() {
    if (editIndex === null) {
      await runCmd('add_channel', {
        options: {
          url,
          from_time: fromTime.getTime(),
          refresh_rate_ms: refreshRateMinutes * 60 * 1000,
          tags: [],
        },
      })
      await loadSettings()
      visible = false
    } else {
      channels[editIndex].from_time = fromTime.getTime()
      channels[editIndex].refresh_rate_ms = refreshRateMinutes * 60 * 1000
      await saveChannels()
      visible = false
    }
  }
  async function onDelete() {
    if (editIndex !== null) {
      channels.splice(editIndex, 1)
      editIndex = null
      await saveChannels()
      visible = false
    }
  }

  let datePopupVisible = false
  $: if (datePopupVisible) {
    setTimeout(() => {
      datePopupVisible = false
    }, 10)
  }

  function keydown(e: KeyboardEvent) {
    if (e.key === 'Escape' && e.target) {
      visible = false
      e.preventDefault()
    }
  }
</script>

<Modal bind:visible on:keydown={keydown}>
  <form class="content" on:submit|preventDefault={submit}>
    {#if editIndex === null}
      <h3>Add Channel</h3>
    {:else}
      <h3>Edit Channel</h3>
    {/if}

    {#if editIndex === null}
      <p>Channel or Video URL</p>
      <input
        type="text"
        placeholder="https://www.youtube.com/channel/UCeTncCK57upn3lPn6PX18Ng"
        bind:value={url} />
    {/if}

    <p>Refresh rate (minutes)</p>
    <p class="sub"
      >Channels with identical refresh rates are grouped together in batches, so it's recommended to
      use only a few different refresh rates</p>
    <input type="number" bind:value={refreshRateMinutes} />

    <p>Check for videos after</p>
    <div class="date-picker">
      <DateInput bind:value={fromTime} bind:visible={datePopupVisible} />
    </div>

    <div class="buttons">
      {#if editIndex !== null}
        <Button danger on:click={onDelete}>Delete</Button>
      {/if}
      <div class="max-spacer" />
      <Button secondary on:click={() => (visible = false)}>Cancel</Button>
      <div class="spacer" />
      <Button type="submit">{editIndex === null ? 'Add' : 'Save'}</Button>
    </div>
  </form>
</Modal>

<style lang="sass">
  .content
    max-width: 440px
  .date-picker
    --date-picker-background: hsla(223, 33%, 64%, 0.1)
    --date-picker-foreground: #f7f7f7
  :global(.date-picker .picker.visible)
    display: none
  p
    font-size: 14px
    font-weight: 500
    margin-top: 5px
    margin-bottom: 7px
    cursor: default
  p.sub
    font-weight: 400
    font-size: 12px
    opacity: 0.6
    margin-top: 5px
    margin-bottom: 7px
  input, :global(.date-picker input)
    display: block
    font-size: 12px
    height: 31px
    padding: 0px 12px
    box-sizing: border-box
    margin: 0px
    background-color: hsla(223, 33%, 64%, 0.1)
    border: 1px solid hsla(0, 0%, 50%, 0.2)
    border-radius: 3px
    transition: all 120ms cubic-bezier(0.4, 0.0, 0.2, 1)
    margin-bottom: 15px
    outline: none
    width: 100%
    &:hover
      border: 1px solid hsla(0, 0%, 50%, 0.3)
    &:focus
      border-color: hsla(220, 100%, 50%, 1)
      box-shadow: 0px 0px 0px 3px hsla(220, 100%, 50%, 0.5)
  .buttons
    margin-top: 20px
    display: flex
    justify-content: flex-end
  .max-spacer
    margin-right: auto
  .spacer
    width: 8px
</style>
