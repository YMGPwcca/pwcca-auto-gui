import { useState, useRef } from 'react'

import style from '../../../../components/styles/Comps.module.css'

import FrameCard from '../../../../components/cards/frameCard'

export default function CredentialsFrameCard({ hide, data }: { hide: Function, data: any }) {
  const [clicking, setClicking] = useState(false)

  const emailRef = useRef<HTMLInputElement>(data.email)
  const passRef = useRef<HTMLInputElement>(data.password)
  const totpRef = useRef<HTMLInputElement>(data.totp)
  const dbRef = useRef<HTMLInputElement>(data.db)

  const saveButton = () => {
    hide()
  }

  return (
    <FrameCard name={'Credentials'} className='pc:order-4'>
      <div className={`${style.info} flex flex-col card-bg p-4`}>
        <div className={`flex text-center justify-center p-5 mobile:m-2 rounded-lg info-bg h-fit`}>
          <div className='text-center space-y-4 h-fit'>
            <input ref={emailRef} className='text-bg text-center rounded-lg p-1 w-full border-none outline-none' placeholder={data.email} type='email' />
            <input ref={passRef} className='text-bg text-center rounded-lg p-1 w-full border-none outline-none' placeholder='Facebook Password' type='password' />
            <input ref={totpRef} className='text-bg text-center rounded-lg p-1 w-full border-none outline-none' placeholder='Facebook TOTP Key' type='text' />
            <input ref={dbRef} className='text-bg text-center rounded-lg p-1 w-full border-none outline-none' placeholder='Dashboard Password' type='text' />
          </div>
        </div>
        <div className='flex-grow' />
        <div
          className={`${!clicking ? 'scale-100' : 'scale-90'} transition-all duration-75 info-bg py-2 px-4 rounded-lg mx-auto pc:cursor-pointer`}
          onPointerDown={() => setClicking(true)}
          onPointerUp={() => setClicking(false)}
          onPointerLeave={() => setClicking(false)}
          onClick={() => saveButton()}
        >
          <span className='text-lg text-[#7369FB]'>Save</span>
        </div>
      </div>
    </FrameCard>
  )
}