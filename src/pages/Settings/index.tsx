import { useLocation } from 'react-router-dom'

export default function Settings() {
  const location = useLocation()

  document.addEventListener('contextmenu', event => event.preventDefault())

  return (
    <div className='flex flex-col bg-tier0 h-screen min-h-screen w-screen text-white overflow-auto'>
      {/* INIT FRAME */}
      <div className='flex flex-col m-auto bg-tier0 rounded-xl border-2 border-gray-600 py-4 w-[350px] h-[750px]'>

        <div className='font-bold text-2xl text-gray-400 mb-2 text-center mt-[19px]'>{location.pathname.split('/')[2]}</div>
        <div className='m-auto flex flex-col gap-5'>
          A
        </div>

      </div>

    </div>
  )
}