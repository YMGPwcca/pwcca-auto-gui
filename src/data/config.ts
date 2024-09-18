import { create } from 'zustand'
import { persist } from 'zustand/middleware'

export interface Config {
  startup: boolean

  microphone: {
    enabled: boolean
    include: string[]
  }

  ethernet: boolean

  taskbar: {
    enabled: boolean
    include: string[]
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
  persist(
    set => ({
      config: {
        startup: false,
        microphone: {
          enabled: false,
          include: [],
        },
        ethernet: false,
        taskbar: {
          enabled: false,
          include: [],
        },
        power: {
          enabled: false,
          timer: 300,
          percentage: 60,
        },
      },

      setConfig: config => {
        console.log(config)
        set({ config })
      }
    }),
    {
      name: 'config',

    }
  )
)