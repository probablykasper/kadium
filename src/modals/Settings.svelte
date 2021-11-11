<script lang="ts">
  import Button from '../lib/Button.svelte'
  import { runCmd } from '../lib/general'
  import { loadSettings } from '../lib/data'
  import Modal from '../lib/Modal.svelte'
  import Link from '../lib/Link.svelte'

  export let apiKey: string
  export let maxConcurrentRequests: number
  export let checkInBackground: boolean

  export let visible = false

  async function setGeneralSettings() {
    await runCmd('set_general_settings', {
      apiKey,
      maxConcurrentRequests,
      checkInBackground,
    })
    await loadSettings()
    visible = false
  }
  function toggleCheckInBg() {
    checkInBackground = !checkInBackground
  }

  let keyGuideVisible = false
  $: console.log('keyGuideVisible', keyGuideVisible)
  $: console.log('visible', visible)
</script>

<Modal bind:visible>
  <form class="page" on:submit|preventDefault={setGeneralSettings}>
    <h3>Settings</h3>
    <p>API Key</p>
    <p class="sub">
      Kadium has a default API key, but it's vulnerable to abuse and may run out of quota.
      <Link on:click={() => (keyGuideVisible = true)}>
        <div class="edit">Get your own key</div>
      </Link>
    </p>
    <input class="textbox" type="text" bind:value={apiKey} placeholder="AIzaSyNq5Y9knL..." />
    <!-- <p>Max Concurrent Requests</p> -->
    <!-- <input class="textbox" type="number" bind:value={maxConcurrentRequests} /> -->
    <div class="checkbox-row" class:checked={checkInBackground} on:click={toggleCheckInBg}>
      <button>
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
      </button>
      <p>Check in the background</p>
    </div>
    <div class="buttons">
      <Button secondary on:click={() => (visible = false)}>Cancel</Button>
      <div class="spacer" />
      <Button type="submit">Save</Button>
    </div>
  </form>
</Modal>

<Modal bind:visible={keyGuideVisible}>
  <form class="guide-page" on:submit|preventDefault={() => (keyGuideVisible = false)}>
    <h3>Create an API key</h3>
    <ol>
      <li>
        Go to the <Link href="https://console.cloud.google.com/apis/dashboard"
          >Google APIs & Services
        </Link> website.
      </li>
      <li>
        <Link href="https://console.cloud.google.com/projectcreate">Create a new project</Link>. Set
        the project name to <b>my-kadium</b> and click <b>CREATE</b>.</li>
      <li
        >Make sure you have the <b>my-kadium</b> project selected in the top-left project dropdown menu.</li>
      <li
        >Go to the <Link
          href="https://console.cloud.google.com/apis/library/youtube.googleapis.com?project=mykadium"
          >YouTube Data API v3</Link>
        page and click <b>ENABLE</b>.</li>
      <li
        >On the <Link href="https://console.cloud.google.com/apis/credentials?project=mykadium"
          >Credentials</Link>
        page, click <b>CREATE CREDENTIALS</b>, then <b>API key</b>.</li>
      <li>You should see your API key!</li>
      <li>Optionally restrict the API key to the YouTube API:</li>
      <ul>
        <li
          >Open your API key from the
          <Link href="https://console.cloud.google.com/apis/credentials?project=mykadium"
            >Credentials</Link> page.</li>
        <li>Under <b>API restrictions</b>, select <b>Restrict key</b>.</li>
        <li>In the dropdown, select <b>YouTube Data API v3</b>.</li>
        <li>Press <b>SAVE</b>.</li>
      </ul>
    </ol>
    <Button type="submit">Ok</Button>
  </form>
</Modal>

<style lang="sass">
  .guide-page
    width: 600px
    li
      font-size: 15px
      margin-bottom: 3px
  .page
    width: 400px
  h3
    margin-top: 0px
  p
    font-size: 14px
    font-weight: 500
    margin-top: 5px
    margin-bottom: 7px
    cursor: default
  p.sub
    font-weight: 400
    font-size: 12px
    color: hsl(220, 5%, 65%)
    margin-top: 5px
    margin-bottom: 7px
  input.textbox
    display: block
    font-size: 12px
    width: 100%
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
    button
      margin: 0px
      padding: 0px
      background: transparent
      border: none
      margin-right: 7px
      // outline: none
    &.checked svg
      fill: hsl(0, 0%, 100%)
      .checkmark
        opacity: 1
        transform: scale(1)
    svg
      fill: hsl(220, 8%, 50%)
      width: 16px
      height: 16px
      display: block
      transition: all 80ms cubic-bezier(0.4, 0.0, 0.2, 1)
      .checkmark
        transform-origin: 20% 80%
        transform: scale(0.8)
        opacity: 0
        transition: all 80ms cubic-bezier(0.4, 0.0, 0.2, 1)
    p
      margin: 0px
  .buttons
    margin-top: 20px
    display: flex
    justify-content: flex-end
  .spacer
    width: 8px
</style>
