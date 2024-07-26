import { invoke } from '@tauri-apps/api/tauri'
import { useState } from 'react'

import style from '../../../../../components/styles/Comps.module.css'

export default function TurnOffDisplay() {
  const [clicking, setClicking] = useState(false)

  const onClick = async () => {
    await invoke('turn_off_screen')
  }

  return (
    <div
      className={`${style.data} group ${!clicking ? 'scale-100' : 'scale-90'} transition-all duration-75 info-bg pc:cursor-pointer pc:order-4`}
      onPointerDown={() => setClicking(true)}
      onPointerUp={() => setClicking(false)}
      onPointerLeave={() => setClicking(false)}
      onClick={onClick}
    >
      <span className='text-lg'>Turn off</span>
      <span className={`${style.redirect} text-[#FB69DB]`}>Turn off the display</span>
    </div>
  )
}