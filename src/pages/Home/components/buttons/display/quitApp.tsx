import { useState } from 'react'

import style from '../../../../../components/styles/Comps.module.css'
import { appWindow } from '@tauri-apps/api/window'

export default function QuitApp() {
  const [clicking, setClicking] = useState(false)

  const onClick = async () => {
    await appWindow.close()
  }

  return (
    <div
      className={`${style.data} group ${!clicking ? 'scale-100' : 'scale-90'} transition-all duration-75 info-bg pc:cursor-pointer pc:order-4`}
      onPointerDown={() => setClicking(true)}
      onPointerUp={() => setClicking(false)}
      onPointerLeave={() => setClicking(false)}
      onClick={onClick}
    >
      <span className='text-lg'>Quit App</span>
      <span className={`${style.redirect} text-[#7369FB]`}>Quit the app</span>
    </div>
  )
}