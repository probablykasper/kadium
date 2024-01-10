<script lang="ts">
	import { checkShortcut } from './general'

	export let value: string[]
	export let onUpdate: () => void
	let inputEl: HTMLInputElement

	let editing = false
	function startEditing() {
		editing = true
		text = value[value.length - 1]
	}
	function startAdding() {
		editing = false
		text = ''
	}
	function applyEditing() {
		if (editing) {
			if (text === '') {
				value.pop()
				update(value)
			} else if (text !== value[value.length - 1]) {
				value[value.length - 1] = text
				update(value)
			}
			text = ''
			editing = false
		}
	}

	let text = ''

	function update(newValue: string[]) {
		value = newValue
		onUpdate()
	}

	function onFocus() {
		text = ''
	}
	function onBlur() {
		if (editing) {
			applyEditing()
		} else if (text !== '') {
			value.push(text)
			update(value)
		}
		text = ''
	}
	let tagXEls: HTMLButtonElement[] = []
	async function editingKeydown(e: KeyboardEvent) {
		if (checkShortcut(e, 'Enter')) {
			applyEditing()
		} else if (checkShortcut(e, 'Backspace')) {
			if (text === '' && value.length >= 2) {
				e.preventDefault()
				value.pop()
				update(value)
				startEditing()
			}
		} else if (checkShortcut(e, 'Escape')) {
			startAdding()
			e.preventDefault()
		}
	}
	async function keydown(e: KeyboardEvent) {
		if (editing) {
			editingKeydown(e)
			return
		}
		if (checkShortcut(e, 'Enter')) {
			if (text !== '') {
				value.push(text)
				update(value)
				startAdding()
			}
		} else if (checkShortcut(e, 'Backspace')) {
			if (text === '' && value.length >= 1) {
				e.preventDefault()
				startEditing()
			}
		} else if (checkShortcut(e, 'Escape')) {
			e.preventDefault()
		}
	}
	function remove(i: number) {
		value.splice(i, 1)
		update(value)
	}
	function tagKeydown(e: KeyboardEvent, i: number) {
		if (checkShortcut(e, 'Backspace')) {
			value.splice(i, 1)
			update(value)
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
		{#if i === value.length - 1 && editing}
			<!-- hide last element when editing -->
		{:else}
			<div class="tag">
				{tag}<button
					bind:this={tagXEls[i]}
					on:keydown={(e) => tagKeydown(e, i)}
					on:click={() => remove(i)}
					tabindex="0">×</button
				>
			</div>
		{/if}
	{/each}
	<input
		bind:this={inputEl}
		type="text"
		placeholder="Add tags..."
		on:focus={onFocus}
		on:blur={onBlur}
		on:keydown={keydown}
		bind:value={text}
	/>
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
