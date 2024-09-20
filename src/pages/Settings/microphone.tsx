import { useEffect, useRef, useState } from 'react'

import { useConfigStore } from '../../data/config'

import SVGPlus from '../../components/svg/SVGPlus'
import SettingLayout from './layout'
import SVGTrash from '../../components/svg/SVGTrash'
import SVGSad from '../../components/svg/SVGSad'

export default function Microphone() {
  const configStore = useConfigStore()

  const [list, setList] = useState(configStore.config.microphone.apps)
  const [inputApp, setInputApp] = useState('')

  const inputRef = useRef<HTMLInputElement>(null)
  const itemRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    configStore.config.microphone.apps = list
    configStore.saveConfig()
  }, [list])

  const modifyList = () => {
    if (inputApp === '') return

    let input = inputApp.toLowerCase()

    if (!input.slice(-4).includes('.exe')) input += '.exe'
    if (list.includes(input)) return

    setList([...list, input])
  }

  const inputClick = () => {
    setTimeout(() => inputRef.current?.scrollIntoView({ behavior: 'smooth' }), 50)
  }

  const inputKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    inputRef.current?.scrollIntoView({ behavior: 'instant' })
    if (e.key === 'Enter') modifyList()
  }

  return (
    <SettingLayout>
      {/* Application list */}
      <span className='text-center text-xl font-bold mt-2'>Program list</span>
      <div className='flex flex-col bg-tier2 w-72 h-px400 mx-auto rounded-lg'>
        <div className='inline-flex flex-col w-full h-full overflow-auto align-top'>
          {
            list.length > 0
              ? list.map(item => (
                <div key={item} className='flex flex-row text-lg my-1 mx-2 border-b-tier4 [&:not(:last-child)]:border-b-2'>
                  <span className='mx-1 overflow-auto w-9/12' ref={itemRef}>{item}</span>
                  <div className='flex-grow'></div>
                  <hr className='w-0.5 h-5 border-0 bg-tier4 my-auto mr-1'></hr>
                  <SVGTrash className='mx-1 w-5 h-5 my-auto cursor-pointer' onClick={() => setList(list.filter(i => i !== item))} />
                </div>
              ))
              : (
                <div className='m-auto flex flex-col'>
                  <SVGSad className='mb-3 w-14 h-14 m-auto'/>
                  <span className='text-lg font-bold'>This list is empty</span>
                </div>
              )
          }
        </div>
      </div>

      {/* Button list */}
      <div className='w-72 h-[54px] flex mx-auto gap-2'>
        <div className='bg-tier2 rounded-lg h-full w-[226px] flex mx-auto'>
          <input
            className='w-4/5 m-auto bg-transparent outline-none border-b-2 border-b-tier4 text-center'
            ref={inputRef}
            type='text'
            placeholder='Add more programs'
            autoComplete='off'
            autoCorrect='off'
            onClick={inputClick}
            onKeyDown={inputKeyDown}
            onChange={e => setInputApp(e.target.value)}
          />
        </div>
        <div
          className='group relative cursor-pointer bg-tier2 rounded-lg h-full w-[54px] flex mx-auto'
          onClick={modifyList}
        >
          <div className='m-auto flex' onClick={() => inputApp.length > 0 ? modifyList() : setList([])}>
            <div className='group-hover:flex hidden w-20 h-9 absolute -top-10 left-1/2 -translate-x-1/2 bg-tier3 rounded-lg text-center'>
              <span className='m-auto'>
                {
                  inputApp.length > 0
                    ? 'Add'
                    : 'Clear all'
                }
              </span>
            </div>
            {
              inputApp.length > 0
                ? <SVGPlus className='w-4 h-4 m-auto' />
                : <SVGTrash className='w-5 h-5 m-auto' />
            }
          </div>
        </div>
      </div>

      {/* Notice */}
      <div className='flex flex-col w-full mt-2'>
        <span className='text-sm text-center'>Use "Name" in Task Manager "Details" tab</span>
        <span className='text-sm text-center'>Eg: discord.exe, msedge.exe, code.exe</span>
      </div>
    </SettingLayout>
  )
}