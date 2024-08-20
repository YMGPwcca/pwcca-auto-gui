import TogglesFrameCard from './components/cards/togglesCard'
import SettingsFrameCard from './components/cards/settingsCard'

export default function Home() {
  document.addEventListener('contextmenu', event => event.preventDefault())

  return (
    <div className='flex flex-col bg-tier0 h-screen min-h-screen w-screen text-white overflow-auto [scrollbar-width:none] [-ms-overflow-style:none]'>
      {/* INIT FRAME */}
      <div className='flex m-auto bg-tier0 rounded-xl border-2 border-gray-600 py-4 w-[350px] h-[750px]'>

        <div className='m-auto flex flex-col gap-5'>
          {/* MAIN CARDS */}
          <TogglesFrameCard />
          <SettingsFrameCard />
        </div>

      </div>

    </div>
  )
}
