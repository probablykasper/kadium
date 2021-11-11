<script lang="ts">
  import Button from '../lib/Button.svelte'
  import { router } from 'tinro'
  import { runCmd } from '../lib/general'
  import { loadSettings } from '../lib/data'

  export let apiKey: string
  export let maxConcurrentRequests: number

  async function setGeneralSettings() {
    await runCmd('set_general_settings', {
      apiKey,
      maxConcurrentRequests,
    })
    await loadSettings()
    router.goto('/', true)
  }
</script>

<div class="scroll">
  <form class="page" on:submit|preventDefault={setGeneralSettings}>
    <p>API Key</p>
    <input type="text" bind:value={apiKey} />
    <p>Max Concurrent Requests</p>
    <input type="number" bind:value={maxConcurrentRequests} />
    <div>
      <Button type="submit">Save</Button>
    </div>
  </form>
</div>

<style lang="sass">
  .scroll
    height: 100%
    overflow-y: auto
  .page
    padding: 10px 20px
    max-width: 550px
    margin: auto
    display: flex
    flex-direction: column
  p
    margin: 0px
    font-size: 14px
    font-weight: 500
    margin-top: 5px
    margin-bottom: 7px
    cursor: default
  input
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
    &:hover
      border-color: hsla(0, 0%, 50%, 0.3)
    &:focus
      border-color: hsla(220, 100%, 50%, 1)
      box-shadow: 0px 0px 0px 3px hsla(220, 100%, 50%, 0.5)
</style>
