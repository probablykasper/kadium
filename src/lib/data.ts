import { writable } from 'svelte/store'
import type { Writable } from 'svelte/store'
import { runCmd } from './general'

export type Channel = {
  id: string
  name: string
  icon: string
  uploads_playlist_id: string
  from_time: number
  refresh_rate_ms: number
  tags: string[]
}

export type Settings = {
  api_key: string
  max_concurrent_requests: number
  channels: Channel[]
  check_in_background: boolean
}

export type Video = {
  id: string
  title: string
  description: string
  publishTimeMs: number
  durationMs: number
  thumbnailStandard: boolean
  thumbnailMaxres: boolean
  channelId: string
  channelName: string
  unread: boolean
  archived: boolean
}

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
  limit: 50,
})

export const videos = writable([] as Video[])

export const settingsOpen = writable(false)
export const settings: Writable<null | Settings> = writable(null)
export const tags = writable([] as string[])
export async function loadSettings() {
  await runCmd('get_settings').then((settingsResponse: Settings) => {
    settings.set(settingsResponse)
  })
  await runCmd('tags').then((tagsResponse) => {
    tags.set(tagsResponse)
  })
}

export function enableSampleData() {
  videos.set(
    (() => {
      const videos = []
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
    })()
  )
  settings.set({
    api_key: 'example key',
    max_concurrent_requests: 5,
    channels: (() => {
      const channels = []
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
  })
}
