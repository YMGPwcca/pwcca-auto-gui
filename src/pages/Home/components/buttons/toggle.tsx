import { invoke } from '@tauri-apps/api/tauri'
import { useEffect, useState } from 'react'

export default function ToggleButton({ name, get, set }: { name: string, get: string, set: string }) {
  const [clicking, setClicking] = useState(false)
  const [state, setState] = useState(false)

  function getToggleData() {
    invoke(get).then(state => setState(state as boolean))
  }

  useEffect(() => getToggleData(), [])

  const onClick = async () => {
    await invoke(set)
    getToggleData()
  }

  return (
    <div
      className={`cursor-pointer flex flex-col text-center justify-center rounded-lg bg-tier2 group ${!clicking ? 'scale-100' : 'scale-90'} transition-all duration-75 info-bg`}
      onPointerDown={() => setClicking(true)}
      onPointerUp={() => setClicking(false)}
      onPointerLeave={() => setClicking(false)}
      onClick={onClick}
    >
      <span className='text-lg'>{name}</span>
      <div className={`${state ? 'bg-[#69E5FB]' : 'bg-[#FB6969]'} w-8 h-3 mt-2 rounded-lg mx-auto`}></div>
    </div>
  )
}