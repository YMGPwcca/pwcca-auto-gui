import { invoke } from '@tauri-apps/api/tauri'
import { useEffect, useRef, useState } from 'react'
import { useNavigate } from 'react-router-dom'

export default function SettingButton({ name, get, set }: { name: string, get: string, set: string }) {
  const navigate = useNavigate()

  const ref = useRef<HTMLDivElement>(null)

  const [state, setState] = useState(false)
  const [clicking, setClicking] = useState(false)
  const [num, setNum] = useState(0)
  const [scale, setScale] = useState(1)

  async function getToggleData() {
    try {
      let got = await invoke(get)
      if (typeof got === 'object') {
        console.log(got)
        setState((got as any).enabled)
      } else {
        setState(got as boolean)
      }
    }
    catch {
      setState(!Math.round(Math.random()))
    }
  }

  useEffect(() => { getToggleData() }, [])
  useEffect(() => {
    if (num === 150) {
      setScale(30)

      for (const item of ref.current?.children!)
        item.classList.add('hidden')

      setTimeout(() => navigate('/setting/' + name), 550 /* animation length - (30scale + 10ms) */)
    }
  }, [num])

  async function onClick() {
    await invoke(set)
    getToggleData()
  }

  const down = useRef<ReturnType<typeof setInterval>>()

  function onPointerDown() {
    setClicking(true)
    clearInterval(down.current)

    down.current = setInterval(() => {
      setNum(prev => {
        if (prev < 150) return prev + 1

        clearInterval(down.current)
        return prev
      })
    }, 5)
  }

  function onPointerUp() {
    setClicking(false)
    clearInterval(down.current)

    if (num < 30) onClick()

    down.current = setInterval(() => {
      setNum(prev => {
        if (prev > 0) return prev - 1

        clearInterval(down.current)
        return prev
      })
    }, 5)
  }

  function onPointerLeave(event: React.PointerEvent) {
    if (event.pointerType === 'mouse' && event.buttons <= 0) return

    setClicking(false)
    clearInterval(down.current)

    down.current = setInterval(() => {
      setNum(prev => {
        if (prev > 0) return prev - 1

        clearInterval(down.current)
        return prev
      })
    }, 5)
  }

  return (
    <div
      ref={ref}
      style={scale > 1 ? { transform: `scale(${scale})`, transitionDuration: '750ms', zIndex: 100 } : { cursor: 'pointer' }}
      className={`flex flex-col text-center justify-center rounded-lg bg-tier2 transition-all duration-75 ${!clicking ? 'scale-100' : 'scale-90'}`}
      onPointerDown={onPointerDown}
      onPointerUp={onPointerUp}
      onPointerLeave={onPointerLeave}
    >
      <span className='text-lg'>{num > 30 ? 'Navigating...' : name}</span>
      <div
        style={{
          backgroundSize: `${num < 30 ? 0 : num - 30}% 100%`,
          backgroundColor: state ? '#69E5FB' : '#FB6969',
        }}
        className='bg-left-bottom bg-gradient-to-r bg-no-repeat duration-[1ms] from-[#F8FB69] to-[#F8FB69] w-8 h-3 mt-2 rounded-lg mx-auto'
      ></div>
    </div>
  )
}