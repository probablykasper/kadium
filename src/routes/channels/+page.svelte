<script lang="ts">
	import Link from '$lib/Link.svelte'
	import { loadSettings, settings } from '$lib/data'
	import Tags from '$lib/Tags.svelte'
	import type { Channel } from '../../../bindings'
	import ChannelModal from '$lib/modals/Channel.svelte'
	import commands from '$lib/commands'
	import { page } from '$app/stores'
	import { goto } from '$app/navigation'
	import { menu_actions } from '../menu'

	$: channels = $settings?.channels ?? []

	$: visibleIndexes = getVisibleIndexes(channels, filter)
	function getVisibleIndexes(channels: Channel[], filter: string) {
		let indexes = []
		let i = 0
		for (const channel of channels) {
			if (filter === '' || channel.name.includes(filter)) {
				indexes.push(i)
			}
			i++
		}
		return indexes
	}

	async function saveChannels() {
		await commands.setChannels(channels)
		await loadSettings()
	}

	let editIndex: null | number = null
	let editVisible = false
	function openEditModal(index: number) {
		editIndex = index
		editVisible = true
	}
	function openAddModal() {
		editIndex = null
		editVisible = true
	}
	$: if ($page.url.searchParams.has('add')) {
		openAddModal()
	}
	$: if (!editVisible) {
		goto('/channels', { replaceState: true })
	}

	let filter = ''
	let filterInput: HTMLInputElement | undefined
	menu_actions.Find = () => {
		filterInput?.focus()
	}

	let channels_scroll_el: HTMLDivElement
</script>

<ChannelModal
	{channels}
	bind:editIndex
	bind:visible={editVisible}
	on_add={() => {
		channels_scroll_el.scrollTo({ behavior: 'smooth', top: channels_scroll_el.scrollHeight })
	}}
/>

<main>
	<header>
		<button class="control-style" on:click={openAddModal}>Add</button>
		<input
			bind:this={filterInput}
			class="bar-item control-style"
			type="text"
			placeholder="Channel Filter"
			bind:value={filter}
		/>
		<div class="page-info">
			{visibleIndexes.length} of {channels.length}
		</div>
	</header>
	<div bind:this={channels_scroll_el} class="channels">
		{#each channels as channel, i}
			{@const lowerName = channel.name.toLowerCase()}
			<div
				class="channel selectable"
				class:hidden={filter !== '' && !lowerName.includes(filter.toLowerCase())}
			>
				<img src={channel.icon} alt="" />
				<div class="details">
					<a
						href="https://youtube.com/channel/{channel.id}"
						target="_blank"
						rel="noreferrer"
						class="title">{channel.name}</a
					>
					<div class="content">
						<span
							>Check for videos after {new Date(Number(channel.from_time)).toLocaleString()}</span
						>
						<span>Refresh rate: {Number(channel.refresh_rate_ms) / (1000 * 60)} minutes</span>
					</div>
					<Tags bind:value={channel.tags} onUpdate={saveChannels} />
				</div>
				<Link on:click={() => openEditModal(i)}>
					<div class="edit">Edit</div>
				</Link>
			</div>
		{/each}
	</div>
</main>

<style lang="sass">
	main
		display: flex
		flex-direction: column
		height: 100%
		overflow-y: auto
	header
		display: flex
		align-items: center
		width: 100%
		box-sizing: border-box
		padding: 0px 20px
		justify-content: space-between
		flex-shrink: 0
		border-bottom: 1px solid hsla(0, 0%, 50%, 0.12)
	.page-info
		flex-shrink: 0
		margin-left: auto
		font-size: 13px
		opacity: 0.7
	.control-style
		border-radius: 3px
		border: 1px solid hsl(233, 7%, 22%)
		background-color: hsla(223, 33%, 64%, 0.12)
		outline: none
		margin: 10px 0px
		margin-right: 8px
		font-size: 13px
		color: inherit
	button.control-style
		height: 28px
		padding: 0px 11px
		box-sizing: border-box
		border: 1px solid hsla(0, 0%, 50%, 0.2)
		background-color: hsl(225, 14%, 20%)
		transition: all 120ms var(--ease-out-cubic)
		&:hover
			border-color: hsla(0, 0%, 50%, 0.5)
		&:focus
			border-color: hsla(220, 100%, 50%, 1)
			box-shadow: 0px 0px 0px 3px hsla(220, 100%, 50%, 0.5)
	.selectable
		user-select: text
	.channels
		padding: var(--page-padding)
		gap: var(--page-padding)
		display: grid
		grid-template-columns: 1fr
		@media screen and (min-width: 600px)
			grid-template-columns: repeat(auto-fill, minmax(550px, 1fr))
	.channel
		display: flex
		align-items: center
		border-radius: 7px
		transition: border 140ms var(--ease-out-cubic)
		padding: 15px 5px
		border: 1px solid transparent
		@media screen and (min-width: 600px)
			padding: 15px
			background-color: hsla(223, 33%, 64%, 0.05)
			box-shadow: 0px 4px 8px 0px hsla(0, 0%, 0%, 0.1)
			border: 1px solid hsla(0, 0%, 50%, 0.04)
			&:hover
				border: 1px solid hsla(0, 0%, 50%, 0.2)
		&.hidden
			display: none
	img
		width: 70px
		padding-right: 20px
	.details
		padding-right: 15px
		flex-grow: 1
		a.title
			color: inherit
			font-weight: bold
			font-size: 16px
			text-decoration: none
		.content
			font-size: 13px
			span
				display: block
				color: hsla(231, 20%, 100%, 0.5)
	.edit
		padding: 10px 0px
		font-size: 13px
	input
		height: 28px
		box-sizing: border-box
		padding: 0px 6px
		transition: all 120ms var(--ease-out-cubic)
		&:focus
			border-color: hsla(220, 100%, 50%, 1)
			box-shadow: 0px 0px 0px 3px hsla(220, 100%, 50%, 0.5)
</style>
