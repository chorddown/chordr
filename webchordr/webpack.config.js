const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const WorkboxPlugin = require('workbox-webpack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');

const distPath = path.resolve(__dirname, "dist");
module.exports = (env, argv) => {
  return {
    devServer: {
      contentBase: distPath,
      compress: argv.mode === 'production',
      port: 8000
    },
    entry: './bootstrap.ts',
    output: {
      path: distPath,
      filename: "webchordr.js",
      webassemblyModuleFilename: "webchordr.wasm"
    },
    module: {
      rules: [
        {
          test: /\.s[ac]ss$/i,
          use: [
            'style-loader',
            'css-loader',
            'sass-loader',
          ],
        },
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
      new CopyWebpackPlugin([
        {from: './static', to: distPath}
      ]),
      new WasmPackPlugin({
        crateDirectory: ".",
        withTypeScript: true
      }),
      new WorkboxPlugin.GenerateSW({
        // these options encourage the ServiceWorkers to get in there fast
        // and not allow any straggling "old" SWs to hang around
        clientsClaim: true,
        skipWaiting: true,
        cacheId: 'net.chordr',
        exclude: ['static/songs/.*', /songs/, /.*\.scss/],
        maximumFileSizeToCacheInBytes: 50 * 1024 * 1024,
        runtimeCaching: [
          {
            urlPattern: /.*/,
            handler: 'NetworkFirst'
          },
          {
            urlPattern: '/catalog.json',
            handler: 'NetworkFirst'
          },
          {
            urlPattern: /catalog\.json\?\d+/,
            handler: 'NetworkOnly'
          },
          {
            urlPattern: /status/,
            handler: 'NetworkOnly'
          },
          {
            urlPattern: /assets/,
            handler: 'StaleWhileRevalidate'
          }
        ]
      })
    ],
    watch: argv.mode !== 'production'
  };
};
