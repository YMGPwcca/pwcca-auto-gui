import { create } from 'zustand'
import { persist } from 'zustand/middleware'

interface SettingAnimationStore {
  state: boolean
  setState: (state: boolean) => void
}

export const useSettingAnimationStore = create<SettingAnimationStore>()(
  persist(
    set => ({
      state: false,
      setState: state => {
        set({ state })
      },
    }),
    {
      name: 'animation_state',
    }
  )
)