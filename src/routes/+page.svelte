<script lang="ts">
	import { type ViewOptions, viewOptions } from '$lib/data'
	import { openUrl } from '@tauri-apps/plugin-opener'
	import { listen } from '@tauri-apps/api/event'
	import { onDestroy, tick } from 'svelte'
	import type { Video } from '../../bindings'
	import { checkModifiers, checkShortcut } from '$lib/general'
	import VideoBar from './_VideoBar.svelte'
	import commands from '$lib/commands'
	import { menu_actions } from './menu'

	let videos: Video[] = []
	let allLoaded = false
	$: getVideos($viewOptions)
	let loading = 0
	async function getVideos(options: ViewOptions) {
		loading++

		const selectedId = videos[selectedIndex]?.id
		const oldselectedIndex = selectedIndex

		const newVideos = await commands.getVideos(options, null)
		if (newVideos.status === 'ok') {
			allLoaded = newVideos.data.length < $viewOptions.limit
			videos = newVideos.data
		}

		// Update the selection index if the video moves
		const newSelectedIndex = videos.findIndex((v) => v.id === selectedId)
		if (newSelectedIndex >= 0 && selectionVisible) {
			selectedIndex = newSelectedIndex
		} else {
			// Or clear selection if the video disappeared from view
			selectedIndex = 0
			selectionVisible = false
		}
		const selectionMoved = selectedIndex !== oldselectedIndex && selectionVisible

		if (selectionMoved) {
			allowScrollToBox = false
			await tick()
			allowScrollToBox = true
			scrollToBox(selectedIndex)
		} else {
			await tick()
		}
		await autoloadHandler()
		loading--
	}
	async function softRefresh(startIndex: number) {
		loading++

		if (startIndex === 0) {
			const newVideos = await commands.getVideos($viewOptions, null)
			if (newVideos.status === 'ok') {
				allLoaded = newVideos.data.length < $viewOptions.limit
				videos = newVideos.data
			}
		} else {
			const prevVideo = videos[startIndex - 1]
			const prevVideos = videos.slice(0, startIndex)
			const maxLength = videos.length
			const reloadedVideos = await commands.getVideos(
				{
					...$viewOptions,
					limit: $viewOptions.limit * 2,
				},
				{
					publishTimeMs: prevVideo.publishTimeMs,
					id: prevVideo.id,
				},
			)

			if (reloadedVideos.status === 'ok') {
				videos = prevVideos.concat(reloadedVideos.data)
				// Shorten length to a mulpitle of `limit`
				if (videos.length > $viewOptions.limit) {
					videos = videos.slice(0, maxLength - (maxLength % $viewOptions.limit))
				}
				videos = videos.slice(0, maxLength)
			}
		}

		await tick()
		await autoloadHandler()
		loading--
	}
	async function getMoreVideos() {
		loading++

		const newVideos = await commands.getVideos($viewOptions, {
			publishTimeMs: videos[videos.length - 1].publishTimeMs,
			id: videos[videos.length - 1].id,
		})

		if (newVideos.status === 'ok') {
			allLoaded = newVideos.data.length < $viewOptions.limit
			videos = videos.concat(newVideos.data)
		}

		await tick()
		await autoloadHandler()
		loading--
	}
	async function autoloadHandler() {
		if (!allLoaded && isScrolledNearBottom() && !loading) {
			await getMoreVideos()
		}
	}

	const refreshUnlistener = listen('refresh', () => {
		getVideos($viewOptions)
	})
	onDestroy(async () => {
		;(await refreshUnlistener)()
	})

	const months = [
		'Jan',
		'Feb',
		'Mar',
		'Apr',
		'May',
		'Jun',
		'Jul',
		'Aug',
		'Sep',
		'Oct',
		'Nov',
		'Dec',
	]
	function formatDate(timestamp: number) {
		let ts = new Date(timestamp)
		return ts.getDate() + ' ' + months[ts.getMonth()] + ' ' + ts.getFullYear()
	}
	async function archive(i: number) {
		loading++
		const video = videos[i]
		await commands.archive(video.id)
		await softRefresh(i)
		selectedIndex = Math.min(selectedIndex, videos.length - 1)
		loading--
	}
	async function unarchive(i: number) {
		loading++
		const video = videos[i]
		await commands.unarchive(video.id)
		await softRefresh(i)
		selectedIndex = Math.min(selectedIndex, videos.length - 1)
		loading--
	}

	let scrollDiv: HTMLElement | null = null
	function isScrolledNearBottom() {
		if (scrollDiv) {
			const lastElement = boxes[videos.length - 1]
			const threshold = lastElement.offsetTop - 200
			const scrollBottom = scrollDiv.clientHeight + scrollDiv.scrollTop
			if (scrollBottom >= threshold) {
				return true
			}
		}
		return false
	}

	let grid: HTMLDivElement

	let selectionVisible = false
	let selectedIndex = 0
	function select(index: number) {
		selectedIndex = index
		selectionVisible = true
	}

	function openVideo(index: number) {
		openUrl('https://youtube.com/watch?v=' + videos[index].id)
	}
	function openChannel(index: number) {
		openUrl('https://www.youtube.com/channel/' + videos[index].channelId)
	}
	function getColumnCount() {
		const gridStyle = window.getComputedStyle(grid)
		const gridTemplateCols = gridStyle.getPropertyValue('grid-template-columns')
		return gridTemplateCols.split(' ').length
	}

	function keydown(e: KeyboardEvent) {
		let target = e.target as HTMLElement
		if (target.nodeName === 'INPUT') {
			return
		}

		if (selectionVisible) {
			if (checkShortcut(e, 'ArrowLeft')) {
				if (selectedIndex % getColumnCount() !== 0) {
					selectedIndex--
					selectedIndex = Math.max(0, selectedIndex)
				}
				e.preventDefault()
			} else if (checkShortcut(e, 'ArrowRight')) {
				if ((selectedIndex + 1) % getColumnCount() !== 0) {
					selectedIndex++
					selectedIndex = Math.min(selectedIndex, videos.length - 1)
				}
				e.preventDefault()
			} else if (checkShortcut(e, 'ArrowUp')) {
				const columnCount = getColumnCount()
				if (selectedIndex - columnCount >= 0) {
					selectedIndex -= columnCount
				}
				e.preventDefault()
			} else if (checkShortcut(e, 'ArrowDown')) {
				const columnCount = getColumnCount()
				selectedIndex += columnCount
				selectedIndex = Math.min(selectedIndex, videos.length - 1)
				e.preventDefault()
			} else if (checkShortcut(e, 'Escape')) {
				selectionVisible = false
				e.preventDefault()
			} else if (checkShortcut(e, 'Enter')) {
				openVideo(selectedIndex)
				e.preventDefault()
			} else if (checkShortcut(e, 'Enter', { shift: true })) {
				openChannel(selectedIndex)
				e.preventDefault()
			}
		} else {
			if (
				checkShortcut(e, 'ArrowLeft') ||
				checkShortcut(e, 'ArrowRight') ||
				checkShortcut(e, 'ArrowUp') ||
				checkShortcut(e, 'ArrowDown')
			) {
				selectionVisible = true
				e.preventDefault()
			}
		}
	}
	function videoClick(e: MouseEvent, index: number) {
		if (checkModifiers(e, {})) {
			selectedIndex = index
		} else if (checkModifiers(e, { cmdOrCtrl: true })) {
			openVideo(index)
		}
	}
	function channelClick(e: MouseEvent, index: number) {
		if (checkModifiers(e, { cmdOrCtrl: true })) {
			openChannel(index)
		}
	}

	let dragEl: HTMLElement
	let dragElDiv: HTMLElement
	function dragStartVideo(e: DragEvent, video: Video) {
		if (e.dataTransfer) {
			e.dataTransfer.effectAllowed = 'link'
			dragElDiv.innerText = video.title
			e.dataTransfer.setDragImage(dragEl, 0, 0)
			e.dataTransfer.setData('text/uri-list', 'https://youtube.com/watch?v=' + video.id)
			e.dataTransfer.setData('text/plain', 'https://youtube.com/watch?v=' + video.id)
		}
	}
	function dragStartChannel(e: DragEvent, video: Video) {
		if (e.dataTransfer) {
			e.dataTransfer.effectAllowed = 'link'
			dragElDiv.innerText = video.channelName
			e.dataTransfer.setDragImage(dragEl, 0, 0)
			e.dataTransfer.setData('text/uri-list', 'https://youtube.com/channel/' + video.channelId)
			e.dataTransfer.setData('text/plain', 'https://youtube.com/channel/' + video.channelId)
		}
	}

	menu_actions.Open = () => {
		if (selectionVisible) openVideo(selectedIndex)
	}
	menu_actions['Open Channel'] = () => {
		if (selectionVisible) openChannel(selectedIndex)
	}
	menu_actions.Archive = () => {
		if (selectionVisible) archive(selectedIndex)
	}
	menu_actions.Unarchive = () => {
		if (selectionVisible) unarchive(selectedIndex)
	}
	onDestroy(() => {
		menu_actions.Open = undefined
		menu_actions['Open Channel'] = undefined
		menu_actions.Archive = undefined
		menu_actions.Unarchive = undefined
	})

	let boxes: HTMLDivElement[] = []
	$: scrollToBox(selectedIndex)
	let allowScrollToBox = true
	function scrollToBox(index: number) {
		if (scrollDiv && boxes[index] && allowScrollToBox) {
			const el = boxes[index].getBoundingClientRect()
			const parent = scrollDiv.getBoundingClientRect()
			const topOffset = el.top - parent.top
			const bottomOffset = el.bottom - parent.bottom
			if (topOffset < 0) {
				scrollDiv.scrollTop += topOffset - 5
			} else if (bottomOffset > 0) {
				scrollDiv.scrollTop += bottomOffset + 5
			}
		}
	}
</script>

<svelte:window on:resize={autoloadHandler} on:keydown|self={keydown} />
<svelte:body on:keydown|self={keydown} />

<VideoBar loadedVideosCount={videos.length} {allLoaded} />

<div class="drag-ghost" bind:this={dragEl}>
	<div bind:this={dragElDiv} />
</div>

<main on:scroll={autoloadHandler} bind:this={scrollDiv}>
	<div class="grid" bind:this={grid}>
		{#each videos as video, i (video.id)}
			<!-- svelte-ignore a11y-click-events-have-key-events -->
			<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
			<div
				class="box"
				class:selected={selectionVisible && i === selectedIndex}
				bind:this={boxes[i]}
				on:mousedown={() => select(i)}
				on:dblclick={() => openUrl('https://youtube.com/watch?v=' + video.id)}
				on:click={(e) => videoClick(e, i)}
				draggable="true"
				on:dragstart={(e) => dragStartVideo(e, video)}
				role="listitem"
			>
				<div class="img-box">
					<div class="img-parent">
						<img
							src="https://i.ytimg.com/vi/{video.id}/hqdefault.jpg"
							alt=""
							draggable="false"
							loading="lazy"
						/>
					</div>
				</div>
				<!-- svelte-ignore a11y-no-static-element-interactions -->
				<div
					class="archive"
					on:click={() => {
						if (video.archived) unarchive(i)
						else archive(i)
					}}
					on:dblclick|stopPropagation
					title="Archive"
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="24"
						height="24"
						viewBox="0 0 24 24"
						class:archived={video.archived}
					>
						<path
							class="frame"
							d="M24,3.382c0,-1.866 -1.516,-3.382 -3.382,-3.382l-17.236,0c-1.866,0 -3.382,1.516 -3.382,3.382l0,17.236c0,1.866
					1.516,3.382 3.382,3.382l17.236,-0c1.866,-0 3.382,-1.516 3.382,-3.382l0,-17.236Zm-2.5,0l-0,17.236c-0,0.487
					-0.395,0.882 -0.882,0.882l-17.236,-0c-0.487,-0 -0.882,-0.395 -0.882,-0.882l0,-17.236c0,-0.487
					0.395,-0.882 0.882,-0.882l17.236,0c0.487,0 0.882,0.395 0.882,0.882Z"
						/>
						<path
							class="checkmark"
							d="M9.348,14.652l8.839,-8.839l1.768,1.768l-10.607,10.606l-5.303,-5.303l1.768,-1.768l3.535,3.536Z"
						/>
					</svg>
				</div>
				<div class="row">
					<p class="title selectable">{video.title}</p>
				</div>
				<p
					class="channel sub"
					on:click|stopPropagation={(e) => channelClick(e, i)}
					on:dblclick|stopPropagation={() => openChannel(i)}
					draggable="true"
					on:dragstart|stopPropagation={(e) => dragStartChannel(e, video)}
				>
					{video.channelName}
				</p>
				<p class="row sub selectable">{formatDate(Number(video.publishTimeMs))}</p>
			</div>
		{/each}
	</div>
</main>

<style lang="sass">
	.selectable
		user-select: text
	main
		overflow-y: auto
		max-width: 100%
	.drag-ghost
		font-size: 14px
		top: -1000px
		position: absolute
		background-color: transparent
		padding-left: 3px
		div
			background-color: hsl(220, 17%, 7%)
			padding: 4px 8px
			max-width: 300px
			border-radius: 3px
	.grid
		box-sizing: border-box
		display: grid
		grid-template-columns: repeat(auto-fill, minmax(210px, 1fr))
		grid-gap: 5px
		@media screen and (min-width: 1000px)
			grid-template-columns: repeat(auto-fill, minmax(220px, 1fr))
			grid-gap: 10px
		@media screen and (min-width: 1500px)
			grid-template-columns: repeat(auto-fill, minmax(230px, 1fr))
			grid-gap: 10px
		padding: var(--page-padding)
		@media screen and (max-width: 450px)
			grid-template-columns: 1fr
			.box
				margin: 0px auto
	.box
		max-width: 280px
		width: 100%
		user-select: none
		position: relative
		padding: 3px
		border: 1px solid transparent
		border-radius: 3px
		box-sizing: border-box
	.selected
		background-color: hsla(210, 100%, 95%, 0.07)
		border-color: hsla(210, 100%, 90%, 0.25)
	.img-box
		display: block
		width: 100%
		padding-top: 56.25%
		position: relative
	.img-parent
		position: absolute
		top: 0px
		left: 0px
		width: 100%
		height: 100%
		overflow: hidden
		display: flex
		align-items: center
	img
		width: 100%
		top:-100%
		bottom:-100%
	p
		margin: 0px
	p.title
		font-size: 13px
		font-weight: 500
		color: #ffffff
		opacity: 1
		margin-top: 1px
	.channel:hover
		color: hsl(210, 8%, 90%)
	p.sub
		font-size: 12px
		color: hsl(210, 8%, 80%)
		margin-top: 2px
	.box:hover .archive
		opacity: 1
	.archive
		position: absolute
		cursor: pointer
		top: 0px
		right: 0px
		border-radius: 10px
		box-shadow: inset 0px 0px 10px 8px rgba(0, 0, 0, 0.15), 0px 0px 12px 12px rgba(0, 0, 0, 0.15)
		background-color: transparent
		margin-right: 2px
		margin-top: 2px
		padding: 0px
		border: none
		transform: translate3d(0, 0, 0) // fix glitch after transform/opacity
		opacity: 0
		transition: opacity 140ms var(--ease-out-cubic)
		svg
			fill: #ffffff
			width: 16px
			height: 16px
			padding-top: 4px
			padding-right: 4px
			padding-bottom: 2px
			padding-left: 2px
			vertical-align: middle
			filter: drop-shadow( 0px 0px 2px rgba(0, 0, 0, 0.4))
			path.frame
				transform: scale(1)
				transform-origin: center
		svg.archived
			path.checkmark
				transform: scale(1)
				transform-origin: 20% 80%
				transition: all 140ms var(--ease-out-cubic)
			&:active path.checkmark
				transform: scale(0.8)
				opacity: 0
		svg:not(.archived)
			path.checkmark
				transform: scale(0.8)
				opacity: 0
				transition: all 140ms var(--ease-out-cubic)
			&:active path.checkmark
				opacity: 1
				transform: scale(1)
</style>
