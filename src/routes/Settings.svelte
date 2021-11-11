<script lang="ts">
  import Button from '../lib/Button.svelte'
  import { router } from 'tinro'
  import { runCmd } from '../lib/general'
  import { loadSettings } from '../lib/data'

  export let apiKey: string
  export let maxConcurrentRequests: number
  export let checkInBackground: boolean

  async function setGeneralSettings() {
    await runCmd('set_general_settings', {
      apiKey,
      maxConcurrentRequests,
      checkInBackground,
    })
    await loadSettings()
    router.goto('/', true)
  }
  function toggleCheckInBg() {
    checkInBackground = !checkInBackground
  }
</script>

<div class="scroll">
  <form class="page" on:submit|preventDefault={setGeneralSettings}>
    <p>API Key</p>
    <input class="textbox" type="text" bind:value={apiKey} />
    <!-- <p>Max Concurrent Requests</p> -->
    <!-- <input class="textbox" type="number" bind:value={maxConcurrentRequests} /> -->
    <div class="checkbox-row" class:checked={checkInBackground} on:click={toggleCheckInBg}>
      <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24">
        <path
          d="M24,3.382c0,-1.866 -1.516,-3.382 -3.382,-3.382l-17.236,0c-1.866,0 -3.382,1.516 -3.382,3.382l0,17.236c0,1.866
          1.516,3.382 3.382,3.382l17.236,-0c1.866,-0 3.382,-1.516 3.382,-3.382l0,-17.236Zm-2.5,0l-0,17.236c-0,0.487
          -0.395,0.882 -0.882,0.882l-17.236,-0c-0.487,-0 -0.882,-0.395 -0.882,-0.882l0,-17.236c0,-0.487
          0.395,-0.882 0.882,-0.882l17.236,0c0.487,0 0.882,0.395 0.882,0.882Z" />
        <path
          class="checkmark"
          d="M9.348,14.652l8.839,-8.839l1.768,1.768l-10.607,10.606l-5.303,-5.303l1.768,-1.768l3.535,3.536Z" />
      </svg>
      <p>Check in the background</p>
    </div>
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
  input.textbox
    display: block
    font-size: 12px
    height: 31px
    padding: 0px 12px
    box-sizing: border-box
    background-color: hsla(223, 33%, 64%, 0.1)
    border: 1px solid hsla(0, 0%, 50%, 0.2)
    border-radius: 3px
    transition: all 120ms cubic-bezier(0.4, 0.0, 0.2, 1)
    margin: 0px
    margin-bottom: 15px
    outline: none
    &:hover
      border-color: hsla(0, 0%, 50%, 0.3)
    &:focus
      border-color: hsla(220, 100%, 50%, 1)
      box-shadow: 0px 0px 0px 3px hsla(220, 100%, 50%, 0.5)
  .checkbox-row
    display: flex
    align-items: center
    margin-bottom: 15px
    &.checked svg
      fill: hsl(0, 0%, 100%)
      .checkmark
        opacity: 1
        transform: scale(1)
    svg
      fill: hsl(220, 8%, 50%)
      width: 16px
      height: 16px
      margin-right: 7px
      transition: all 80ms cubic-bezier(0.4, 0.0, 0.2, 1)
      .checkmark
        transform-origin: 20% 80%
        transform: scale(0.8)
        opacity: 0
        transition: all 80ms cubic-bezier(0.4, 0.0, 0.2, 1)
    p
      margin: 0px
</style>
