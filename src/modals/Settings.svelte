<script lang="ts">
  import Button from '../lib/Button.svelte'
  import { loadSettings } from '../lib/data'
  import Modal from '../lib/Modal.svelte'
  import Link from '../lib/Link.svelte'
  import Switch from '../lib/Switch.svelte'
  import commands from 'src/lib/commands'

  export let apiKey: string
  export let maxConcurrentRequests: number
  export let checkInBackground: boolean

  export let visible = false

  async function setGeneralSettings() {
    await commands.setGeneralSettings(apiKey, maxConcurrentRequests, checkInBackground)
    await loadSettings()
    visible = false
  }

  function keydown(e: KeyboardEvent) {
    if (e.key === 'Escape' && e.target) {
      visible = false
      e.preventDefault()
    }
  }

  let keyGuideVisible = false
  function keyGuideKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      keyGuideVisible = false
      e.preventDefault()
    }
  }
</script>

<Modal bind:visible on:keydown={keydown}>
  <form class="page" on:submit|preventDefault={setGeneralSettings}>
    <h3>Settings</h3>
    <p>API Key</p>
    <p class="sub">
      Kadium has a default API key, but it's vulnerable to abuse and could run out of quota.
      <Link on:click={() => (keyGuideVisible = true)}>
        <div>Get your own key</div>
      </Link>
    </p>
    <input class="textbox" type="text" bind:value={apiKey} placeholder="AIzaSyNq5Y9knL..." />
    <div class="toggle-row">
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <p on:click={() => (checkInBackground = !checkInBackground)}>
        Check for new videos automatically
      </p>
      <Switch bind:checked={checkInBackground} />
    </div>
    <div class="buttons">
      <Button secondary on:click={() => (visible = false)}>Cancel</Button>
      <div class="spacer" />
      <Button type="submit">Save</Button>
    </div>
  </form>
</Modal>

<Modal bind:visible={keyGuideVisible} on:keydown={keyGuideKeydown}>
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
        the project name to <b>my-kadium</b> and click <b>CREATE</b>.
      </li>
      <li>
        Make sure you have the <b>my-kadium</b> project selected in the top-left project dropdown menu.
      </li>
      <li>
        Go to the <Link
          href="https://console.cloud.google.com/apis/library/youtube.googleapis.com?project=mykadium"
          >YouTube Data API v3</Link
        >
        page and click <b>ENABLE</b>.
      </li>
      <li>
        On the <Link href="https://console.cloud.google.com/apis/credentials?project=mykadium"
          >Credentials</Link
        >
        page, click <b>CREATE CREDENTIALS</b>, then <b>API key</b>.
      </li>
      <li>You should see your API key!</li>
      <li>Optionally restrict the API key to the YouTube API:</li>
      <ul>
        <li>
          Open your API key from the
          <Link href="https://console.cloud.google.com/apis/credentials?project=mykadium"
            >Credentials</Link
          > page.
        </li>
        <li>Under <b>API restrictions</b>, select <b>Restrict key</b>.</li>
        <li>In the dropdown, select <b>YouTube Data API v3</b>.</li>
        <li>Press <b>SAVE</b>.</li>
      </ul>
    </ol>
    <div class="right">
      <Button type="submit">Oh okay</Button>
    </div>
  </form>
</Modal>

<style lang="sass">
  .guide-page
    width: 520px
    li
      font-size: 15px
      margin-bottom: 3px
    .right
      display: flex
      justify-content: flex-end
  .page
    width: 400px
  h3
    margin-top: 0px
  p
    font-size: 14px
    font-weight: 500
    margin-top: 5px
    margin-bottom: 7px
  .sub
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
  .toggle-row
    display: flex
    align-items: center
    justify-content: space-between
    margin-bottom: 15px
    p
      user-select: none
      cursor: default
  .buttons
    margin-top: 20px
    display: flex
    justify-content: flex-end
  .spacer
    width: 8px
</style>
