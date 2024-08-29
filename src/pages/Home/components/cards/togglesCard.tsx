import FrameCard from '../../../../components/card/frameCard'
import ItemCard from '../../../../components/card/itemCard'

import BlankButton from '../buttons/blank'
import ToggleButton from '../buttons/toggle'

export default function TogglesFrameCard() {
  return (
    <FrameCard name={'Toggles'}>
      <ItemCard>
        <ToggleButton name='Refresh Rate' get='' set='' />
        <ToggleButton name='Startup' get='' set='' />
        <ToggleButton name='Ethernet' get='' set='' />
        <BlankButton />
      </ItemCard>
    </FrameCard>
  )
}