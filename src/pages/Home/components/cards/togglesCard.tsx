import FrameCard from '../../../../components/cards/frameCard'
import ItemCard from '../../../../components/cards/itemCard'

import ToggleButton from '../buttons/toggle'

export default function TogglesFrameCard() {
  return (
    <FrameCard name={'Toggles'}>
      <ItemCard>
        <ToggleButton name='Refresh Rate' get='get_refresh_rate' set='set_refresh_rate' />
        <ToggleButton name='Startup' get='get_run_with_windows' set='set_run_with_windows' />
        <ToggleButton name='Ethernet' get='get_ethernet_state' set='set_ethernet_state' />
        <ToggleButton name='Taskbar' get='get_taskbar_state' set='set_taskbar_state' />
      </ItemCard>
    </FrameCard>
  )
}