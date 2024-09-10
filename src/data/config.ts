import { create } from 'zustand'

export interface Config {
  startup: boolean

  microphone: {
    enabled: boolean
    includeApps: string[]
  }

  ethernet: boolean

  taskbar: {
    enabled: boolean
    includeApps: string[]
  }

  power: {
    enabled: boolean
    timer: number
    percentage: number
  }
}

interface ConfigStore {
  config: Config
  setConfig: (config: Config) => void
}

export const useConfigStore = create<ConfigStore>()(
  set => ({
    config: {
      startup: false,
      microphone: {
        enabled: false,
        includeApps: [],
      },
      ethernet: false,
      taskbar: {
        enabled: false,
        includeApps: [],
      },
      power: {
        enabled: false,
        timer: 300,
        percentage: 60,
      },
    },

    setConfig: config => set({ config })
  }),
)