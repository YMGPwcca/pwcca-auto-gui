import { create } from 'zustand'

export interface TauriError {
  state: boolean,
  error?: {
    kind: string,
    message: string
  }
}

interface TauriErrorStore {
  store: TauriError
  setError: (error: TauriError['error']) => void
  clearError: () => void
}

export const useTauriErrorStore = create<TauriErrorStore>()(
  (set, get) => ({
    store: {
      state: false,
      error: {
        kind: '',
        message: ''
      }
    },
    setError: error => {
      set({ store: { state: true, error } })
    },
    clearError: () => {
      set({ store: { state: false, error: get().store.error } })
    },
  }),
)