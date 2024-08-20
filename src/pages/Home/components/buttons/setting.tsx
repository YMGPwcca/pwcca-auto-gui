import { invoke } from '@tauri-apps/api/tauri'
import { useEffect, useRef, useState } from 'react'
import { useNavigate } from 'react-router-dom'

export default function SettingButton({ name, get, set }: { name: string, get: string, set: string }) {
  const navigate = useNavigate()

  const [state, setState] = useState(false)
  const [clicking, setClicking] = useState(false)
  const [num, setNum] = useState(0)

  function getToggleData() {
    invoke(get).then(state => setState(state as boolean))
    setState(!Math.round(Math.random()))
  }

  useEffect(() => getToggleData(), [])
  useEffect(() => {
    if (num === 200) navigate('/setting/' + name)
  }, [navigate, num])

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
        if (prev < 200) return prev + 1

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
      className={`cursor-pointer flex flex-col text-center justify-center rounded-lg bg-tier2 h-full transition-all duration-100 ${!clicking ? 'scale-100' : 'scale-90'}`}
      onPointerDown={onPointerDown}
      onPointerUp={onPointerUp}
      onPointerLeave={onPointerLeave}
    >
      <span className='text-lg text-gray-400'>{num > 30 ? 'Navigating...' : name}</span>
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