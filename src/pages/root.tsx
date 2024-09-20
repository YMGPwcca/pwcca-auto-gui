import { useEffect } from 'react'

import { useConfigStore } from '../data/config'

export default function Root({ children }: React.PropsWithChildren) {
  const configStore = useConfigStore()

  document.addEventListener('contextmenu', event => event.preventDefault())

  useEffect(() => {
    configStore.loadConfig()
  }, [])

  return (
    <div className='mobile:bg-tier0 [scrollbar-width:none] [-ms-overflow-style:none] select-none [-webkit-touch-callout:none] [-webkit-user-select:none] [-khtml-user-select:none] [-moz-user-select:none] [-ms-user-select:none] [-webkit-tap-highlight-color:transparent]'>
      {children}
    </div>
  )
}
