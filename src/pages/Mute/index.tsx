import { useEffect, useRef, useState } from 'react'
import { Event, listen } from '@tauri-apps/api/event'
import { getCurrentWindow } from '@tauri-apps/api/window'

import SVGMute from '@/components/svg/SVGMute'
import SVGUnmute from '@/components/svg/SVGUnmute'
import SVGComputer from '@/components/svg/SVGComputer'

interface Payload {
  type: 'system' | 'program'
  mute: boolean,
  icon: []
}

export default function Mute() {
  let [program, setProgram] = useState<Payload>({ type: 'system', mute: false, icon: [] })

  listen('program', (e: Event<Payload>) => setProgram({ ...e.payload, type: 'program' }))
  listen('system', (e: Event<Payload>) => setProgram({ ...e.payload, type: 'system' }))

  const timer = useRef<Timer>()
  const icon = useRef<HTMLCanvasElement>(null)

  useEffect(() => {
    clearTimeout(timer.current)
    let currentWindow = getCurrentWindow()
    currentWindow.show()

    if (program.type === 'program' && program.icon.length > 0) {
      let ctx = icon.current!.getContext('2d')!
      let size = Math.sqrt(program.icon.length)
      let imgData = ctx.createImageData(size, size)
      for (let i = 0; i < program.icon.length; i++) {
        imgData.data[i * 4 + 0] = program.icon[i][0]
        imgData.data[i * 4 + 1] = program.icon[i][1]
        imgData.data[i * 4 + 2] = program.icon[i][2]
        imgData.data[i * 4 + 3] = program.icon[i][3]
      }
      ctx.putImageData(imgData, 0, 0)
    }

    timer.current = setTimeout(() => currentWindow.hide(), 2000)
  }, [program])

  return (
    <div className='w-[140px] h-[80px] overscroll-none [scrollbar-width:none] [-ms-overflow-style:none] select-none [-webkit-touch-callout:none] [-webkit-user-select:none] [-khtml-user-select:none] [-moz-user-select:none] [-ms-user-select:none] [-webkit-tap-highlight-color:transparent]'>
      <div className='pc:rounded-xl flex flex-col h-dvh min-h-dvh w-dvw text-white overflow-auto [scrollbar-width:none] [-ms-overflow-style:none] relative'>
        <div className='m-auto grid grid-cols-2 w-full h-full bg-zinc-600/50'>
          {
            program.type === 'program'
              ? <canvas ref={icon} width={48} height={48} className='m-auto rounded-lg'></canvas>
              : <SVGComputer className='w-12 h-12 m-auto' />
          }
          <div className='flex w-12 h-12 m-auto'>
            {program.mute ? <SVGMute className='w-12 h-12 m-auto' /> : <SVGUnmute className='w-12 h-12 m-auto' />}
          </div>
        </div>
      </div>
    </div>
  )
}
