import FrameCard from '../../../../components/cards/frameCard'
import InfoCard from '../../../../components/cards/infoCard'

import EthernetButton from '../buttons/toggles/ethernetButton'
import ChangeRefreshRate from '../buttons/toggles/refreshRateButton'
import RunWithWindows from '../buttons/toggles/runWithWindows'
import TaskbarButton from '../buttons/toggles/taskbarButton'

export default function TogglesFrameCard() {
  return (
    <FrameCard name={'Toggles'}>
      <InfoCard>
        <ChangeRefreshRate />
        <RunWithWindows />
        <EthernetButton />
        <TaskbarButton />
      </InfoCard>
    </FrameCard>
  )
}