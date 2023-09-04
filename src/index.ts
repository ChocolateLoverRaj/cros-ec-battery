import express from 'express'
import webpackDevServer from 'webpack-dev-middleware'
import webpack from 'webpack'
import { join } from 'path'
import HtmlWebpackPlugin from 'html-webpack-plugin'
import 'webpack-dev-server'
import ReactRefreshWebpackPlugin from '@pmmmwh/react-refresh-webpack-plugin'
import webpackHotMiddleware from 'webpack-hot-middleware'

const app = express()
const port = 9897

const mode = process.env.NODE_ENV === 'production' ? 'production' : 'development'
const compiler = webpack({
  entry: ['webpack-hot-middleware/client', join(__dirname, '../src/browser/index')],
  mode,
  plugins: [
    new HtmlWebpackPlugin(),
    ...mode === 'development'
      ? [
          new ReactRefreshWebpackPlugin(),
          new webpack.HotModuleReplacementPlugin()
        ]
      : []
  ],
  resolve: {
    extensions: ['.tsx', '.ts', '.js']
  },
  module: {
    rules: [{
      test: /\.tsx?$/,
      exclude: /node_modules/,
      use: {
        loader: 'babel-loader',
        options: {
          presets: [
            '@babel/preset-typescript',
            '@babel/preset-react'
          ],
          plugins: [
            'babel-plugin-react-require',
            ...mode === 'development' ? [require.resolve('react-refresh/babel')] : []
          ]
        }
      }
    }]
  }
})

// eslint-disable-next-line @typescript-eslint/no-misused-promises
app.use(webpackDevServer(compiler))
app.use(webpackHotMiddleware(compiler))

app.get('/api', (req, res) => {
  res.send('Hello from Chromebook EC Battery Health Saver Server!')
})

app.listen(port, () => {
  console.log(`Listening on port http://localhost:${port}`)
})
