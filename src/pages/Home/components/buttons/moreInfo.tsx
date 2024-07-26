import { useState } from 'react'
import { Link } from 'react-router-dom'

import style from '../../../../components/styles/Comps.module.css'

export default function MoreInfo(props: React.PropsWithChildren<{ name: string }>) {
  const [clicking, setClicking] = useState(false)

  return (
    <Link
      to={'/' + props.name}
      className={`${style.data} group ${!clicking ? 'scale-100' : 'scale-90'} pc:order-4 transition-all duration-75 info-bg pc:cursor-pointer`}
      onPointerDown={() => setClicking(true)}
      onPointerUp={() => setClicking(false)}
      onPointerLeave={() => setClicking(false)}
      onContextMenu={e => e.preventDefault()}
    >
      <span className='text-lg'>More</span>
      <span className='text-[#c8fb69]'>
        Open <span className={`${style.redirect} text-[#C8FB69] from-[#C8FB69] to-[#C8FB69] pc:group-hover:bg-[length:100%_100%] pc:group-hover:text-[#2d2d2d]`}>{props.name}</span> page
      </span>
    </Link>
  )
}