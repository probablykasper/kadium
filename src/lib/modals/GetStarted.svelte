<script lang="ts">
	import { settingsOpen } from '$lib/data'
	import Modal from 'modal-svelte'
	import Button from '$lib/Button.svelte'
	import Link from '$lib/Link.svelte'

	export let visible = false
</script>

{#if visible}
	<Modal onCancel={() => (visible = false)} form={() => (visible = false)} noCloseIcon>
		<main>
			<h3>Get Started</h3>
			<p>You can add a channel on the Channels page.</p>
			<p>
				Consider opening the
				<Link
					href=""
					on:click={(e) => {
						e.preventDefault()
						$settingsOpen = true
					}}>settings</Link
				>
				to see how to get your own API key. Kadium has a default one, but it's vulnerable to abuse and
				could run out of quota.
			</p>
			<div class="buttons">
				<Button type="submit" on:click={() => (visible = false)}>Whatever</Button>
			</div>
		</main>
	</Modal>
{/if}

<style lang="sass">
	main
		max-width: 400px
	h3
		margin-top: 0px
	p
		font-size: 14px
		margin-top: 5px
		margin-bottom: 7px
	.buttons
		margin-top: 20px
		display: flex
		justify-content: flex-end
</style>
