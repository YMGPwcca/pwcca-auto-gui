import { useState } from 'react'

import style from '../../../../components/styles/Comps.module.css'

export default function CredsButton({ show }: { show: Function }) {
  const [clicking, setClicking] = useState(false)

  return (
    <>
      <div
        className={`${style.data} group ${!clicking ? 'scale-100' : 'scale-90'} transition-all duration-75 info-bg pc:cursor-pointer pc:order-2`}
        onPointerDown={() => setClicking(true)}
        onPointerUp={() => setClicking(false)}
        onPointerLeave={() => setClicking(false)}
        onClick={() => show()}
      >
        <span className='text-lg'>Credentials</span>
        <span className={`${style.redirect} text-[#7369FB] from-[#7369FB] to-[#7369FB] pc:group-hover:bg-[length:100%_100%] pc:group-hover:text-[#2d2d2d]`}>Edit</span>
      </div>
    </>
  )
}