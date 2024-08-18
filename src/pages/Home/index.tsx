import TogglesFrameCard from './components/cards/togglesCard'
import SettingsFrameCard from './components/cards/settingsCard'

export default function Home() {
  document.addEventListener('contextmenu', event => event.preventDefault())

  return (
    <div className='content-container no-scrollbar rounded-xl border-2 border-gray-600 py-5'>
      {/* INIT FRAME */}
      <div>
        <div className='flex flex-col gap-8'>

          {/* MAIN CARDS */}
          <TogglesFrameCard />
          <SettingsFrameCard />

        </div>
      </div>

    </div>
  )
}
