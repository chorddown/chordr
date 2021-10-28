const path = require('path');
const webpack = require('webpack');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const WorkboxPlugin = require('workbox-webpack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');

const distPath = path.resolve(__dirname, "dist");

const SpeedMeasurePlugin = require("speed-measure-webpack-plugin");

const smp = new SpeedMeasurePlugin();

module.exports = smp.wrap((env, argv) => {
  return {
    devServer: {
      contentBase: distPath,
      compress: argv.mode === 'production',
      port: 8000,
      historyApiFallback: {disableDotRule: true}
    },
    entry: './bootstrap.ts',
    output: {
      publicPath: '/',
      path: distPath,
      filename: "app.js",
      webassemblyModuleFilename: "app.wasm"
    },
    module: {
      rules: [
        // {
        //   test: /\.s[ac]ss$/i,
        //   use: [
        //     'style-loader',
        //     'css-loader?sourceMap',
        //     'sass-loader?sourceMap',
        //   ],
        // },
        {
          test: /\.tsx?$/,
          use: 'ts-loader',
          exclude: /node_modules/,
        },
      ],
    },
    resolve: {
      extensions: ['.tsx', '.ts', '.js'],
    },
    plugins: [
      new CopyWebpackPlugin({
        patterns: [
          {
            from: './static',
            globOptions: {
              dot: false,
              ignore: ['**/songs/**', '**/*.scss', '**/*.chorddown'],
            },
          }
        ]
      }),
      new WasmPackPlugin({
        crateDirectory: __dirname
      }),
      new WorkboxPlugin.GenerateSW({
        // these options encourage the ServiceWorkers to get in there fast
        // and not allow any straggling "old" SWs to hang around
        clientsClaim: true,
        skipWaiting: true,
        cacheId: 'net.chordr',
        exclude: ['static/songs/.*', /songs/, /.*\.scss/, 'catalog.json'],
        maximumFileSizeToCacheInBytes: 50 * 1024 * 1024,
        navigateFallback: '/index.html',
        runtimeCaching: [
          {
            urlPattern: '/catalog.json',
            handler: 'NetworkFirst'
          },
          {
            urlPattern: /catalog\.json\?\d+/,
            handler: 'NetworkOnly'
          },
          {
            urlPattern: /\/status\/$/,
            handler: 'NetworkOnly'
          },
          {
            // Development server URI
            urlPattern: 'http://localhost:9000/status/',
            handler: 'NetworkOnly'
          },
          {
            urlPattern: /assets/,
            handler: 'StaleWhileRevalidate'
          },
          {
            urlPattern: /.*/,
            handler: 'NetworkFirst'
          }
        ]
      }),
      new webpack.HotModuleReplacementPlugin(),

    ],
    watchOptions: {
      aggregateTimeout: 3000,
      ignored: ['**/*.scss', '**/node_modules'],
    },
    cache: true
  };
});
