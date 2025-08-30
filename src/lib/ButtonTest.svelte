<script lang="ts">
	import { example_multitype } from './external'

	console.log('prefix')
	export let inferred = example_multitype()
	export let explicit: number | null = null
	console.log('middle')
	export let required: boolean
	console.log('suffix')

	let a_var = $state('hey')
</script>

<button>{required}</button>

<input type="text" bind:value={a_var} />
{a_var}
