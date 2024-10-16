import SVGMute from '@/components/svg/SVGMute'
import SVGUnmute from '@/components/svg/SVGUnmute'
import { listen } from '@tauri-apps/api/event'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { useEffect, useRef, useState } from 'react'

interface Payload {
  mute: boolean,
  rgba: []
}

export default function Mute() {
  let [program, setProgram] = useState<Payload>({ mute: false, rgba: [] })

  listen('program_name', e => setProgram(e.payload as Payload))

  const timer = useRef<Timer>()
  const icon = useRef<HTMLCanvasElement>(null)

  useEffect(() => {
    if (program.rgba.length > 0) {
      clearTimeout(timer.current)
      let currentWindow = getCurrentWindow()
      currentWindow.show()

      let ctx = icon.current!.getContext('2d')!
      let size = Math.sqrt(program.rgba.length)
      let imgData = ctx.createImageData(size, size)
      for (let i = 0; i < program.rgba.length; i++) {
        imgData.data[i * 4 + 0] = program.rgba[i][0]
        imgData.data[i * 4 + 1] = program.rgba[i][1]
        imgData.data[i * 4 + 2] = program.rgba[i][2]
        imgData.data[i * 4 + 3] = program.rgba[i][3]
      }
      ctx.putImageData(imgData, 0, 0)

      timer.current = setTimeout(() => currentWindow.hide(), 2000)
    }
  }, [program])

  return (
    <div className='w-[140px] h-[80px] overscroll-none [scrollbar-width:none] [-ms-overflow-style:none] select-none [-webkit-touch-callout:none] [-webkit-user-select:none] [-khtml-user-select:none] [-moz-user-select:none] [-ms-user-select:none] [-webkit-tap-highlight-color:transparent]'>
      <div className='pc:rounded-xl flex flex-col bg-gray-500 h-dvh min-h-dvh w-dvw text-white overflow-auto [scrollbar-width:none] [-ms-overflow-style:none] relative'>
        <div className='m-auto grid grid-cols-2 w-full h-full'>
          <canvas ref={icon} width={48} height={48} className='m-auto rounded-lg'></canvas>
          <div className='flex w-12 h-12 m-auto'>
            {program.mute ? <SVGMute /> : <SVGUnmute />}
          </div>
        </div>
      </div>
    </div>
  )
}
