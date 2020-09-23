const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = (env, arg) => {
    return {
        mode: 'development',
        entry: {
            app: path.resolve(__dirname, 'index.js'),
        },
        output: {
            publicPath: './',
            path: path.resolve(__dirname, 'dist'),
            filename: '[name].bundle.js',
        },
        devServer: {
            contentBase: path.resolve(__dirname, 'dist'),
            port: 8080,
        },
        plugins: [
            new WasmPackPlugin({
                crateDirectory: __dirname,
                extraArgs: '--no-typescript',
                outName: 'index',
                outDir: path.resolve(__dirname, 'pkg'),
                watchDirectories: [
                    path.resolve(__dirname, '../../src'),
                ],
            }),
            new HtmlWebpackPlugin({
                filename: 'index.html',
                template: path.resolve(__dirname, 'index.html'),
            }),
        ],
        module: {
            rules: [
                {
                    test: /\.css$/i,
                    use: [ 'style-loader', 'css-loader' ],
                }
            ],
        }
    };
};
