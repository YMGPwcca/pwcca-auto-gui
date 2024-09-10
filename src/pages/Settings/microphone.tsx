import { useEffect, useRef, useState } from 'react'
import SVGPlus from '../../components/svg/SVGPlus'
import SettingLayout from './layout'
import SVGTrash from '../../components/svg/SVGTrash'
import { invoke } from '@tauri-apps/api'

export default function Microphone() {
  const [list, setList] = useState([] as string[])
  const [inputApp, setInputApp] = useState('')

  const inputRef = useRef<HTMLInputElement>(null)
  const itemRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    /* GET DATA FROM THE CONFIG LIST */


  }, [])

  const addList = () => {
    if (inputApp === '') return
    if (list.includes(inputApp)) return

    setList([...list, inputApp])
  }

  const inputClick = () => {
    setTimeout(() => inputRef.current?.scrollIntoView({ behavior: 'smooth' }), 50)
  }

  const inputKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    inputRef.current?.scrollIntoView({ behavior: 'instant' })
    if (e.key === 'Enter') addList()
  }

  const toggleButton = async () => {
    await invoke('toggle_microphone')
  }

  return (
    <SettingLayout>
      <div className='h-full w-full gap-3 flex flex-col relative mt-0.5'>
        {/* Enable toggle */}
        <div className='w-72 h-14 flex flex-row bg-tier2 rounded-lg mx-auto px-2'>
          <span className='my-auto text-lg font-bold'>Enable</span>
          <div className='flex-grow'></div>
          <hr className='w-0.5 h-10 border-0 bg-tier4 my-auto mr-2'></hr>
          <label className='inline-flex items-center cursor-pointer'>
            <input type='checkbox' className='sr-only peer' onClick={toggleButton}></input>
            <div className='relative w-11 h-6 bg-tier4 rounded-full peer peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:absolute after:top-[2px] after:start-[2px] after:bg-white after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-700'></div>
          </label>
        </div>

        {/* Application list */}
        <span className='text-center text-xl font-bold mt-2'>Program list</span>
        <div className='flex flex-col bg-tier2 w-72 h-px400 mx-auto rounded-lg'>
          <div className='inline-flex flex-col w-full h-full overflow-auto align-top'>
            {list.map(item => (
              <div className='flex flex-row text-lg my-1 mx-2 border-b-tier4 [&:not(:last-child)]:border-b-2'>
                <span className='mx-1 overflow-auto w-9/12' ref={itemRef}>{item}</span>
                <div className='flex-grow'></div> {/* Little trick */}
                <SVGTrash className='mx-1 w-5 h-5 my-auto cursor-pointer' onClick={() => setList(list.filter(i => i !== item))} />
              </div>
            ))}
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
            onClick={addList}
          >
            <div className='m-auto flex'>
              <div className='group-hover:flex hidden w-16 h-10 absolute -top-8 left-1/2 -translate-x-1/2 bg-tier3 rounded-lg text-center'>
                <span className='m-auto'>{
                  inputApp.length > 0
                    ? 'Add'
                    : 'Clear all'
                }</span>
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
      </div>
    </SettingLayout>
  )
}