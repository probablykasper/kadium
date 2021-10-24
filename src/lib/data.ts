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
