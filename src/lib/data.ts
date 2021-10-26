import { writable } from 'svelte/store'
import type { Writable } from 'svelte/store'
import { runCmd } from './general'

export type Channel = {
  id: string
  name: string
  icon: string
  uploads_playlist_id: string
  from_time: number
  minutes_between_refreshes: number
  tags: string[]
}

export type Settings = {
  api_key: string
  max_concurrent_requests: number
  channels: Channel[]
}

export const settings: Writable<null | Settings> = writable(null)
export function loadSettings() {
  runCmd('get_settings').then(async (settingsResponse: Settings) => {
    settings.set(settingsResponse)
    if (settingsResponse.channels.length === 0) {
      const importedSettings = await runCmd('maybe_ask_for_import')
      if (importedSettings !== null) {
        settings.set(importedSettings)
      }
    }
  })
}
export function reloadSettings() {
  runCmd('get_settings').then(async (settingsResponse: Settings) => {
    settings.set(settingsResponse)
  })
}

export function useSampleSettings() {
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
          minutes_between_refreshes: 60,
          tags: ['Chungus'],
        })
      }
      return channels
    })(),
  })
}
