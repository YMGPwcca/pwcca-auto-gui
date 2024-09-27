import FrameCard from '@/components/card/frameCard'
import ItemCard from '@/components/card/itemCard'
import SettingButton from '@/pages/Home/components/buttons/setting'

export default function SettingsFrameCard() {
  return (
    <FrameCard name={'Settings'}>
      <ItemCard>
        <SettingButton name='Microphone' />
        <SettingButton name='Power' />
        <SettingButton name='AutoStart' />
        <SettingButton name='Taskbar' />
      </ItemCard>
    </FrameCard>
  )
}