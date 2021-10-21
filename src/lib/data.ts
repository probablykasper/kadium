export type Channel = {
  id: string
  name: string
  icon: string
  uploads_playlist_id: string
  from_time: number
}
export type Group = {
  email: string
  minutes_between_refreshes: number
  channels: Channel[]
}
export type Settings = {
  api_key: string
  from_email: string
  unread_errors: boolean
  max_concurrent_requests: number
  groups: Group[]
}
