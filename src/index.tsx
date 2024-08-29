import ReactDOM from 'react-dom/client'
import { BrowserRouter, Route, Routes } from 'react-router-dom'

import './index.css'

import Home from './pages/Home'

import Microphone from './pages/Settings/microphone'
import AutoStart from './pages/Settings/autostart'
import Battery from './pages/Settings/Battery'
import Taskbar from './pages/Settings/taskbar'

const root = ReactDOM.createRoot(document.getElementById('root')!)

root.render(
  <BrowserRouter>
    <div className='mobile:bg-tier0 [scrollbar-width:none] [-ms-overflow-style:none] select-none [-webkit-touch-callout:none] [-webkit-user-select:none] [-khtml-user-select:none] [-moz-user-select:none] [-ms-user-select:none] [-webkit-tap-highlight-color:transparent]'>
      <Routes>
        <Route path='/' element={<Home />} />

        <Route path='/setting/Microphone' element={<Microphone />} />
        <Route path='/setting/AutoStart' element={<AutoStart />} />
        <Route path='/setting/Battery' element={<Battery />} />
        <Route path='/setting/Taskbar' element={<Taskbar />} />

        <Route path='*' element={<>404 FUCK</>} />
      </Routes>
    </div>
  </BrowserRouter>
)