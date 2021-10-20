import { invoke } from '@tauri-apps/api/tauri'

export function popup(msg: string) {
  invoke('error_popup', { msg })
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export async function runCmd<T = any>(cmd: string, options: { [key: string]: T } = {}) {
  return (await invoke(cmd, options).catch(popup)) as T
}
