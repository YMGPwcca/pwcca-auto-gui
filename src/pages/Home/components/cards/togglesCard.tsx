import FrameCard from '@/components/card/frameCard'
import ItemCard from '@/components/card/itemCard'

import ToggleButton from '@/pages/Home/components/buttons/toggle'

export default function TogglesFrameCard() {
  return (
    <FrameCard name={'Toggles'}>
      <ItemCard>
        <ToggleButton name='Refresh Rate' get='' set='' />
        <ToggleButton name='Startup' get='get_run_with_windows' set='set_run_with_windows' />
        <ToggleButton name='Ethernet' get='' set='' />
        <ToggleButton name='Monitor' get='' set='' />
      </ItemCard>
    </FrameCard>
  )
}