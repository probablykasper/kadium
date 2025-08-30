// utils.ts
import { commands } from '../../bindings'

export default new Proxy({} as typeof commands, {
	get:
		(_, property: string) =>
		async (...args: unknown[]) => {
			try {
				// eslint-disable-next-line @typescript-eslint/no-explicit-any
				const result = await (commands as any)[property](...args)
				if (result && 'status' in result && result.status === 'error') {
					throw new Error(result.error)
				}

				return result
			} catch (e) {
				commands.errorPopup(String(e))
				throw e
			}
		},
})
