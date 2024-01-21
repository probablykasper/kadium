<script lang="ts" context="module">
	let getStartedWasShown = false
</script>

<script lang="ts">
	import { event } from '@tauri-apps/api'
	import { checkShortcut, checkModifiers } from '$lib/general'
	import SettingsModal from '$lib/modals/Settings.svelte'
	import {
		loadSettings,
		settings,
		enableSampleData,
		viewOptions,
		tags,
		settingsOpen,
	} from '$lib/data'
	import Nav from '$lib/Nav.svelte'
	import { onDestroy } from 'svelte'
	import GetStarted from '$lib/modals/GetStarted.svelte'
	import { page } from '$app/stores'
	import { goto } from '$app/navigation'

	let error = false
	loadSettings().catch(() => {
		error = true
	})

	const numShortcutDigits = ['1', '2', '3', '4', '5', '6', '7', '8', '9']
	function keydown(e: KeyboardEvent) {
		if (
			checkShortcut(e, 'Backspace') &&
			!(e.target instanceof HTMLInputElement) &&
			!(e.target instanceof HTMLTextAreaElement)
		) {
			let el = e.target as { isContentEditable?: () => boolean }
			if (el.isContentEditable && el.isContentEditable()) return
			e.preventDefault()
		} else if (checkModifiers(e, { cmdOrCtrl: true }) && numShortcutDigits.includes(e.key)) {
			const num = Number(e.key)
			if ($page.route.id === '/' && $tags[num - 1] === $viewOptions.tag) {
				e.preventDefault()
				$viewOptions.tag = null
			} else if (num <= $tags.length) {
				goto('/', { replaceState: true })
				e.preventDefault()
				$viewOptions.tag = $tags[num - 1]
			}
		}
	}

	const unlistenFuture = event.listen('tauri://menu', ({ payload }) => {
		if (payload === 'Videos') {
			goto('/', { replaceState: true })
		} else if (payload === 'Channels') {
			goto('/channels', { replaceState: true })
		} else if (payload === 'History') {
			goto('/history', { replaceState: true })
		} else if (payload === 'Preferences...' || payload === 'Options...') {
			$settingsOpen = true
		} else if (payload === 'Add Channel...') {
			goto('/channels?add', { replaceState: true })
		} else if (payload === 'Show New') {
			goto('/', { replaceState: true })
			$viewOptions.show_all = false
			$viewOptions.show_archived = false
		} else if (payload === 'Show Archived') {
			goto('/', { replaceState: true })
			$viewOptions.show_all = false
			$viewOptions.show_archived = true
		} else if (payload === 'Show All') {
			goto('/', { replaceState: true })
			$viewOptions.show_all = true
			$viewOptions.show_archived = false
		} else if (payload === 'Get Started') {
			showGetStarted = true
		}
	})
	onDestroy(async () => {
		const unlisten = await unlistenFuture
		unlisten()
	})
	let showGetStarted = false
	$: if (showGetStarted) {
		getStartedWasShown = true
	}

	$: if (!getStartedWasShown && $settings?.channels.length === 0 && $settings.api_key === '') {
		showGetStarted = true
	}
</script>

<svelte:window on:keydown={keydown} />
{#if $settings !== null}
	<Nav />
	<slot />

	<SettingsModal
		apiKey={$settings.api_key}
		maxConcurrentRequests={$settings.max_concurrent_requests}
		checkInBackground={$settings.check_in_background}
		bind:visible={$settingsOpen}
	/>

	<GetStarted bind:visible={showGetStarted} />
{:else if error}
	Error loading.

	<button on:click={enableSampleData}>Check out sample data?</button>
{/if}

<style lang="sass">
	:root
		--options-bar-height: 42px
		--page-padding: 16px
		--ease-out-cubic: cubic-bezier(0.33, 1, 0.68, 1)
	:global(html), :global(body)
		background-color: hsl(220, 17%, 7%)
	:global(html)
		overflow: hidden
		height: 100%
		font-family: -apple-system, BlinkMacSystemFont, Segoe UI, Helvetica, Arial, sans-serif, Apple Color Emoji, Segoe UI Emoji
		color: #f2f2f2
		color-scheme: dark
		user-select: none
		pointer: default
	:global(p), :global(h1), :global(h2), :global(h3)
		user-select: text
	:global(span)
		pointer: inherit
	:global(body)
		height: 100%
		margin: 0px
		color-scheme: dark
		display: flex
		flex-direction: column
		margin-right: 1px
</style>
