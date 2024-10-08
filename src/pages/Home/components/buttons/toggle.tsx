import { useConfigStore } from '@/data/config'
import { invoke } from '@/invokeTauri'
import { useEffect, useState } from 'react'

export default function ToggleButton({ name, get, set }: { name: string, get?: string, set: string }) {
  const configStore = useConfigStore()

  const [clicking, setClicking] = useState(false)
  const [state, setState] = useState(true)

  const getToggleData = async () => {
    if (get) {
      if (!get.startsWith('config_')) setState(await invoke<boolean>(get!))
      else setState(configStore.config[get.slice(7)])
    }
  }

  useEffect(() => { getToggleData() }, [])

  const onClick = async () => {
    if (!set.startsWith('config_')) {
      await invoke(set)
      await getToggleData()
    }
    else {
      configStore.config[set.slice(7)] = !configStore.config[set.slice(7)]
      await configStore.saveConfig()
      await getToggleData()
    }
  }

  return (
    <div
      className={`cursor-pointer flex flex-col text-center justify-center rounded-lg bg-tier2 group ${!clicking ? 'scale-100' : 'scale-90'} transition-all duration-75 info-bg`}
      onPointerDown={() => setClicking(true)}
      onPointerUp={() => setClicking(false)}
      onPointerLeave={() => setClicking(false)}
      onClick={onClick}
    >
      <span className='text-lg font-medium'>{name}</span>
      <div className={`${state ? 'bg-[#69E5FB]' : 'bg-[#FB6969]'} w-8 h-3 mt-2 rounded-lg mx-auto`}></div>
    </div>
  )
}