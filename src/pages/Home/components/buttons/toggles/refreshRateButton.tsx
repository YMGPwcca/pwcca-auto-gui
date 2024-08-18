import { invoke } from '@tauri-apps/api/tauri'
import { useEffect, useState } from 'react'

export default function ChangeRefreshRate() {
  const [clicking, setClicking] = useState(false)
  const [hz, setHz] = useState(60)

  function getDisplayFrequencyData() {
    invoke('get_refresh_rate').then(hz => setHz(hz as number))
  }

  useEffect(() => getDisplayFrequencyData(), [])

  const onClick = async () => {
    await invoke('set_refresh_rate')
    getDisplayFrequencyData()
  }

  return (
    <div
      className={`cursor-pointer flex flex-col text-center justify-center rounded-lg info-bg group ${!clicking ? 'scale-100' : 'scale-90'} transition-all duration-75 info-bg`}
      onPointerDown={() => setClicking(true)}
      onPointerUp={() => setClicking(false)}
      onPointerLeave={() => setClicking(false)}
      onClick={onClick}
    >
      <span className='text-lg'>Refresh Rate</span>
      <div className={`${hz !== 60 ? 'bg-[#69E5FB]' : 'bg-[#FB6969]'} w-8 h-3 mt-1 rounded-lg mx-auto`}></div>
    </div>
  )
}