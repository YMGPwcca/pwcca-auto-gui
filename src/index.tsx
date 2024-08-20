import ReactDOM from 'react-dom/client'
import { BrowserRouter, Route, Routes } from 'react-router-dom'

import './index.css'

import Home from './pages/Home'
import Settings from './pages/Settings'

const root = ReactDOM.createRoot(document.getElementById('root')!)

root.render(
  <BrowserRouter>
    <div className='[scrollbar-width:none] [-ms-overflow-style:none] select-none [-webkit-touch-callout:none] [-webkit-user-select:none] [-khtml-user-select:none] [-moz-user-select:none] [-ms-user-select:none] [-webkit-tap-highlight-color:transparent]'>
      <Routes>
        <Route path='/' element={<Home />} />
        <Route path='/setting/*' element={<Settings />} />
        <Route path='*' element={<>404 FUCK</>} />
      </Routes>
    </div>
  </BrowserRouter>
)