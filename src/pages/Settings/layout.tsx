import { useEffect, useRef } from 'react'
import { useLocation, useNavigate } from 'react-router-dom'

import SVGBackArrow from '../../components/svg/SVGBackArrow'

export default function SettingLayout({ children }: React.PropsWithChildren) {
  const navigate = useNavigate()
  const location = useLocation()
  const ref = useRef<HTMLDivElement>(null)

  useEffect(() => {
    setTimeout(() => {
      ref.current?.classList.add('hidden')
    }, 290 /* animation length - 10ms */)
  }, [])

  return (
    <div className='pc:rounded-xl flex flex-col bg-tier0 h-dvh min-h-dvh w-dvw text-tier0 overflow-auto'>

      {/* ANIMATION FRAME */}
      <div ref={ref} className='absolute bg-tier2 h-dvh w-dvw top-0 left-0 pc:rounded-xl animate-fade-out z-50'></div>

      {/* INIT FRAME */}
      <div className='m-auto bg-tier0 py-4 w-[350px] h-[750px] pc:border-2 pc:rounded-xl pc:border-tier3 mobile:w-full mobile:h-full'>

        <div className='flex flex-col m-auto w-full h-full'>
          <div className='flex flex-row text-center mx-auto relative w-full'>
            <SVGBackArrow className='w-6 h-6 absolute left-1 top-1/2 -translate-y-1/2 cursor-pointer' onClick={() => navigate(-1)} />
            <div className='font-bold text-2xl text-tier0 text-center m-auto'>{location.pathname.split('/')[2]} Settings</div>
          </div>

          <div className='m-auto mt-5 flex flex-col w-full h-full'>
            {children}
          </div>
        </div>

      </div>

    </div>
  )
}