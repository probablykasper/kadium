<script lang="ts">
	import { openUrl } from '@tauri-apps/plugin-opener'
	import { commands } from '../../../bindings'
	import Link from '$lib/Link.svelte'

	const history = commands.getHistory().then((result) => {
		if (result.status == 'ok') {
			return result.data
		} else {
			throw new Error(`Failed to fetch history: ${result.error}`)
		}
	})
</script>

<main>
	<h1>Basic-ass history page</h1>

	<p class="dark">The history is lost when you close the app</p>

	{#await history then history}
		{#if history.entries.length === 0}
			Empty! When you do things, it will show here.
		{/if}
		<table>
			{#each history.entries.reverse() as [timestamp, action]}
				<!-- svelte-ignore node_invalid_placement_ssr -->
				<tr>
					<td class="timestamp dark">
						{new Date(timestamp * 1000).toLocaleString()}
					</td>
					<td>
						{#if action === 'CheckNow'}
							Manually check for videos
						{:else if 'Archive' in action}
							{@const id = action.Archive}
							Archive video ID <Link
								on:click={() => openUrl(`https://www.youtube.com/watch?v=${id}`)}
								>{action.Archive}</Link
							>
						{:else if 'Unarchive' in action}
							{@const id = action.Unarchive}
							Unarchive video ID <Link
								on:click={() => openUrl(`https://www.youtube.com/watch?v=${id}`)}
								>{action.Unarchive}</Link
							>
						{:else if 'AddChannel' in action}
							{@const id = action.AddChannel}
							Added channel <Link on:click={() => openUrl(`https://www.youtube.com/channel/${id}`)}
								>{action.AddChannel}</Link
							>
						{:else if 'UpdateOrDeleteChannels' in action}
							Updated or deleted channel(s)
						{/if}
					</td>
				</tr>
			{/each}
		</table>
		{#if history.entries.length >= 100}
			<p>The end. Only 100 entries are shown.</p>
		{/if}
	{:catch error}
		Error {error}
	{/await}
</main>

<style lang="sass">
	main
		padding: 20px
		padding-top: 0px
		overflow-y: auto
	.dark
		color: hsl(210, 8%, 80%)
	.timestamp
		padding-right: 10px
</style>
