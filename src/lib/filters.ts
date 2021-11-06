import { writable } from 'svelte/store'

export const show_all = writable(false)
export const show_archived = writable(false)
export const channel_filter = writable('')
