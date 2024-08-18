import { invoke } from '@tauri-apps/api/tauri'
import { useEffect, useState } from 'react'

export default function EthernetButton() {
  const [clicking, setClicking] = useState(false)
  const [state, setState] = useState(false)

  function getEthernetData() {
    invoke('get_ethernet_state').then(state => setState(state as boolean))
  }

  useEffect(() => getEthernetData(), [])

  const onClick = async () => {
    await invoke('set_ethernet_state')
    getEthernetData()
  }

  return (
    <div
      className={`cursor-pointer flex flex-col text-center justify-center rounded-lg info-bg group ${!clicking ? 'scale-100' : 'scale-90'} transition-all duration-75 info-bg`}
      onPointerDown={() => setClicking(true)}
      onPointerUp={() => setClicking(false)}
      onPointerLeave={() => setClicking(false)}
      onClick={onClick}
    >
      <span className='text-lg'>Ethernet</span>
      <div className={`${state ? 'bg-[#69E5FB]' : 'bg-[#FB6969]'} w-8 h-3 mt-1 rounded-lg mx-auto`}></div>
    </div>
  )
}