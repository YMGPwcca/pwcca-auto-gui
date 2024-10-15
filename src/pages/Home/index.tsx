import { invoke } from '@/invokeTauri'
import { useState } from 'react'

import TogglesFrameCard from '@/pages/Home/components/cards/togglesCard'
import SettingsFrameCard from '@/pages/Home/components/cards/settingsCard'
import SVGQuit from '@/components/svg/SVGQuit'
import Root from '../root'

export default function Home() {
  const [quit, setQuit] = useState(false)
  const [show, setShow] = useState(false)

  document.addEventListener('contextmenu', event => event.preventDefault())

  const quitButton = async () => {
    if (!quit) {
      setQuit(true)
      setTimeout(async () => setShow(true), 500)
    }
    else {
      setQuit(false)
      setShow(false)
    }
  }

  return (
    <Root>
      {/* INIT FRAME */}
      <div className='relative flex m-auto bg-tier0 py-4 pc:border-2 pc:rounded-xl pc:border-tier3 w-full h-full'>
        <SVGQuit className='top-1 right-1 absolute h-6 w-6 cursor-pointer' onClick={quitButton} />

        <div className='m-auto flex flex-col gap-5'>
          {/* MAIN CARDS */}
          <TogglesFrameCard />
          <SettingsFrameCard />
        </div>

        <div className={`${quit ? 'opacity-100 h-28 w-48' : 'opacity-0 h-0 w-0'} z-10 flex flex-col bg-tier2 shadow-lg backdrop-blur absolute top-1 right-1 pc:border-2 pc:rounded-xl pc:border-tier3 transition-all duration-500`}>
          <div className={`${!show ? 'hidden' : 'flex'} m-auto flex-col gap-2 transition-all`}>
            <span className='text-lg mx-auto font-medium'>Quit the program?</span>
            <div className='flex flex-row mx-auto gap-3'>
              <div className='cursor-pointer rounded-md w-10 h-7 flex bg-blue-600 text-white' onClick={() => invoke('exit_app')}><span className='m-auto'>Yes</span></div>
              <div className='cursor-pointer rounded-md w-10 h-7 flex bg-red-600 text-white' onClick={quitButton}><span className='m-auto'>No</span></div>
            </div>
          </div>
        </div>

      </div>
    </Root>
  )
}
