import TogglesFrameCard from './components/cards/togglesCard'
import SettingsFrameCard from './components/cards/settingsCard'

export default function Home() {
  document.addEventListener('contextmenu', event => event.preventDefault())

  return (
    <div className='pc:rounded-xl flex flex-col bg-tier0 h-screen min-h-screen w-screen text-tier0 overflow-auto [scrollbar-width:none] [-ms-overflow-style:none]'>
      {/* INIT FRAME */}
      <div className='flex m-auto bg-tier0 py-4 w-[350px] h-[750px] pc:border-2 pc:rounded-xl pc:border-tier3 mobile:w-full mobile:h-full'>

        <div className='m-auto flex flex-col gap-5'>
          {/* MAIN CARDS */}
          <TogglesFrameCard />
          <SettingsFrameCard />
        </div>

      </div>

    </div>
  )
}
