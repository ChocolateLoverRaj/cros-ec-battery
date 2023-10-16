import express from 'express'
import webpackDevServer from 'webpack-dev-middleware'
import webpack from 'webpack'
import webpackHotMiddleware from 'webpack-hot-middleware'
import webpackConfig from './browser/webpack.config'
import { join } from 'path'
import { WebSocketServer } from 'ws'
import { createServer } from 'http'

const app = express()
const port = 9897

const compiler = webpack(webpackConfig)

if (process.env.NODE_ENV === 'production') {
  app.use(express.static(join(__dirname, './browser')))
} else {
  // eslint-disable-next-line @typescript-eslint/no-misused-promises
  app.use(webpackDevServer(compiler))
  app.use(webpackHotMiddleware(compiler))
}

app.get('/api', (req, res) => {
  res.send('Hello from Chromebook EC Battery Health Saver Server!')
})

const server = createServer(app)

const wss = new WebSocketServer({ server })

wss.on('connection', ws => {
  console.log('connect')

  ws.on('error', error => {
    console.error(error)
  })

  ws.on('message', message => {
    console.log(message)
  })

  ws.on('close', () => {
    console.log('close')
  })
})

server.listen(port, () => {
  console.log(`Listening on port http://localhost:${port}`)
})
