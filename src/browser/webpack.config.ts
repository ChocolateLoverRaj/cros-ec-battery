import webpack from 'webpack'
import { join } from 'path'
import HtmlWebpackPlugin from 'html-webpack-plugin'
import 'webpack-dev-server'
import ReactRefreshWebpackPlugin from '@pmmmwh/react-refresh-webpack-plugin'

const isProduction = process.env.NODE_ENV === 'production'
const isDevelopment = !isProduction
const mode = isProduction ? 'production' : 'development'
const config: webpack.Configuration = {
  entry: [
    ...isDevelopment ? ['webpack-hot-middleware/client'] : [],
    join(__dirname, './index')
  ],
  output: {
    path: join(__dirname, '../../dist/browser')
  },
  mode,
  plugins: [
    new HtmlWebpackPlugin(),
    ...isDevelopment
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
            ...isDevelopment ? [require.resolve('react-refresh/babel')] : []
          ]
        }
      }
    }]
  }
}

export default config
