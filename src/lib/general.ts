import { invoke } from '@tauri-apps/api/tauri'
import { Commands } from '../../bindings'
export * from '../../bindings'

export function popup(msg: string) {
  invoke('error_popup', { msg })
}

export async function runCmd<N extends Commands['name']>(
  cmd: N,
  input: Extract<Commands, { name: N }>['input']
) {
  try {
    type ThisCmd = Extract<Commands, { name: N }>
    return await invoke<ThisCmd['result']>(cmd, input || undefined)
  } catch (e) {
    popup(String(e))
    throw e
  }
}

type ShortcutOptions = {
  shift?: boolean
  alt?: boolean
  cmdOrCtrl?: boolean
}
const isMac = navigator.userAgent.indexOf('Mac') != -1

export function checkModifiers(e: KeyboardEvent | MouseEvent, options: ShortcutOptions) {
  const target = {
    shift: options.shift || false,
    alt: options.alt || false,
    ctrl: (!isMac && options.cmdOrCtrl) || false,
    meta: (isMac && options.cmdOrCtrl) || false,
  }

  const pressed = {
    shift: !!e.shiftKey,
    alt: !!e.altKey,
    ctrl: !!e.ctrlKey,
    meta: !!e.metaKey,
  }

  return (
    pressed.shift === target.shift &&
    pressed.alt === target.alt &&
    pressed.ctrl === target.ctrl &&
    pressed.meta === target.meta
  )
}

export function checkShortcut(e: KeyboardEvent, key: string, options: ShortcutOptions = {}) {
  if (e.key.toUpperCase() !== key.toUpperCase()) return false
  return checkModifiers(e, options)
}
