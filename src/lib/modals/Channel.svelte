<script lang="ts">
	import { loadSettings } from '$lib/data'
	import Modal from 'modal-svelte'
	import { DateInput } from 'date-picker-svelte'
	import Button from '$lib/Button.svelte'
	import type { Channel } from '../../../bindings'
	import commands from '$lib/commands'

	async function saveChannels() {
		await commands.setChannels(channels)
		await loadSettings()
	}

	export let channels: Channel[]
	export let visible = false
	export let on_add: () => void

	let url = ''
	let fromTime: Date | null
	let refreshRateMinutes = 60

	export let editIndex: null | number
	function get(channels: Channel[], index: number) {
		url = ''
		fromTime = new Date(Number(channels[index].from_time))
		refreshRateMinutes = Number(channels[index].refresh_rate_ms) / 1000 / 60
	}
	$: if (visible && editIndex === null) {
		url = ''
		fromTime = new Date()
		refreshRateMinutes = 60
	}
	$: if (visible && editIndex !== null) {
		get(channels, editIndex)
	}

	async function submit() {
		if (fromTime === null) {
			return
		}
		if (editIndex === null) {
			await commands.addChannel({
				url,
				from_time: Math.round(fromTime.getTime()),
				refresh_rate_ms: Math.round(refreshRateMinutes * 60 * 1000),
				tags: [],
			})
			await loadSettings()
			visible = false
			on_add()
		} else {
			channels[editIndex].from_time = Math.round(fromTime.getTime())
			channels[editIndex].refresh_rate_ms = Math.round(refreshRateMinutes * 60 * 1000)
			await saveChannels()
			visible = false
		}
	}
	async function onDelete() {
		if (editIndex !== null) {
			channels.splice(editIndex, 1)
			editIndex = null
			await saveChannels()
			visible = false
		}
	}

	let datePopupVisible = false
	$: if (datePopupVisible) {
		setTimeout(() => {
			datePopupVisible = false
		}, 0)
	}
</script>

{#if visible}
	<Modal
		onCancel={() => {
			visible = false
		}}
		noCloseIcon
		form={submit}
	>
		<div class="content">
			{#if editIndex === null}
				<h3>Add Channel</h3>
			{:else}
				<h3>Edit Channel</h3>
			{/if}

			{#if editIndex === null}
				<p>Channel or Video URL</p>
				<!-- svelte-ignore a11y-autofocus -->
				<input
					type="text"
					placeholder="https://www.youtube.com/channel/UCeTncCK57upn3lPn6PX18Ng"
					bind:value={url}
					autofocus
				/>
			{/if}

			<p>Refresh rate (minutes)</p>
			<p class="sub">
				Channels with identical refresh rates are grouped together in batches, so it's recommended
				to use only a few different refresh rates
			</p>
			<input type="number" bind:value={refreshRateMinutes} />

			<p>Check for videos after</p>
			<div
				role="none"
				class="date-picker"
				on:keydown|capture={(e) => {
					if (e.key === 'Enter') e.stopPropagation()
				}}
			>
				<DateInput bind:value={fromTime} bind:visible={datePopupVisible} />
			</div>

			<div class="buttons">
				{#if editIndex !== null}
					<Button danger on:click={onDelete}>Delete</Button>
				{/if}
				<div class="max-spacer" />
				<Button secondary on:click={() => (visible = false)}>Cancel</Button>
				<div class="spacer" />
				<Button type="submit">{editIndex === null ? 'Add' : 'Save'}</Button>
			</div>
		</div>
	</Modal>
{/if}

<style lang="sass">
	h3
		margin-top: 0px
	.content
		max-width: 440px
	.date-picker
		--date-picker-background: hsla(223, 33%, 64%, 0.1)
		--date-picker-foreground: #f7f7f7
	:global(.date-picker .picker.visible)
		display: none
	p
		font-size: 14px
		font-weight: 500
		margin-top: 5px
		margin-bottom: 7px
	p.sub
		font-weight: 400
		font-size: 12px
		color: hsla(0, 0%, 100%, 0.6)
		margin-top: 5px
		margin-bottom: 7px
	input, :global(.date-picker input)
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
		width: 100%
		&:hover
			border: 1px solid hsla(0, 0%, 50%, 0.3)
		&:focus
			border-color: hsla(220, 100%, 50%, 1)
			box-shadow: 0px 0px 0px 3px hsla(220, 100%, 50%, 0.5)
	.buttons
		margin-top: 20px
		display: flex
		justify-content: flex-end
	.max-spacer
		margin-right: auto
	.spacer
		width: 8px
</style>
