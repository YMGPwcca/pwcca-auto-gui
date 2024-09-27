import { useEffect, useRef } from 'react'

import { useConfigStore } from '@/data/config'
import { useTauriErrorStore } from '@/data/tauriInvoke'

export default function Root({ children }: React.PropsWithChildren) {
  const configStore = useConfigStore()
  const tauriErrorStore = useTauriErrorStore()

  const errorRef = useRef<HTMLDivElement>(null)

  document.addEventListener('contextmenu', event => event.preventDefault())

  useEffect(() => {
    configStore.loadConfig()
  }, [])

  return (
    <div className='mobile:bg-tier0 pc:w-[350px] pc:h-[750px] overscroll-none [scrollbar-width:none] [-ms-overflow-style:none] select-none [-webkit-touch-callout:none] [-webkit-user-select:none] [-khtml-user-select:none] [-moz-user-select:none] [-ms-user-select:none] [-webkit-tap-highlight-color:transparent]'>
      <div className='pc:rounded-xl flex flex-col bg-tier0 h-dvh min-h-dvh w-dvw text-tier0 overflow-auto [scrollbar-width:none] [-ms-overflow-style:none] relative'>
        {children}

        <div ref={errorRef} className={`${tauriErrorStore.store.state ? 'opacity-100 z-50' : 'opacity-0 -z-50'} top-0 left-0 w-full h-full absolute flex backdrop-blur-md transition-all duration-500 border-2 border-tier3 pc:rounded-xl`}>
          <div className='w-full h-full absolute top-0 left-0 z-10' onClick={() => tauriErrorStore.clearError()}></div>
          <div className='flex flex-col p-2 m-auto w-72 h-44 bg-tier3 rounded-xl border-2 border-tier4 transition-all z-20'>
            <span className='text-center text-2xl font-bold text-red-400'>Tauri Error</span>
            <span className='text-center text-lg mt-1'><span className='font-medium'>Type:</span> {tauriErrorStore.store.error?.kind}</span>
            <span className='text-center truncate text-ellipsis text-balance h-full'><span className='font-medium'>Message:</span> {tauriErrorStore.store.error?.message}</span>
          </div>
        </div>
      </div>
    </div>
  )
}
