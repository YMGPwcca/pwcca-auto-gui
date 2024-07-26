import ReactDOM from 'react-dom/client'
import { BrowserRouter, Route, Routes } from 'react-router-dom'

import './index.css'

import Home from './pages/Home'

const root = ReactDOM.createRoot(document.getElementById('root')!)

root.render(
  <BrowserRouter>
    <div className='no-scrollbar select-none [-webkit-touch-callout:none] [-webkit-user-select:none] [-khtml-user-select:none] [-moz-user-select:none] [-ms-user-select:none]'>
      <Routes>
        <Route path='/' element={<Home />} />
        <Route path='*' element={<>404 FUCK</>} />
      </Routes>
    </div>
  </BrowserRouter>
)