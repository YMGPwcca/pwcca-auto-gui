import { useState } from 'react'
import { invoke } from '@tauri-apps/api'

import { useConfigStore } from '../../data/config'

import SettingLayout from './layout'

export default function Battery() {
  const configStore = useConfigStore()

  const [enabled, setEnabled] = useState(configStore.config.microphone.enabled)

  const toggleButton = async () => {
    setEnabled(prev => !prev)
    await invoke('toggle_microphone')
  }

  return (
    <SettingLayout>
      <div className='h-full w-full gap-3 flex flex-col relative mt-0.5'>
        {/* Enable toggle */}
        <div className='w-72 h-14 flex flex-row bg-tier2 rounded-lg mx-auto px-2'>
          <span className='my-auto text-lg font-bold'>Enable</span>
          <div className='flex-grow'></div>
          <hr className='w-0.5 h-10 border-0 bg-tier4 my-auto mr-2'></hr>
          <label className='inline-flex items-center cursor-pointer'>
            <input type='checkbox' className='sr-only peer' onClick={toggleButton} checked={enabled}></input>
            <div className='relative w-11 h-6 bg-tier4 rounded-full peer peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:absolute after:top-[2px] after:start-[2px] after:bg-white after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-700'></div>
          </label>
        </div>
      </div>
    </SettingLayout>
  )
}