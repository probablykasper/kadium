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
  export let index: number
  export let visible = false

  let editMode = true

  let fromTime: Date
  let refreshRateMinutes = 60

  function get(channels: Channel[], index: number) {
    fromTime = new Date(channels[index].from_time)
    refreshRateMinutes = channels[index].refresh_rate_ms / 1000 / 60
  }
  $: get(channels, index)

  async function onDelete() {
    channels.splice(index, 1)
    await saveChannels()
    visible = false
  }
  async function onSave() {
    channels[index].from_time = fromTime.getTime()
    channels[index].refresh_rate_ms = refreshRateMinutes * 60 * 1000
    await saveChannels()
    visible = false
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
  <div class="content">
    <h3>Edit Channel</h3>
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
      {#if editMode}
        <Button danger on:click={onDelete}>Delete</Button>
      {/if}
      <div class="max-spacer" />
      <Button secondary on:click={() => (visible = false)}>Cancel</Button>
      <div class="spacer" />
      <Button on:click={onSave}>Save</Button>
    </div>
  </div>
</Modal>

<style lang="sass">
  .content
    max-width: 400px
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
