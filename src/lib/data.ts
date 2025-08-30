import { writable } from 'svelte/store'
import type { Writable } from 'svelte/store'
import commands from './commands'
import type { Channel, Settings, Video } from '../../bindings'

export type ViewOptions = {
	show_all: boolean
	show_archived: boolean
	channel_filter: string
	tag: string | null
	limit: number
}
export const viewOptions: Writable<ViewOptions> = writable({
	show_all: false,
	show_archived: false,
	channel_filter: '',
	tag: null,
	limit: 100,
})

export const videos: Writable<Video[]> = writable([])

export const settingsOpen = writable(false)
export const settings: Writable<null | Settings> = writable(null)
export const tags: Writable<string[]> = writable([])
export async function loadSettings() {
	const settingsResult = await commands.getSettings()
	if (settingsResult.status === 'ok') {
		settings.set(settingsResult.data)
	}

	const tagsResult = await commands.tags()
	if (tagsResult.status === 'ok') {
		tags.set(tagsResult.data)
	}
}

export function enableSampleData() {
	videos.set(
		(() => {
			const videos: Video[] = []
			for (let i = 0; i < 100; i++) {
				videos.push({
					archived: false,
					channelId: 'UC9RM-iSvTu1uPJb8X5yp3EQ',
					channelName: 'Wendover Productions ' + i,
					description: '',
					durationMs: 1093000,
					id: 'aH4b3sAs-l8',
					publishTimeMs: 1623861277000,
					thumbnailMaxres: true,
					thumbnailStandard: true,
					title: 'Why Electric Planes are Inevitably Coming',
					unread: true,
				})
			}
			return videos
		})(),
	)
	settings.set({
		api_key: 'example key',
		max_concurrent_requests: 5,
		channels: (() => {
			const channels: Channel[] = []
			for (let i = 0; i < 100; i++) {
				channels.push({
					from_time: 1611870142000,
					icon: 'https://yt3.ggpht.com/ytc/AAUvwni4bZoon2txFxQCiRVUoabFsxFhth0z5W89mymg=s240-c-k-c0x00ffffff-no-rj',
					id: 'UCp4csaOD64mSzPxbfuzJcuA',
					name: 'Chuckle Sandwich ' + i,
					uploads_playlist_id: 'UUp4csaOD64mSzPxbfuzJcuA',
					refresh_rate_ms: 60 * 1000,
					tags: ['Chungus'],
				})
			}
			return channels
		})(),
		check_in_background: true,
		window_decorations: true,
	})
}
