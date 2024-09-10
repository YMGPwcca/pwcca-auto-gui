import TogglesFrameCard from './components/cards/togglesCard'
import SettingsFrameCard from './components/cards/settingsCard'
import { useEffect } from 'react'

import { Config, useConfigStore } from '../../data/config'
import { invoke } from '@tauri-apps/api'

export default function Home() {
  const configStore = useConfigStore()

  document.addEventListener('contextmenu', event => event.preventDefault())

  useEffect(() => {
    invoke('get_config').then(config => configStore.setConfig(config as Config))
  }, [])

  useEffect(() => console.log(configStore.config), [configStore.config])

  return (
    <div className='pc:rounded-xl flex flex-col bg-tier0 h-dvh w-dvw text-tier0 overflow-hidden [scrollbar-width:none] [-ms-overflow-style:none]'>
      {/* INIT FRAME */}
      <div className='flex m-auto bg-tier0 py-4 pc:w-[350px] pc:h-[750px] pc:border-2 pc:rounded-xl pc:border-tier3 mobile:w-full mobile:h-full'>

        <div className='m-auto flex flex-col gap-5'>
          {/* MAIN CARDS */}
          <TogglesFrameCard />
          <SettingsFrameCard />
        </div>

      </div>

    </div>
  )
}
