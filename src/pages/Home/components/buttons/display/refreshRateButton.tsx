import { invoke } from '@tauri-apps/api/tauri'
import { message } from '@tauri-apps/api/dialog'
import { useEffect, useState } from 'react'

import style from '../../../../../components/styles/Comps.module.css'

export default function ChangeRefreshRate() {
  const [clicking, setClicking] = useState(false)
  const [hz, setHz] = useState(0)

  async function getDisplayFrequencyData() {
    let refresh_rate: number = await invoke('get_refresh_rate')
    console.log(refresh_rate)
    setHz(refresh_rate)
  }

  useEffect(() => {
    getDisplayFrequencyData()
  }, [])

  const onClick = async () => {
    let success = (await invoke('set_refresh_rate'))
    if (!success) await message("YOU DONKEY", { title: "PwccaAuto", type: 'error', okLabel: 'Close' })

    await getDisplayFrequencyData()
  }

  return (
    <div
      className={`${style.data} group ${!clicking ? 'scale-100' : 'scale-90'} transition-all duration-75 info-bg pc:cursor-pointer pc:order-4`}
      onPointerDown={() => setClicking(true)}
      onPointerUp={() => setClicking(false)}
      onPointerLeave={() => setClicking(false)}
      onClick={onClick}
    >
      <span className='text-lg'>Refresh Rate</span>
      <span className={`${style.redirect} text-[#C8FB69]`}>{hz}Hz</span>
    </div>
  )
}