import { useEffect, useState } from 'react'
import './App.css'
import Button from './components/Button'

function App() {
  const [message, setMessage] = useState('')

  useEffect(() => {
    fetch('http://localhost:3001/')
      .then(response => response.text())
      .then(data => setMessage(data))
  })

  return (
    <>
      <Button color="primary" onClick={() => {console.log("Pressed")}}>Press Me</Button>
      <p>{message}</p>
    </>
  )
}

export default App
