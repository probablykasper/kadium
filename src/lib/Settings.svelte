<script lang="ts">
  import Button from './Button.svelte'
  import { router } from 'tinro'
  import { runCmd } from './general'
  import { reloadSettings } from './data'

  export let apiKey: string
  export let maxConcurrentRequests: number

  function setGeneralSettings() {
    runCmd('set_general_settings', {
      apiKey,
      maxConcurrentRequests,
    }).then(() => {
      reloadSettings()
      router.goto('/')
    })
  }
</script>

<div class="page">
  <p>API Key</p>
  <input type="text" bind:value={apiKey} />
  <p>Max Concurrent Requests</p>
  <input type="number" bind:value={maxConcurrentRequests} />
  <div>
    <Button on:click={setGeneralSettings}>Save</Button>
  </div>
</div>

<style lang="sass">
  .page
    padding: 0px 20px
    max-width: 550px
    margin: auto
    display: flex
    flex-direction: column
  p
    margin: 0px 2px
    font-size: 14px
    font-weight: 500
    margin-top: 5px
    margin-bottom: 0.5em
  input
    display: block
    padding: 8px 12px
    margin: 2px
    background-color: hsla(223, 33%, 64%, 0.1)
    border: 1px solid hsla(0, 0%, 50%, 0.2)
    border-radius: 3px
    transition: all 120ms cubic-bezier(0.4, 0.0, 0.2, 1)
    margin-bottom: 15px
    outline: none
    &:hover
      border: 1px solid hsla(0, 0%, 50%, 0.3)
    &:focus
      border-color: hsla(220, 100%, 50%, 1)
      box-shadow: 0px 0px 0px 3px hsla(220, 100%, 50%, 0.5)
</style>
