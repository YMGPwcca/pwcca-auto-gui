import { useEffect } from 'react'

import { Config, useConfigStore } from '../data/config'
import { invoke } from '@tauri-apps/api'

export default function Root({ children }: React.PropsWithChildren) {
  const configStore = useConfigStore()

  document.addEventListener('contextmenu', event => event.preventDefault())

  useEffect(() => {
    invoke('get_config').then(config => configStore.setConfig(config as Config))
  }, [])

  return (
    <div className='mobile:bg-tier0 [scrollbar-width:none] [-ms-overflow-style:none] select-none [-webkit-touch-callout:none] [-webkit-user-select:none] [-khtml-user-select:none] [-moz-user-select:none] [-ms-user-select:none] [-webkit-tap-highlight-color:transparent]'>
      {children}
    </div>
  )
}
