<script lang="ts">
	// @ts-ignore
	import Changelog from '../../../../CHANGELOG.md'
	import Revealed from '../Revealed.svelte'
</script>

<svelte:head>
	<title>Changelog • Kadium</title>
</svelte:head>

<div class="changelog mx-auto mb-24 max-w-3xl p-6">
	<Revealed options={{ duration: 1000, opacity: 0 }}>
		<Changelog />
	</Revealed>
</div>

<style lang="sass">
	.changelog
		:global(h1)
			text-align: center
			margin: auto
			margin-top: 2rem
			margin-bottom: 4rem
			font-size: 3rem
			font-weight: 800
			background: linear-gradient(130deg,#f844cb 10%,#fd713a 90%)
			background-clip: text
			text-fill-color: transparent
			-webkit-text-fill-color: transparent
			position: relative

			&::before
				content: ''
				pointer-events: none
				position: absolute
				inset: 0
				will-change: contents
				animation: show-gradient 1s ease-out forwards
				opacity: 0
				animation-delay: 200ms
				@keyframes show-gradient
					0%
						opacity: 0
					100%
						opacity: 0.7
				width: 100%
				height: 100%
				filter: blur(80px)
				transform: scale(1.2)
				background: conic-gradient(from 150deg at 50% 50%,#d000ff 0deg,#f51971 90deg,#ff781f 180deg,#ff0059 270deg,#6d1fff 360deg)
		:global(h2)
			font-size: 1.5rem
			font-weight: 500
			margin-top: 3rem
			margin-bottom: 1rem
		:global(li)
			font-size: 1rem
			line-height: 2
			list-style-type: disc
			margin-left: 2rem
</style>
