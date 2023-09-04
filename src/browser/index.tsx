import { createRoot } from 'react-dom/client'
import App from './App'
import { StrictMode } from 'react'

const app = document.createElement('div')
document.body.appendChild(app)
createRoot(app).render(<StrictMode><App /></StrictMode>)
