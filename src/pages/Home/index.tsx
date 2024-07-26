import Title from '../../components/Title'
import DisplayFrameCard from './components/cards/displayCard'
import QuickActionsFrameCard from './components/cards/powerCard'

export default function Home() {
  document.addEventListener('contextmenu', event => event.preventDefault())

  return (
    <div className='content-container no-scrollbar rounded-xl border-2 border-gray-600'>
      {/* TITLE FRAME */}
      <Title />

      {/* INIT FRAME */}
      <div className='pc:flex pc:justify-center pc:items-center pc:h-full mobile:mt-24'>
        <div className='pc:inline-grid pc:grid-cols-2 pc:grid-rows-2 pc:h-full pc:w-[80vw] mobile:flex mobile:flex-col'>

          {/* MAIN CARDS */}
          <DisplayFrameCard />
          <QuickActionsFrameCard />

        </div>
      </div>

    </div>
  )
}
