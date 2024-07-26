import DataCard from '../../../../components/cards/dataCard'
import FrameCard from '../../../../components/cards/frameCard'
import InfoCard from '../../../../components/cards/infoCard'
import QuitApp from '../buttons/display/quitApp'

import ChangeRefreshRate from '../buttons/display/refreshRateButton'
import TurnOffDisplay from '../buttons/display/turnOffDisplay'

export default function DisplayFrameCard() {
  return (
    <FrameCard name={'Display'} className='pc:order-2'>
      <InfoCard>
        <DataCard title='Brightness'>
          <span className='text-lg'>Brightness</span>
          <span className='text-[#69FBAD]'>Loading...</span>
        </DataCard>
        <QuitApp />
        <TurnOffDisplay />
        <ChangeRefreshRate />
      </InfoCard>
    </FrameCard>
  )
}