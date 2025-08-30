<script lang="ts">
	import { example_multitype } from './external'

	console.log('prefix')
	export let inferred = example_multitype()
	export let explicit: number | null = null
	console.log('middle')
	export let required: boolean
	export let implicit_any
	console.log('suffix')
</script>

<button>
	{@render implicit_any?.()}
</button>
