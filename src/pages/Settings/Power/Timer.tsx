import { useEffect, WheelEvent } from 'react'

import { useConfigStore } from '../../../data/config'

export default function Power_Timer() {
  const configStore = useConfigStore()

  const hour_values = Array.from(Array(24).keys()).map(i => i.toString().padStart(2, '0'))
  const other_values = Array.from(Array(60).keys()).map(i => i.toString().padStart(2, '0'))

  const setTimer = (type?: ('hours' | 'minutes' | 'seconds')[]) => {
    let timer = configStore.config.power.timer

    if (type?.includes('hours')) {
      const hoursE = document.querySelector('#hours')!
      const hours = (Math.floor(timer / 3600)).toString().padStart(2, '0')

      const hourIndex = hour_values.indexOf(hours)
      hoursE.children[0].textContent = hour_values.at(hourIndex - 1)!
      hoursE.children[1].textContent = hours
      hoursE.children[2].textContent = hour_values.at(hourIndex + 1)!
    }

    if (type?.includes('minutes')) {
      const minutesE = document.querySelector('#minutes')!
      const minutes = (Math.floor((timer % 3600) / 60)).toString().padStart(2, '0')

      const minuteIndex = other_values.indexOf(minutes)
      minutesE.children[0].textContent = other_values.at(minuteIndex - 1)!
      minutesE.children[1].textContent = minutes
      minutesE.children[2].textContent = other_values.at(minuteIndex + 1)!
    }

    if (type?.includes('seconds')) {
      const secondsE = document.querySelector('#seconds')!
      const seconds = (timer % 60).toString().padStart(2, '0')

      const secondIndex = other_values.indexOf(seconds)
      secondsE.children[0].textContent = other_values.at(secondIndex - 1)!
      secondsE.children[1].textContent = seconds
      secondsE.children[2].textContent = other_values.at(secondIndex + 1)!
    }
  }

  useEffect(() => setTimer(['hours', 'minutes', 'seconds']), [])

  const wheelHandler = (event: WheelEvent<HTMLDivElement>) => {
    if (event.target instanceof HTMLElement) {
      const target = event.target.parentElement!
      if (target.id) {
        let values: string[] = []

        if (target.id === 'hours') {
          const beforeIndex = hour_values.indexOf(target.children[0].textContent!)
          values = hour_values.slice(beforeIndex, hour_values.length).concat(hour_values.slice(0, beforeIndex))
        }
        if (target.id === 'minutes' || target.id === 'seconds') {
          const beforeIndex = other_values.indexOf(target.children[0].textContent!)
          values = other_values.slice(beforeIndex, other_values.length).concat(other_values.slice(0, beforeIndex))
        }

        if (event.deltaY > 0) values.push(values.shift()!)
        else values.unshift(values.pop()!)

        target.children[0].textContent = values[0]
        target.children[1].textContent = values[1]
        target.children[2].textContent = values[2]
      }
    }
  }

  const inputKeyDown = (event: React.KeyboardEvent<HTMLDivElement>) => {
    if (event.key === 'Enter') {
      event.preventDefault()
      if (document.activeElement instanceof HTMLElement) document.activeElement.blur()

      if (event.target instanceof HTMLElement) {
        const target = event.target.parentElement!
        const mainIndex = (target.id === 'hours' ? hour_values : other_values).indexOf(target.children[1].textContent!.padStart(2, '0'))

        if (mainIndex > -1) {
          target.children[0].textContent = other_values.at(mainIndex - 1)!
          target.children[1].textContent = other_values.at(mainIndex)!
          target.children[2].textContent = other_values.at(mainIndex + 1)!
        }
        else setTimer([target.id as ('hours' | 'minutes' | 'seconds')])
      }
    }

    if (!Number.isInteger(parseInt(event.key)) && event.key !== 'Backspace' && !event.key.includes('Arrow') && event.key !== 'Delete')
      event.preventDefault()
  }

  return (
    <>
      <span className='text-center text-xl font-bold mt-2'>Timer Settings</span>
      <div className='flex flex-col bg-tier2 w-72 h-[220px] mx-auto rounded-lg p-2 relative'>
        <div className='flex flex-col w-full h-full'>
          <div className='flex flex-row w-full'>
            <span className='my-auto text-lg text-center w-full font-bold'>Hours</span>
            <span className='grow'></span>
            <span className='my-auto text-lg text-center w-full font-bold'>Minutes</span>
            <span className='grow'></span>
            <span className='my-auto text-lg text-center w-full font-bold'>Seconds</span>
          </div>
          <div className='flex flex-row w-full h-fit mt-6'>
            <div id='hours' className='flex flex-col mx-auto h-full w-full text-center gap-4' onWheel={wheelHandler}>
              <span className='text-xl font-bold m-auto -mb-3 text-tier5'></span>
              <span className='text-3xl font-bold m-auto cursor-text outline-none' onKeyDown={inputKeyDown} contentEditable></span>
              <span className='text-xl font-bold m-auto -mt-3 text-tier5'></span>
            </div>
            <span className='my-auto text-center text-2xl font-bold w-2'>:</span>
            <div id='minutes' className='flex flex-col mx-auto h-full w-full text-center gap-4' onWheel={wheelHandler}>
              <span className='text-xl font-bold m-auto -mb-3 text-tier5'></span>
              <span className='text-3xl font-bold m-auto cursor-text outline-none' onKeyDown={inputKeyDown} contentEditable></span>

              <span className='text-xl font-bold m-auto -mt-3 text-tier5'></span>
            </div>
            <span className='my-auto text-center text-2xl font-bold w-2'>:</span>
            <div id='seconds' className='flex flex-col mx-auto h-full w-full text-center gap-4' onWheel={wheelHandler}>
              <span className='text-xl font-bold m-auto -mb-3 text-tier5'></span>
              <span className='text-3xl font-bold m-auto cursor-text outline-none' onKeyDown={inputKeyDown} contentEditable></span>
              <span className='text-xl font-bold m-auto -mt-3 text-tier5'></span>
            </div>
          </div>

          <span className='m-auto text-sm absolute bottom-0 left-1/2 -translate-x-1/2 w-full text-center mb-2'>Set to 00:00:00 to disable</span>
        </div>
      </div>
    </>
  )
}