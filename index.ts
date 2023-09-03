import express from 'express'

const app = express()
const port = 9897

app.get('/', (req, res) => {
  res.send('Hello from Chromebook EC Battery Health Saver Server!')
})

app.listen(port, () => {
  console.log(`Listening on port http://localhost:${port}`)
})
