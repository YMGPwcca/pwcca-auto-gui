import { useRef, useState } from 'react'
import SVGPlus from '../../components/svg/SVGPlus'
import SettingLayout from './layout'
import SVGTrash from '../../components/svg/SVGTrash'

export default function Microphone() {
  const [list, setList] = useState([...Array(30).keys()].map(i => (i + 1).toString()))

  const inputRef = useRef<HTMLInputElement>(null)
  const itemRef = useRef<HTMLDivElement>(null)

  const addList = () => {
    if (inputRef.current!.value === '') return
    if (list.includes(inputRef.current!.value)) return

    setList([...list, inputRef.current!.value])
  }

  const onClick = () => {
    setTimeout(() => inputRef.current?.scrollIntoView({ behavior: 'smooth' }), 50)
  }

  const onKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    inputRef.current?.scrollIntoView({ behavior: 'instant' })
    if (e.key === 'Enter') addList()
  }

  return (
    <SettingLayout>
      <span className='mx-auto font-bold text-xl'>Program List</span>

      <div className='h-full w-full gap-3 flex flex-col relative'>
        {/* Application list */}
        <div className='flex flex-col bg-tier2 w-72 h-px500 mx-auto rounded-lg'>
          <div className='inline-flex flex-col w-full h-full overflow-auto align-top'>
            {list.map(item => (
              <div className='flex flex-row text-lg my-1 mx-2 border-b-tier4 [&:not(:last-child)]:border-b-2'>
                <span className='mx-1 overflow-auto w-9/12' ref={itemRef}>{item}</span>
                <div className='flex-grow'></div> {/* Little trick */}
                <SVGTrash className='mx-1 w-5 h-5 my-auto cursor-pointer' />
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
              onClick={onClick}
              onKeyDown={onKeyDown}
            />
          </div>
          <div
            className='cursor-pointer bg-tier2 rounded-lg h-full w-[54px] flex mx-auto'
            onClick={addList}
          >
            <SVGPlus className='w-5 h-5 m-auto' />
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