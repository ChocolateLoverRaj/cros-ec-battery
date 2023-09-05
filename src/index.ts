import express from 'express'
import webpackDevServer from 'webpack-dev-middleware'
import webpack from 'webpack'
import webpackHotMiddleware from 'webpack-hot-middleware'
import webpackConfig from './browser/webpack.config'
import { join } from 'path'
const app = express()
const port = 9897

const compiler = webpack(webpackConfig)

if (process.env.NODE_ENV === 'development') {
  // eslint-disable-next-line @typescript-eslint/no-misused-promises
  app.use(webpackDevServer(compiler))
  app.use(webpackHotMiddleware(compiler))
} else {
  app.use(express.static(join(__dirname, './browser')))
}

app.get('/api', (req, res) => {
  res.send('Hello from Chromebook EC Battery Health Saver Server!')
})

app.listen(port, () => {
  console.log(`Listening on port http://localhost:${port}`)
})
