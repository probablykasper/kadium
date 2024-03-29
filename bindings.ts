/* eslint-disable */
// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

declare global {
    interface Window {
        __TAURI_INVOKE__<T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
    }
}

// Function avoids 'window not defined' in SSR
const invoke = () => window.__TAURI_INVOKE__;

export function errorPopup(msg: string) {
    return invoke()<null>("error_popup", { msg })
}

export function getSettings() {
    return invoke()<Settings>("get_settings")
}

export function tags() {
    return invoke()<string[]>("tags")
}

export function setChannels(channels: Channel[]) {
    return invoke()<null>("set_channels", { channels })
}

export function addChannel(options: AddChannelOptions) {
    return invoke()<null>("add_channel", { options })
}

export function setGeneralSettings(apiKey: string, maxConcurrentRequests: number, checkInBackground: boolean) {
    return invoke()<null>("set_general_settings", { apiKey,maxConcurrentRequests,checkInBackground })
}

export function checkNow() {
    return invoke()<null>("check_now")
}

export function getHistory() {
    return invoke()<UndoHistory>("get_history")
}

export function getVideos(options: Options, after: After | null) {
    return invoke()<Video[]>("get_videos", { options,after })
}

export function archive(id: string) {
    return invoke()<null>("archive", { id })
}

export function unarchive(id: string) {
    return invoke()<null>("unarchive", { id })
}

export type Settings = { api_key: string; max_concurrent_requests: number; channels: Channel[]; check_in_background: boolean }
export type Options = { show_all: boolean; show_archived: boolean; channel_filter: string; tag: string | null; limit: number }
export type Video = { id: string; title: string; description: string; publishTimeMs: number; durationMs: number; thumbnailStandard: boolean; thumbnailMaxres: boolean; channelId: string; channelName: string; unread: boolean; archived: boolean }
export type After = { publishTimeMs: number; id: string }
export type Channel = { id: string; name: string; icon: string; uploads_playlist_id: string; from_time: number; refresh_rate_ms: number; tags: string[] }
export type UndoHistory = { entries: ([number, Action])[] }
export type AddChannelOptions = { url: string; from_time: number; refresh_rate_ms: number; tags: string[] }
export type Action = "CheckNow" | { Archive: string } | { Unarchive: string } | { AddChannel: string } | { UpdateOrDeleteChannels: string }
