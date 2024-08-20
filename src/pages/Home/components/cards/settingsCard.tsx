import FrameCard from '../../../../components/cards/frameCard'
import ItemCard from '../../../../components/cards/itemCard'
import BlankButton from '../buttons/blank'
import SettingButton from '../buttons/setting'

export default function SettingsFrameCard() {
  return (
    <FrameCard name={'Settings'}>
      <ItemCard>
        <SettingButton name='Microphone' get='' set=''></SettingButton>
        <SettingButton name='Battery' get='' set=''></SettingButton>
        <SettingButton name='AutoStart' get='' set=''></SettingButton>
        <BlankButton />
      </ItemCard>
    </FrameCard>
  )
}