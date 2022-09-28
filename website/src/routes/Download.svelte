<script lang="ts">
  import { fly, scale } from 'svelte/transition'
  import ButtonPopup from './ButtonPopup.svelte'
  import Bowser from 'bowser'
  import { onMount } from 'svelte'
  import type { RestEndpointMethodTypes } from '@octokit/plugin-rest-endpoint-methods'

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

  let downloadError = ''
  let loading = false
  type LatestRelease = RestEndpointMethodTypes['repos']['getLatestRelease']['response']['data']

  function getAsset(assets: LatestRelease['assets'], version: Version) {
    for (const asset of assets) {
      if (asset.name.toLowerCase().endsWith(version.ending)) {
        return asset
      }
    }
    return null
  }
  async function innerDownload(version: Version) {
    downloadError = ''
    try {
      const response = await fetch(
        `https://api.github.com/repos/probablykasper/kadium/releases/latest`
      )
      console.log(response)

      const json: LatestRelease = await response.json()
      console.log(json)

      if (response.status !== 200) {
        downloadError = 'Could not download'
        console.error('Error response from GitHub', json)
        return
      }

      const asset = getAsset(json.assets, version)
      if (asset === null) {
        downloadError = 'No file found'
        console.error('No file found')
        return
      }
      console.log(asset)
      location.href = asset.browser_download_url
    } catch (error) {
      downloadError = 'Network error'
      console.error(error)
    }
  }
  async function download(version: Version) {
    loading = true
    await innerDownload(version)
    loading = false
  }
</script>

<div class="mb-2 h-6">
  {#if downloadError}
    <div
      class="error block text-center text-sm font-semibold text-red-500"
      transition:fly={{ y: 3, duration: 200 }}
    >
      {downloadError}
    </div>
  {/if}
</div>

<ButtonPopup let:toggle let:isOpen let:close>
  <div
    class="relative mx-auto flex h-9 cursor-default items-center border border-white border-opacity-10 bg-white bg-opacity-5 text-base font-medium transition-all duration-300 ease-in-out hover:border-opacity-20"
    class:rounded-2xl={!isOpen}
    class:rounded-lg={isOpen}
    class:pointer-events-none={loading}
    class:opacity-75={loading}
  >
    <button
      class="group relative flex h-full items-center pr-4 pl-5 text-white text-opacity-70 outline-none transition-all duration-300 hover:text-opacity-100"
      on:click={() => download(suggestedVersion)}
      class:opacity-50={loading}
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
        on:click={() => {
          close()
          download(version)
        }}
      >
        {version.os} <span class="opacity-70">{version.arch}</span>
      </button>
    {/each}
  </div>
</ButtonPopup>

<style lang="sass">
  .gradient
    will-change: contents
  .gradient-3
    background: linear-gradient(130deg,#09cff6 10%,#3159f6 90%)
  .error
    text-shadow: 0px 0px 7px hsl(0deg 100% 50% / 60%)
</style>
