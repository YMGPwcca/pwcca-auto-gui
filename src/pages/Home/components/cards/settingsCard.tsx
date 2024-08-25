import FrameCard from '../../../../components/card/frameCard'
import ItemCard from '../../../../components/card/itemCard'
import SettingButton from '../buttons/setting'

export default function SettingsFrameCard() {
  return (
    <FrameCard name={'Settings'}>
      <ItemCard>
        <SettingButton name='Microphone' get='' set='' />
        <SettingButton name='Battery' get='' set='' />
        <SettingButton name='AutoStart' get='' set='' />
        <SettingButton name='Taskbar' get='' set='' />
      </ItemCard>
    </FrameCard>
  )
}