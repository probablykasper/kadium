// utils.ts
import * as c from '../../bindings'

export default new Proxy({} as typeof c, {
  get:
    (_, property: string) =>
    async (...args: unknown[]) => {
      try {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        return await (c as any)[property](...args)
      } catch (e) {
        c.errorPopup(String(e))
        throw e
      }
    },
})
