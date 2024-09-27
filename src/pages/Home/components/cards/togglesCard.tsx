import FrameCard from '@/components/card/frameCard'
import ItemCard from '@/components/card/itemCard'

import ToggleButton from '@/pages/Home/components/buttons/toggle'

export default function TogglesFrameCard() {
  return (
    <FrameCard name={'Toggles'}>
      <ItemCard>
        <ToggleButton name='Refresh Rate' get='get_refresh_rate' set='set_refresh_rate' />
        <ToggleButton name='Startup' get='get_run_with_windows' set='set_run_with_windows' />
        <ToggleButton name='Ethernet' get='config_ethernet' set='config_ethernet' />
        <ToggleButton name='Monitor' set='turn_off_screen' />
      </ItemCard>
    </FrameCard>
  )
}