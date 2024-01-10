<script lang="ts">
	export let href = ''
	let target = '_self'
	$: if (href.startsWith('/') || href.startsWith('./')) {
		target = '_self'
	} else {
		target = '_blank'
	}
</script>

{#if href === ''}
	<button type="button" on:click><slot /></button>
{:else}
	<a {href} {target}><slot /></a>
{/if}

<style lang="sass">
	button
		font-size: 13px
		margin: 0px
		background-color: transparent
		padding: 0px
		border: none
		cursor: default
	a, button
		color: hsl(210, 100%, 55%)
		&:hover
			color: hsl(210, 100%, 45%)
	a
		text-decoration: none
		font-size: inherit
</style>
