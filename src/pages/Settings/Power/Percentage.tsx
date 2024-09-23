import { useEffect } from 'react'
import { useConfigStore } from '../../../data/config'

export default function Power_Percentage() {
  const configStore = useConfigStore()

  useEffect(() => {
    document.querySelector('#percentage')!.textContent = configStore.config.power.percentage.toString().padStart(2, '0')
  }, [])

  const inputKeyDown = (event: React.KeyboardEvent<HTMLDivElement>) => {
    if (event.key === 'Enter') {
      event.preventDefault()
      if (document.activeElement instanceof HTMLElement) document.activeElement.blur()

      if (event.target instanceof HTMLElement) {
        if (+event.target.textContent! >= 0 && +event.target.textContent! <= 100)
          document.querySelector('#percentage')!.textContent = event.target.textContent!.padStart(2, '0')
        else
          event.target.textContent = configStore.config.power.percentage.toString().padStart(2, '0')
      }
    }

    if (!Number.isInteger(parseInt(event.key)) && event.key !== 'Backspace' && !event.key.includes('Arrow') && event.key !== 'Delete')
      event.preventDefault()
  } 

  return (
    <>
      <span className='text-center text-xl font-bold mt-2'>Percentage Settings</span>
      <div className='flex flex-col bg-tier2 w-72 h-[120px] mx-auto rounded-lg p-2 relative'>
        <div className='flex flex-col w-full mt-4'>
          <div className='m-auto flex flex-row'>
            <div id='percentage' className='w-12 border-b-2 border-tier5 outline-none text-center text-3xl font-medium' onKeyDown={inputKeyDown} contentEditable></div>
            <span className='my-auto text-3xl text-center font-bold ml-1'>%</span>
          </div>
          <span className='m-auto text-sm absolute bottom-0 left-1/2 -translate-x-1/2 w-full text-center mb-2'>Set to 00:00:00 to disable</span>
        </div>
      </div>
    </>
  )
} 