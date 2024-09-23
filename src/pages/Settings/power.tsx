import { useConfigStore } from '../../data/config'

import SettingLayout from './layout'
import Power_Timer from './Power/Timer'
import Power_Percentage from './Power/Percentage'

export default function Power() {
  const configStore = useConfigStore()

  const saveConfig = async () => {
    // Timer
    const hoursE = document.querySelector('#hours')!
    const minutesE = document.querySelector('#minutes')!
    const secondsE = document.querySelector('#seconds')!

    let timer = 0
    timer += +hoursE.children[1].textContent! * 3600
    timer += +minutesE.children[1].textContent! * 60
    timer += +secondsE.children[1].textContent!
    configStore.config.power.timer = timer

    // Percentage
    const percentageE = document.querySelector('#percentage')!
    configStore.config.power.percentage = +percentageE.textContent!

    await configStore.saveConfig()
  }

  return (
    <SettingLayout>
      {/* Timer Settings */}
      <Power_Timer />

      {/* Power Settings */}
      <Power_Percentage />

      {/* Save */}
      <div className='mx-auto bg-tier3 hover:bg-blue-700 w-24 h-px40 flex rounded-xl cursor-pointer mt-4' onClick={saveConfig}>
        <span className='m-auto text-lg font-medium'>Save</span>
      </div>

      {/* Notice */}
      <div className='flex flex-col absolute bottom-0 left-1/2 -translate-x-1/2 w-full text-center mb-2'>
        <span className='text-sm text-center'>This is an OR statement,</span>
        <span className='text-sm text-center'>which happens first will be used</span>
      </div>
    </SettingLayout>
  )
}