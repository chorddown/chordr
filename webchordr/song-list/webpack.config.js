const path = require('path');
const webpack = require('webpack');

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
        entry: './src-typescript/SortableWrapper.ts',
        experiments: {
            outputModule: true,
        },
        output: {
            publicPath: '/',
            path: distPath,
            filename: "sortable.js",

            // library: 'SortableWrapper',
            library: {type: 'module'},
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
            new webpack.HotModuleReplacementPlugin(),
        ],
        watchOptions: {
            aggregateTimeout: 3000,
            ignored: ['**/*.scss', '**/node_modules'],
        },
        cache: true
    };
});
