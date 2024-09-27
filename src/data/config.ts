import { invoke } from '@/invokeTauri'
import { create } from 'zustand'
import { persist } from 'zustand/middleware'

export interface Config {

  // Toggles
  startup: boolean,
  ethernet: boolean,

  // Configs
  microphone: {
    enabled: boolean,
    apps: string[]
  },
  power: {
    enabled: boolean,
    timer: number,
    percentage: number
  },
  autostart: {
    enabled: boolean,
    apps: string[]
  },
  taskbar: {
    enabled: boolean,
    apps: string[]
  },

  [T: string]: any
}

interface ConfigStore {
  config: Config
  setConfig: (config: Config) => void,
  loadConfig: () => Promise<void>,
  saveConfig: () => Promise<void>,
}

export const useConfigStore = create<ConfigStore>()(
  persist(
    (set, get) => ({
      config: {
        // Toggles
        startup: false,
        ethernet: false,

        // Configs
        microphone: {
          enabled: false,
          apps: [],
        },
        power: {
          enabled: false,
          timer: 300,
          percentage: 60,
        },
        autostart: {
          enabled: false,
          apps: [],
        },
        taskbar: {
          enabled: false,
          apps: [],
        },
      },

      setConfig: config => {
        set({ config })
      },

      loadConfig: async () => {
        set({ config: await invoke('get_config') })
      },

      saveConfig: async () => {
        await invoke('save_config', { config: JSON.stringify(get().config) })
        await get().loadConfig()
      },

    }),
    {
      name: 'config',
    }
  )
)