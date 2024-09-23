import { useEffect, useRef, useState } from 'react'
import { useLocation, useNavigate } from 'react-router-dom'

import { useConfigStore } from '../../data/config'
import { useSettingAnimationStore } from '../../data/settingAnimation'

import SVGBackArrow from '../../components/svg/SVGBackArrow'

export default function SettingLayout({ children }: React.PropsWithChildren) {
  const configStore = useConfigStore()
  const settingAnimationStore = useSettingAnimationStore()

  const navigate = useNavigate()
  const location = useLocation()

  const title = location.pathname.split('/')[2]

  const [enabled, setEnabled] = useState(configStore.config[title.toLowerCase()].enabled as boolean)

  const ref = useRef<HTMLDivElement>(null)

  useEffect(() => {
    setTimeout(() => {
      ref.current?.classList.add('hidden')
      settingAnimationStore.setState(false)
    }, 290 /* animation length - 10ms */)
  }, [])

  const toggleButton = async () => {
    setEnabled(prev => !prev)
    configStore.config[title.toLowerCase()].enabled = !configStore.config[title.toLowerCase()].enabled
    await configStore.saveConfig()
  }

  return (
    <div className='pc:rounded-xl flex flex-col bg-tier0 h-dvh min-h-dvh w-dvw text-tier0 overflow-auto [scrollbar-width:none] [-ms-overflow-style:none] relative'>

      {/* ANIMATION FRAME */}
      <div ref={ref} className={`${!settingAnimationStore.state ? 'hidden' : 'block'} absolute bg-tier2 h-dvh w-dvw top-0 left-0 pc:rounded-xl animate-fade-out z-50`}></div>

      {/* INIT FRAME */}
      <div className='m-auto bg-tier0 py-4 w-[350px] h-[750px] pc:border-2 pc:rounded-xl pc:border-tier3 mobile:w-full mobile:h-full'>

        <div className='flex flex-col m-auto w-full h-full'>
          <div className='flex flex-row text-center mx-auto relative w-full'>
            <SVGBackArrow className='w-6 h-6 absolute left-1 top-1/2 -translate-y-1/2 cursor-pointer' onClick={() => navigate(-1)} />
            <div className='font-bold text-2xl text-tier0 text-center m-auto'>{location.pathname.split('/')[2]}</div>
          </div>

          <div className='m-auto mt-5 flex flex-col w-full h-full'>
            <div className='h-full w-full gap-3 flex flex-col relative mt-0.5'>
              {/* Enable toggle */}
              <div className='w-72 h-14 flex flex-row bg-tier2 rounded-lg mx-auto px-2'>
                <span className='my-auto text-lg font-bold'>Enable</span>
                <div className='flex-grow'></div>
                <hr className='w-0.5 h-10 border-0 bg-tier4 my-auto mr-2'></hr>
                <label className='inline-flex items-center cursor-pointer'>
                  <input type='checkbox' className='sr-only peer' onClick={toggleButton} defaultChecked={enabled}></input>
                  <div className='relative w-11 h-6 bg-tier4 rounded-full peer peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:absolute after:top-[2px] after:start-[2px] after:bg-white after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-700'></div>
                </label>
              </div>

              {/* Main function */}
              {children}
            </div>
          </div>
        </div>

      </div>

    </div>
  )
}