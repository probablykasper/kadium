<script lang="ts">
	import { tick } from 'svelte'
	import { checkShortcut } from './general'

	export let value: string[]
	export let onUpdate: () => void
	let inputEl: HTMLInputElement
	let text = ''
	let editingIndex: number | null = null

	async function startAdding() {
		editingIndex = null
		await tick()
		inputEl.focus()
		text = ''
	}
	async function startEditing(index: number) {
		editingIndex = index
		await tick()
		inputEl.focus()
		text = value[index]
	}

	function add(text: string) {
		value.push(text)
		update(value)
	}
	function applyEditing() {
		if (editingIndex !== null) {
			if (text === '') {
				value.splice(editingIndex, 1)
				update(value)
			} else if (text !== value[editingIndex]) {
				value[editingIndex] = text
				update(value)
			}
			text = ''
			editingIndex = null
		}
	}
	function remove(i: number) {
		value.splice(i, 1)
		update(value)
	}

	function update(newValue: string[]) {
		value = newValue
		onUpdate()
	}

	function onFocus() {
		text = ''
	}
	function onBlur() {
		if (editingIndex !== null) {
			applyEditing()
		} else if (text !== '') {
			add(text)
		}
		text = ''
	}
	let tagXEls: HTMLButtonElement[] = []
	async function keydown(e: KeyboardEvent) {
		if (editingIndex !== null) {
			editingKeydown(e)
			return
		}
		if (checkShortcut(e, 'Enter')) {
			if (text !== '') {
				add(text)
				startAdding()
			}
		} else if (checkShortcut(e, 'Backspace')) {
			if (text === '' && value.length >= 1) {
				e.preventDefault()
				startEditing(value.length - 1)
			}
		}
	}
	async function editingKeydown(e: KeyboardEvent) {
		if (checkShortcut(e, 'Enter')) {
			applyEditing()
			startAdding()
		} else if (checkShortcut(e, 'Backspace') && editingIndex !== null) {
			if (text === '' && value.length >= 2) {
				e.preventDefault()
				remove(editingIndex)
				startEditing(editingIndex - 1)
			}
		} else if (checkShortcut(e, 'Escape')) {
			startAdding()
		}
	}
	function tagKeydown(e: KeyboardEvent, i: number) {
		if (checkShortcut(e, 'Backspace')) {
			remove(i)
			if (i >= 1) {
				tagXEls[i - 1].focus()
			} else {
				inputEl.focus()
			}
		}
	}
</script>

<div class="tags">
	<div class="label">Tags</div>
	{#each value as tag, i}
		{#if i === editingIndex}
			<input
				bind:this={inputEl}
				type="text"
				placeholder="Add tags..."
				on:focus={onFocus}
				on:blur={onBlur}
				on:keydown={keydown}
				bind:value={text}
			/>
		{:else}
			<div class="tag">
				<span
					role="none"
					on:dblclick={() => {
						startEditing(i)
					}}>{tag}</span
				><button
					bind:this={tagXEls[i]}
					on:keydown={(e) => tagKeydown(e, i)}
					on:click={() => remove(i)}
					tabindex="0">×</button
				>
			</div>
		{/if}
	{/each}
	{#if editingIndex === null}
		<input
			bind:this={inputEl}
			type="text"
			placeholder="Add tags..."
			on:focus={onFocus}
			on:blur={onBlur}
			on:keydown={keydown}
			bind:value={text}
		/>
	{/if}
</div>

<style lang="sass">
	.tags
		color: hsla(231, 20%, 100%, 0.5)
		font-size: 14px
		display: flex
		flex-wrap: wrap
		align-items: center
	.label, .tag, input
		margin-right: 4px
		margin-top: 4px
		height: 20px
		line-height: 20px
	.tag
		background-color: hsla(230, 20%, 70%, 0.1)
		color: hsla(231, 10%, 90%)
		padding: 0px 4px
		border-radius: 3px
		border: 1px solid hsla(230, 20%, 70%, 0.1)
		display: inline-block
		button
			display: inline-block
			padding: 0px 3px
			margin: 0px
			margin-right: -1px
			background-color: transparent
			font-size: inherit
			border: 2px solid transparent
			line-height: 1
			outline: none
			border-radius: 3px
			color: hsla(231, 10%, 90%)
			&:hover
				color: hsla(231, 10%, 70%)
			&:focus
				border-color: hsl(210, 100%, 55%)
	input
		width: 75px
		background-color: transparent
		font-size: inherit
		padding: 1px 4px
		border: 1px solid transparent
		&:not(:focus):global(::placeholder)
			color: hsl(210, 100%, 45%)
		&:focus
			width: 150px
			background-color: hsla(230, 20%, 70%, 0.1)
			color: hsla(231, 20%, 100%, 0.8)
			border-radius: 3px
			border: 1px solid hsla(230, 20%, 70%, 0.1)
			outline: none
		&:empty::before
			content: 'Add tags...'
			opacity: 0.5
</style>
