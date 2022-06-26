<script lang="ts">
  import { scale } from 'svelte/transition'
  import ButtonPopup from './_ButtonPopup.svelte'
  import Revealed from './_Revealed.svelte'
  import Bowser from 'bowser'
  import { onMount } from 'svelte'
  // @ts-ignore (linter error)
  import { Octokit } from '/skypack/octokit@1.x'

  type Version = {
    os: string
    arch: string
    ending: string
  }
  const macOS: Version = {
    os: 'macOS',
    arch: 'x64',
    ending: '.dmg',
  }
  const windows: Version = {
    os: 'Windows',
    arch: 'x64',
    ending: '.msi',
  }
  const linuxDeb: Version = {
    os: 'Linux .deb',
    arch: 'x64',
    ending: '.deb',
  }
  const linuxAppImage: Version = {
    os: 'Linux AppImage',
    arch: 'x64',
    ending: '.appimage',
  }
  const versionList: Version[] = [macOS, windows, linuxDeb, linuxAppImage]
  let suggestedVersion = windows
  onMount(() => {
    const browser = Bowser.getParser(window.navigator.userAgent)
    const osName = browser.getOSName()
    if (osName === 'macOS' || osName === 'iOS') {
      suggestedVersion = macOS
    } else if (osName === 'Windows') {
      suggestedVersion = windows
    } else if (osName === 'Linux' || osName === 'Chrome OS') {
      suggestedVersion = linuxDeb
    }
  })

  async function download(version: Version) {
    const octokit = new Octokit()
    try {
      const { data } = await octokit.rest.repos.getLatestRelease({
        owner: 'probablykasper',
        repo: 'kadium',
      })
      for (const asset of data.assets) {
        if (asset.name.toLowerCase().endsWith(version.ending)) {
          console.log(asset)
          window.open(asset.browser_download_url, '_self')
          return
        }
      }
    } catch (e) {
      console.error(e)
    }
  }
</script>

<svelte:head>
  <title>Kadium</title>
  <meta name="description" content="An app for staying ontop of YouTube channels' uploads" />
</svelte:head>

<Revealed
  class="mt-20 mb-4 text-center transition-all ease-out"
  options={{ opacity: 0, scale: 0.9, duration: 1000 }}
>
  <h1 class="inline-block text-7xl font-extrabold">Kadium</h1>
</Revealed>
<Revealed
  class="mb-12 text-center transition-all ease-out"
  options={{ opacity: 0, y: -10, duration: 750, delay: 500 }}
>
  <p class="text-xl text-blue-100 opacity-60 transition-all duration-500 ease-out">
    An app for staying ontop of YouTube channels' uploads
  </p>
</Revealed>

<Revealed
  class="mb-24 flex justify-center transition-all ease-out"
  options={{ opacity: 0, scale: 0.9, duration: 750, delay: 700 }}
>
  <ButtonPopup let:toggle let:isOpen>
    <div
      class="relative mx-auto flex h-9 cursor-default items-center border border-white border-opacity-10 bg-white bg-opacity-5 text-base font-medium transition-all duration-300 ease-in-out hover:border-opacity-20"
      class:rounded-2xl={!isOpen}
      class:rounded-lg={isOpen}
    >
      <button
        class="group relative flex h-full items-center pr-4 pl-5 text-white text-opacity-70 outline-none transition-all duration-300 hover:text-opacity-100"
        on:click={() => download(suggestedVersion)}
      >
        <div
          class="opacity-0 transition-all duration-700 ease-out group-hover:opacity-40 group-focus:opacity-40"
        >
          <div
            class="gradient gradient-3 scale-80 absolute inset-0 -z-10 transition-all duration-700 ease-out group-hover:scale-100 group-hover:blur-md group-focus:scale-100 group-focus:blur-md"
          />
        </div>
        Download for {suggestedVersion.os}
      </button>
      <div class="h-5 border-l border-white border-opacity-30" />
      <button
        class="group relative h-full pl-4 pr-5 text-white text-opacity-70 outline-none transition-all duration-300 hover:text-opacity-100"
        on:click={toggle}
      >
        <div
          class="opacity-0 transition-all duration-700 ease-out group-hover:opacity-40 group-focus:opacity-40"
        >
          <div
            class="gradient gradient-3 scale-80 absolute inset-0 -z-10 transition-all duration-700 ease-out group-hover:scale-100 group-hover:blur-md group-focus:scale-100 group-focus:blur-md"
          />
        </div>
        <svg
          fill="currentColor"
          xmlns="http://www.w3.org/2000/svg"
          width="12"
          height="12"
          viewBox="0 0 24 24"
          ><path d="M0 7.33l2.829-2.83 9.175 9.339 9.167-9.339 2.829 2.83-11.996 12.17z" /></svg
        >
      </button>
    </div>
    <div
      slot="popup"
      class="mt-0.5 w-full divide-y divide-white divide-opacity-10 rounded-xl border border-white border-opacity-20 bg-white bg-opacity-5 text-white backdrop-blur-md will-change-contents"
      transition:scale={{ start: 0.9, opacity: 0, duration: 300 }}
    >
      {#each versionList as version}
        <button
          class="h-9 w-full bg-white bg-opacity-0 px-5 text-left outline-none hover:bg-opacity-5 focus:bg-opacity-5"
          on:click={() => download(version)}
        >
          {version.os} <span class="opacity-70">{version.arch}</span>
        </button>
      {/each}
    </div>
  </ButtonPopup>
</Revealed>

<Revealed
  class="relative mx-auto mb-24 max-w-5xl px-6 sm:px-8"
  options={{ opacity: 0, y: 10, duration: 1000, delay: 600 }}
>
  <div class="gradient gradient-1 show-gradient absolute inset-0 -z-10 blur-[160px]" />
  <div class="relative">
    <div class="ripple absolute inset-0 -z-10 rounded-[5px]" />
    <img src="/screenshot-1.webp" alt="" />
  </div>
</Revealed>
<Revealed
  class="relative mx-auto mb-24 max-w-5xl px-6 sm:px-8"
  options={{ opacity: 0, y: 10, duration: 1000 }}
>
  <div class="gradient gradient-2 absolute inset-0 -z-10 blur-[160px]" />
  <div class="relative">
    <div class="ripple absolute inset-0 -z-10 rounded-[5px]" />
    <img src="/screenshot-2.webp" alt="" />
  </div>
</Revealed>

<style lang="sass">
  h1
    background: linear-gradient(130deg,#09cff6 10%,#3159f6 90%)
    background-clip: text
    text-fill-color: transparent
    -webkit-text-fill-color: transparent
  .gradient
    will-change: contents
    &.gradient-1
      background: conic-gradient(from 230.29deg at 50% 50%,#2400ff 0deg,#199af5 90deg,#691eff 180deg,#00d4ff 270deg,#1fff8b 360deg)
    &.gradient-2
      background: conic-gradient(from 230.29deg at 50% 50%,#311bf3 0deg,#f5198f 90deg,#691eff 180deg,#2400ff 270deg,#199af5 360deg)
    &.gradient-3
      background: linear-gradient(130deg,#09cff6 10%,#3159f6 90%)
  .show-gradient
    animation: show-gradient 3s ease-in-out forwards
    opacity: 0
    animation-delay: 500ms
  @keyframes show-gradient
    0%
      opacity: 0
    50%
      opacity: 0.9
    100%
      opacity: 0.7
  .ripple
    // animation: ping 3s cubic-bezier(0, 0, 0.2, 1) infinite
    box-shadow: 0px 0px 0px 1px hsla(0, 0%, 100%, 0.1), 0px 0px 0px 4px hsla(0, 0%, 100%, 0.15)
  // @keyframes ping
  //   0%
  //     box-shadow: 0px 0px 0px 1px hsla(0, 0%, 100%, 0.1), 0px 0px 0px 0px hsla(0, 0%, 100%, 0.15)
  //   100%
  //     box-shadow: 0px 0px 0px 1px hsla(0, 100%, 100%, 0.1), 0px 0px 0px 9px hsla(0, 0%, 100%, 0)
</style>
