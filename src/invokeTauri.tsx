import { invoke as tauriInvoke } from '@tauri-apps/api'
import { useTauriErrorStore } from '@/data/tauriInvoke'

export async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  return await tauriInvoke(cmd, args)
    .catch(error =>
      useTauriErrorStore.setState({
        store: {
          state: true,
          error: {
            kind: 'Command',
            command: cmd,
            message: error
          }
        }
      })
    ) as T
}