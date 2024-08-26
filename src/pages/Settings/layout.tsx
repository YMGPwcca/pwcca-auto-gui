import { useEffect, useRef } from 'react'
import { useLocation, useNavigate } from 'react-router-dom'

export default function SettingLayout() {
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
      <div ref={ref} className='absolute bg-tier2 h-dvh w-dvw top-0 left-0 pc:rounded-xl animate-fade-out'></div>

      {/* INIT FRAME */}
      <div className='m-auto bg-tier0 py-4 w-[350px] h-[750px] pc:border-2 pc:rounded-xl pc:border-tier3 mobile:w-full mobile:h-full'>

        <div className='flex flex-col m-auto'>
          <div className='flex flex-row text-center mx-auto'>
            <div className='font-bold text-2xl text-center' onClick={() => navigate(-1)}>BACK</div>
            <div className='font-bold text-2xl text-tier9 text-center'>{location.pathname.split('/')[2]}</div>
          </div>

          <div className='m-auto flex flex-col gap-5'>
            A
          </div>
        </div>

      </div>

    </div>
  )
}