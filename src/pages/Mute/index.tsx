import { listen } from '@tauri-apps/api/event'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { useEffect, useState } from 'react'

interface Payload {
  name: string,
  mute: boolean,
}

export default function Mute() {
  const [program, setProgram] = useState<Payload>({ name: '', mute: false })

  listen('program_name', e => setProgram(e.payload as Payload))

  useEffect(() => {
    if (program.name.length > 0) {
      let currentWindow = getCurrentWindow()
      currentWindow.show()
      setTimeout(() => {
        currentWindow.hide()
        setProgram({ name: '', mute: false })
      }, 3000)
    }
  }, [program])

  return (
    <div className='w-[200px] h-[100px] overscroll-none [scrollbar-width:none] [-ms-overflow-style:none] select-none [-webkit-touch-callout:none] [-webkit-user-select:none] [-khtml-user-select:none] [-moz-user-select:none] [-ms-user-select:none] [-webkit-tap-highlight-color:transparent]'>
      <div className='pc:rounded-xl flex flex-col bg-gray-500 opacity-70 h-dvh min-h-dvh w-dvw text-white overflow-auto [scrollbar-width:none] [-ms-overflow-style:none] relative'>
        {program.name} is {program.mute ? 'mute' : 'unmute'}d
      </div>
    </div>
  )
}
