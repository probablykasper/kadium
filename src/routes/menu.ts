import { goto } from '$app/navigation'
import { settingsOpen, viewOptions } from '$lib/data'
import { Menu, Submenu, type SubmenuOptions } from '@tauri-apps/api/menu'
import { openUrl } from '@tauri-apps/plugin-opener'
import { show_get_started } from './+layout.svelte'

export const menu_actions: Partial<
	Record<'Find' | 'Open' | 'Open Channel' | 'Archive' | 'Unarchive', () => void>
> = {}

export async function create_menu() {
	const app_menu: SubmenuOptions = {
		text: 'Kadium',
		items: [
			{
				item: {
					About: null,
				},
			},
			{ item: 'Separator' },
			{
				text: 'Preferences...',
				id: 'Preferences...',
				accelerator: 'cmdOrControl+,',
				action() {
					settingsOpen.set(true)
				},
			},
			{ item: 'Separator' },
			{ item: 'Services' },
			{ item: 'Separator' },
			{ item: 'Hide' },
			{ item: 'HideOthers' },
			{ item: 'ShowAll' },
			{ item: 'Separator' },
			{ item: 'Quit' },
		],
	}
	const file_menu: SubmenuOptions = {
		text: 'File',
		items: [
			{
				text: 'Add Channel...',
				accelerator: 'cmdOrControl+N',
				action() {
					goto('/channels?add', { replaceState: true })
				},
			},
			{
				text: 'Open',
				action: () => menu_actions.Open?.(),
			},
			{
				text: 'Open Channel',
				action: () => menu_actions['Open Channel']?.(),
			},
			{
				text: 'Archive',
				accelerator: 'CmdOrCtrl+Backspace',
				action: () => menu_actions.Archive?.(),
			},
			{
				text: 'Unarchive',
				accelerator: 'Shift+CmdOrCtrl+Backspace',
				action: () => menu_actions.Unarchive?.(),
			},
			{ item: 'Separator' },
			{
				text: 'Options...',
				id: 'Preferences...',
				accelerator: 'cmdOrControl+,',
				action() {
					settingsOpen.set(true)
				},
			},
			{ item: 'Separator' },
			{ item: 'CloseWindow' },
		],
	}
	const edit_menu: SubmenuOptions = {
		text: 'Edit',
		items: [
			{ item: 'Undo' },
			{ item: 'Redo' },
			{ item: 'Separator' },
			{ item: 'Cut' },
			{ item: 'Copy' },
			{ item: 'Paste' },
			// #[cfg(not(target_os = "macos"))]
			{ item: 'Separator' },
			{ item: 'SelectAll' },
			{ item: 'Separator' },
			{
				text: 'Find',
				accelerator: 'CmdOrCtrl+F',
				action() {
					menu_actions.Find?.()
				},
			},
		],
	}
	const view_menu: SubmenuOptions = {
		text: 'View',
		items: [
			{
				text: 'Show New',
				accelerator: 'Alt+CmdOrCtrl+N',
				action() {
					goto('/', { replaceState: true })
					viewOptions.update((v) => {
						v.show_all = false
						v.show_archived = false
						return v
					})
				},
			},
			{
				text: 'Show Archived',
				accelerator: 'Alt+CmdOrCtrl+E',
				action() {
					goto('/', { replaceState: true })
					viewOptions.update((v) => {
						v.show_all = false
						v.show_archived = true
						return v
					})
				},
			},
			{
				text: 'Show All',
				accelerator: 'Alt+CmdOrCtrl+A',
				action() {
					goto('/', { replaceState: true })
					viewOptions.update((v) => {
						v.show_all = true
						v.show_archived = false
						return v
					})
				},
			},
			{ item: 'Separator' },
			{
				text: 'History',
				accelerator: 'CmdOrCtrl+Y',
				action() {
					goto('/history', { replaceState: true })
				},
			},
			{ item: 'Separator' },
			{ item: 'Fullscreen' },
		],
	}
	const window_menu: SubmenuOptions = {
		text: 'Window',
		items: [
			//
			{ item: 'Minimize' },
			{ item: 'Maximize' },
			{ item: 'Separator' },
			{
				text: 'Videos',
				accelerator: 'Alt+CmdOrCtrl+1',
				action() {
					goto('/', { replaceState: true })
				},
			},
			{
				text: 'Channels',
				accelerator: 'Alt+CmdOrCtrl+2',
				action() {
					goto('/channels', { replaceState: true })
				},
			},
		],
	}
	const help_menu = await Submenu.new({
		text: 'Help',
		items: [
			{
				text: 'Get Started',
				action() {
					show_get_started.set(true)
				},
			},
			{
				text: 'Learn More',
				action() {
					openUrl('https://github.com/probablykasper/kadium')
				},
			},
		],
	})
	const menu = await Menu.new({
		items: [app_menu, file_menu, edit_menu, view_menu, window_menu, help_menu],
	})
	menu.setAsAppMenu()
	// https://github.com/tauri-apps/tauri/issues/12652
	help_menu.setAsHelpMenuForNSApp()
}
