<script lang="ts">
	import Revealed from './Revealed.svelte'
	import Download from './Download.svelte'
</script>

<svelte:head>
	<title>Kadium</title>
</svelte:head>

<Revealed
	class="ease-out-cubic mx-2 mb-4 mt-20 text-center transition-all"
	options={{ opacity: 0, scale: 0.9, duration: 750 }}
>
	<h1 class="inline-block text-7xl font-extrabold">Kadium</h1>
</Revealed>
<Revealed
	class="ease-out-cubic mx-2 mb-12 text-center transition-all"
	options={{ opacity: 0, y: -10, duration: 550, delay: 250 }}
>
	<p class="ease-out-cubic text-xl text-blue-100 opacity-60 transition-all duration-500">
		An app for staying ontop of YouTube channels' uploads
	</p>
</Revealed>

<Revealed
	class="ease-out-cubic mb-24 flex justify-center transition-all"
	options={{ opacity: 0, scale: 0.9, duration: 550, delay: 450 }}
>
	<div>
		<Download />
	</div>
</Revealed>

<Revealed
	class="relative mx-auto mb-24 max-w-5xl px-6 transition-[transform,opacity] will-change-contents sm:px-8"
	options={{ opacity: 0, y: 10, duration: 750, delay: 350 }}
>
	<div class="gradient gradient-1 show-gradient absolute inset-0 -z-10 blur-[160px]" />
	<div class="relative">
		<div class="ripple absolute inset-0 -z-10 rounded-[5px]" />
		<img src="/screenshot-1.webp" alt="" />
	</div>
</Revealed>
<Revealed
	class="relative mx-auto mb-24 max-w-5xl px-6 transition-[transform,opacity] will-change-contents sm:px-8"
	options={{ opacity: 0, y: 10, duration: 750 }}
>
	<div class="gradient gradient-2 absolute inset-0 -z-10 blur-[160px]" />
	<div class="relative">
		<div class="ripple absolute inset-0 -z-10 rounded-[5px]" />
		<img src="/screenshot-2.webp" alt="" />
	</div>
</Revealed>

<style lang="sass">
	h1
		background: linear-gradient(130deg,#09cff6 10%,#3159f6 90%)
		background-clip: text
		text-fill-color: transparent
		-webkit-text-fill-color: transparent
	.gradient
		will-change: contents
		&.gradient-1
			background: conic-gradient(from 230.29deg at 50% 50%,#2400ff 0deg,#199af5 90deg,#691eff 180deg,#00d4ff 270deg,#1fff8b 360deg)
		&.gradient-2
			background: conic-gradient(from 230.29deg at 50% 50%,#311bf3 0deg,#f5198f 90deg,#691eff 180deg,#2400ff 270deg,#199af5 360deg)
	.show-gradient
		animation: show-gradient 3s ease-in-out forwards
		opacity: 0
	@keyframes show-gradient
		0%
			opacity: 0
		50%
			opacity: 0.9
		100%
			opacity: 0.7
	.ripple
		box-shadow: 0px 0px 0px 1px hsla(0, 0%, 100%, 0.1), 0px 0px 0px 4px hsla(0, 0%, 100%, 0.15)
</style>
