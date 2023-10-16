import React, { FC, useEffect } from 'react'

const Ws: FC = () => {
  useEffect(() => {
    const ws = new WebSocket(`ws://${window.location.host}`)
    ws.addEventListener('open', () => {
      ws.send('hi');
    })
    return () => {
      ws.close()
    }
  }, [])

  return <>Web socket</>
}

export default Ws
