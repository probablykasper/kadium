<script lang="ts">
	export let isOpen = false

	function toggle() {
		isOpen = !isOpen
	}
	function close() {
		isOpen = false
	}

	let parent: HTMLDivElement
	function focusout(e: FocusEvent) {
		if (parent.contains(document.activeElement)) {
			return
		} else if (e.relatedTarget === null) {
			isOpen = false
		} else if (e.relatedTarget instanceof HTMLElement) {
			const stayingInParent = parent.contains(e.relatedTarget)
			if (!stayingInParent && !e.relatedTarget.isSameNode(parent)) {
				isOpen = false
			}
		}
	}
</script>

<div class="relative" bind:this={parent} on:focusout={focusout} tabindex="-1">
	<slot {toggle} {isOpen} {close} />
	{#if isOpen}
		<div class="absolute z-10 w-full divide-y divide-gray-100 text-sm text-gray-700 shadow">
			<slot name="popup" />
		</div>
	{/if}
</div>
