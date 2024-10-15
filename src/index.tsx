import ReactDOM from 'react-dom/client'
import { BrowserRouter, Route, Routes } from 'react-router-dom'

import './index.css'

import Home from '@/pages/Home'

import Microphone from '@/pages/Settings/microphone'
import AutoStart from '@/pages/Settings/autostart'
import Power from '@/pages/Settings/power'
import Taskbar from '@/pages/Settings/taskbar'
import Mute from '@/pages/Mute'

const root = ReactDOM.createRoot(document.getElementById('root')!)

root.render(
  <BrowserRouter>
    <Routes>
      <Route path='/' element={<Home />} />

      <Route path='/setting/Microphone' element={<Microphone />} />
      <Route path='/setting/AutoStart' element={<AutoStart />} />
      <Route path='/setting/Power' element={<Power />} />
      <Route path='/setting/Taskbar' element={<Taskbar />} />

      <Route path='/Mute' element={<Mute />} />

      <Route path='*' element={<>404 FUCK</>} />
    </Routes>
  </BrowserRouter>
)