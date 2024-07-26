import { useState } from 'react'
import { Link } from 'react-router-dom'

import style from '../../../../components/styles/Comps.module.css'

export default function FilesButton() {
  const [clicking, setClicking] = useState(false)

  return (
    <>
      <Link
        to='/Files'
        className={`${style.data} group ${!clicking ? 'scale-100' : 'scale-90'} transition-all duration-75 info-bg pc:cursor-pointer pc:order-2`}
        onPointerDown={() => setClicking(true)}
        onPointerUp={() => setClicking(false)}
        onPointerLeave={() => setClicking(false)}
      >
        <span className='text-lg'>Files</span>
        <span className={`${style.redirect} text-[#FB69DB] from-[#FB69DB] to-[#FB69DB] pc:group-hover:bg-[length:100%_100%] pc:group-hover:text-[#2d2d2d]`}>WIP</span>
      </Link>
    </>
  )
}