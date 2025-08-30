<script lang="ts" context="module">
	import { writable } from 'svelte/store'

	let get_started_was_shown = false
	export let show_get_started = writable(false)
</script>

<script lang="ts">
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
	import GetStarted from '$lib/modals/GetStarted.svelte'
	import { page } from '$app/stores'
	import { goto } from '$app/navigation'
	import { create_menu } from './menu'

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

	create_menu()

	// $: if ($show_get_started) {
	// 	get_started_was_shown = true
	// }

	// $: if (!get_started_was_shown && $settings?.channels.length === 0 && $settings.api_key === '') {
	// 	$show_get_started = true
	// }
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

	<GetStarted bind:visible={$show_get_started} />
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
