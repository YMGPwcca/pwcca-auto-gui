import { invoke } from '@/invokeTauri'
import { useEffect, useState } from 'react'

import { useConfigStore } from '@/data/config'

import SettingLayout from '@/pages/Settings/layout'

interface Startup {
  group: 'User' | 'System',
  path: string,
  name: string,
  state: boolean,
}

export default function AutoStart() {
  const configStore = useConfigStore()

  const [list, setList] = useState(configStore.config.autostart.apps)
  const [appList, setAppList] = useState<Startup[]>([])

  useEffect(() => {
    invoke<Startup[]>('get_autostart_apps').then(res => {
      let list: Startup[] = []

      for (const item of res) {
        if (configStore.config.autostart.apps.includes(item.name))
          list.push({
            ...item,
            state: false
          })

        if ((item.state && !list.some(i => i.name === item.name)))
          list.push(item)
      }

      setAppList(list)
    })
  }, [])

  const onClick = async (e: React.MouseEvent<HTMLInputElement>) => {
    setList(prev => {
      if (e.target instanceof HTMLInputElement) {
        const name = e.target.parentElement!.parentElement!.children.item(0)!.textContent!
        if (!e.target.checked && !prev.includes(name))
          prev.push(name)
        if (e.target.checked && prev.includes(name))
          prev.splice(prev.indexOf(name), 1)
      }

      return prev
    })
  }

  const saveConfig = async () => {
    configStore.config.autostart.apps = list
    await configStore.saveConfig()
  }

  return (
    <SettingLayout>
      {/* Application list */}
      <span className='text-center text-xl font-bold mt-2'>Disallow list</span>
      <div className='flex flex-col bg-tier2 w-72 h-px400 mx-auto rounded-lg'>
        <div className='flex flex-col w-full h-full overflow-auto align-top'>
          {
            appList.map(item => (
              <div key={item.name} className='flex flex-row text-lg my-1 mx-2 border-b-tier4 pb-2 [&:not(:last-child)]:border-b-2'>
                <span className='mx-1 truncate w-8/12'>{item.name}</span>
                <div className='flex-grow'></div>
                <hr className='w-0.5 h-5 border-0 bg-tier4 my-auto mr-2'></hr>
                <label className='flex items-center cursor-pointer'>
                  <input type='checkbox' className='sr-only peer' defaultChecked={item.state} onClick={onClick}></input>
                  <div className='relative w-9 h-5 bg-tier4 rounded-full peer peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:absolute after:top-[2px] after:start-[2px] after:bg-white after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:bg-blue-700'></div>
                </label>
              </div>
            ))
          }
        </div>
      </div>

      {/* Save */}
      <div className='mx-auto bg-tier3 hover:bg-blue-700 w-24 h-px40 flex rounded-xl cursor-pointer mt-3' onClick={saveConfig}>
        <span className='m-auto text-lg font-medium'>Save</span>
      </div>

      {/* Notice */}
      <div className='flex flex-col absolute bottom-0 left-1/2 -translate-x-1/2 text-center mb-2 w-4/5'>
        <span className='text-sm text-center'>Turn OFF to disallow program from starting.</span>
        <div className="w-full truncate m-auto">
          <div className="inline-block w-full hover:w-auto active:w-auto">
            <div className="relative left-0 truncate text-sm hover:left-[calc(51%-100%)] hover:transition-[left] hover:duration-[5000ms] hover:ease-linear">
              Manually disabled items in Task Manager "Startup Apps" will not be displayed in this list.
            </div>
          </div>
        </div>
      </div>
    </SettingLayout>
  )
}