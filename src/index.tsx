import ReactDOM from 'react-dom/client'
import { BrowserRouter, Route, Routes } from 'react-router-dom'

import './index.css'

import Home from './pages/Home'

import Microphone from './pages/Settings/microphone'
import AutoStart from './pages/Settings/autostart'
import Power from './pages/Settings/battery'
import Taskbar from './pages/Settings/taskbar'
import Root from './pages/root'

const root = ReactDOM.createRoot(document.getElementById('root')!)

root.render(
  <BrowserRouter>
    <Root>
      <Routes>
        <Route path='/' element={<Home />} />

        <Route path='/setting/Microphone' element={<Microphone />} />
        <Route path='/setting/AutoStart' element={<AutoStart />} />
        <Route path='/setting/Power' element={<Power />} />
        <Route path='/setting/Taskbar' element={<Taskbar />} />

        <Route path='*' element={<>404 FUCK</>} />
      </Routes>
    </Root>
  </BrowserRouter>
)