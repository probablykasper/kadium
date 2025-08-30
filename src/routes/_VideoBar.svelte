<script lang="ts">
	import { tags, viewOptions } from '$lib/data'
	import { checkShortcut } from '$lib/general'
	import { menu_actions } from './menu'

	export let loadedVideosCount: number
	export let allLoaded: boolean

	let show = 0
	$: {
		if ($viewOptions.show_all) {
			show = 2
		} else if ($viewOptions.show_archived) {
			show = 1
		} else {
			show = 0
		}
	}
	function setShow(i: number) {
		if (i === 0) {
			$viewOptions.show_all = false
			$viewOptions.show_archived = false
		} else if (i === 1) {
			$viewOptions.show_all = false
			$viewOptions.show_archived = true
		} else if (i === 2) {
			$viewOptions.show_all = true
			$viewOptions.show_archived = false
		}
	}

	function showGroupKeydown(e: KeyboardEvent) {
		if (checkShortcut(e, 'ArrowLeft')) {
			setShow(show - 1)
			e.preventDefault()
		} else if (checkShortcut(e, 'ArrowRight')) {
			setShow(show + 1)
			e.preventDefault()
		}
	}

	function toggleTag(tag: string) {
		if ($viewOptions.tag === tag) {
			$viewOptions.tag = null
		} else {
			$viewOptions.tag = tag
		}
	}

	let filterInput: HTMLInputElement
	menu_actions.Find = () => {
		filterInput.focus()
	}

	function blurEscapeKeydown(e: KeyboardEvent) {
		if (checkShortcut(e, 'Escape')) {
			if (e.target instanceof HTMLElement) {
				e.target.blur()
				e.preventDefault()
			}
		}
	}
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<header on:keydown={blurEscapeKeydown}>
	<div class="options-bar">
		<button class="control-style group" on:keydown={showGroupKeydown}>
			<div class="item" class:selected={show === 0} on:mousedown={() => setShow(0)}>New</div>
			<div class="item" class:selected={show === 1} on:mousedown={() => setShow(1)}>Archived</div>
			<div class="item" class:selected={show === 2} on:mousedown={() => setShow(2)}>All</div>
		</button>
		<input
			bind:this={filterInput}
			class="control-style"
			type="text"
			placeholder="Channel Filter"
			bind:value={$viewOptions.channel_filter}
		/>
		{#each $tags as tag}
			<button
				class="control-style tag"
				class:enabled={$viewOptions.tag === tag}
				on:click={() => toggleTag(tag)}>{tag}</button
			>
		{/each}
	</div>
	<div class="page-info">
		{#if allLoaded}
			{loadedVideosCount} of {loadedVideosCount}
		{:else}
			{loadedVideosCount} of ?
		{/if}
	</div>
</header>

<style lang="sass">
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
		margin-left: 5px
		font-size: 13px
		opacity: 0.7
	.options-bar
		box-sizing: border-box
		align-items: center
		display: flex
		flex-wrap: wrap
	.control-style
		border-radius: 3px
		border: 1px solid hsl(233, 7%, 22%)
		background-color: hsla(223, 33%, 64%, 0.12)
		outline: none
		margin: 10px 0px
		margin-right: 8px
		font-size: 13px
		color: inherit
	.group
		padding: 0px
		height: 28px
		transition: all 140ms var(--ease-out-cubic)
		&:focus
			border-color: hsla(220, 100%, 50%, 1)
			box-shadow: 0px 0px 0px 3px hsla(220, 100%, 50%, 0.5)
			.item.selected
				background-color: hsla(220, 100%, 50%, 1)
		.item
			float: left
			display: flex
			align-items: center
			height: 100%
			padding: 0px 11px
			transition: background-color 140ms var(--ease-out-cubic)
			&.selected
				background-color: hsl(225, 14%, 28%)
	button.tag
		height: 24px
		font-size: 12px
		box-sizing: border-box
		padding: 0px 8px
		border-radius: 10px
		margin-right: 5px
		transition: all 140ms var(--ease-out-cubic)
		&:hover
			border-color: hsla(0, 0%, 50%, 0.5)
		&:focus
			border-color: hsla(220, 100%, 50%, 1)
			box-shadow: 0px 0px 0px 3px hsla(220, 100%, 50%, 0.5)
		&.enabled
			background-color: hsl(220, 14%, 28%)
			border-color: hsl(220, 10%, 50%)
			color: #ffffff
		&.enabled:focus
			background-color: hsla(220, 100%, 50%, 1)
			border-color: hsla(220, 100%, 60%, 1)
	input
		height: 28px
		box-sizing: border-box
		padding: 0px 6px
		transition: all 140ms var(--ease-out-cubic)
		&:focus
			border-color: hsla(220, 100%, 50%, 1)
			box-shadow: 0px 0px 0px 3px hsla(220, 100%, 50%, 0.5)
</style>
