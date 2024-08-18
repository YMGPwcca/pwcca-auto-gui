import { invoke } from '@tauri-apps/api/tauri'
import { useEffect, useState } from 'react'

export default function RunWithWindows() {
  const [clicking, setClicking] = useState(false)
  const [state, setState] = useState(false)

  function getRunWithWindowsData() {
    invoke('get_run_with_windows').then(state => setState(state as boolean))
  }

  useEffect(() => getRunWithWindowsData(), [])

  const onClick = async () => {
    await invoke('set_run_with_windows')
    getRunWithWindowsData()
  }

  return (
    <div
      className={`cursor-pointer flex flex-col text-center justify-center rounded-lg info-bg group ${!clicking ? 'scale-100' : 'scale-90'} transition-all duration-75 info-bg`}
      onPointerDown={() => setClicking(true)}
      onPointerUp={() => setClicking(false)}
      onPointerLeave={() => setClicking(false)}
      onClick={onClick}
    >
      <span className='text-lg'>Startup</span>
      <div className={`${state ? 'bg-[#69E5FB]' : 'bg-[#FB6969]'} w-8 h-3 mt-1 rounded-lg mx-auto`}></div>
    </div>
  )
}