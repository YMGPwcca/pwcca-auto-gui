import { useRef, useState } from 'react'

import style from '../../../../components/styles/Comps.module.css'

import SVGSpinner from '../../../../components/SVGSpinner'

export default function SpecialButton({ func }: { func: Function }) {
  const [clicking, setClicking] = useState(false)
  const [isDoing, setDoing] = useState(false)
  const [mode, setMode] = useState<0 | 1>(0)
  const [num, setNum] = useState(10)

  const up = useRef<Timer>()
  const down = useRef<Timer>()

  const handleOnPointerUp = () => {
    setClicking(false)
    clearTimeout(up.current)

    if (num < 40) return run(0)

    down.current = setInterval(() => setNum(prev => {
      if (prev > 10) return prev - 1

      clearTimeout(down.current)
      return prev
    }), 5)
  }

  const handleOnPointerDown = () => {
    setClicking(true)
    clearTimeout(down.current)

    up.current = setInterval(() => setNum(prev => {
      if (prev < 100) return prev + 1

      clearTimeout(up.current)
      run(1)
      return prev
    }), 5)
  }

  const handleCancel = (event: React.PointerEvent<HTMLDivElement>) => {
    if (event.pointerType === 'mouse' && event.buttons <= 0) return

    setClicking(false)
    clearTimeout(up.current)

    down.current = setInterval(() => setNum(prev => {
      if (prev > 10) return prev - 1

      clearTimeout(down.current)
      return prev
    }), 5)
  }

  const run = async (mode: 0 | 1) => {
    mode === 1 && await new Promise(res => setTimeout(res, 250))

    setClicking(false)
    func()
    setMode(mode)
    !isDoing && setDoing(true)
  }

  return (
    <>
      {
        !isDoing ?
          <div
            className={`${style.data} group ${!clicking ? 'scale-100' : 'scale-90'} transition-all duration-75 info-bg pc:cursor-pointer pc:order-1`}
            onPointerDown={handleOnPointerDown}
            onPointerUp={handleOnPointerUp}
            onPointerLeave={handleCancel}
          >
            <span className='text-lg'>Restart</span>
            <span className='text-[#69FBAD]'>
              Hold to <span style={{ backgroundSize: `100% ${num}%` }} className={`${style.redirect} from-[#69FBAD] to-[#69FBAD]`}>Shutdown</span>
            </span>
          </div> :
          <div className={`${style.data} ${!clicking ? 'scale-100' : 'scale-90'} transition-all duration-75 info-bg pc:order-1`}>
            <span className='text-lg'>{mode ? 'Shutting down' : 'Restarting'}</span>
            <SVGSpinner className='text-[#69FBAD] pt-3 mx-auto box-border overflow-visible w-6 h-6 mobile:w-5 mobile:h-5 mobile:pb-3' />
          </div>
      }
    </>
  )
}