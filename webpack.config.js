const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

const distDir = path.resolve(__dirname, 'dist');

module.exports = (env, argv) => {
    return {
        mode: 'development',
        entry: {
            app: path.resolve(__dirname, 'index.js'),
        },
        output: {
            publicPath: '/',
            path: distDir,
            filename: '[name].[contenthash].js',
        },
        devServer: {
            contentBase: distDir,
            port: 8080,
        },
        plugins: [
            new WasmPackPlugin({
                crateDirectory: __dirname,
                outName: 'index',
                extraArgs: '--no-typescript',
            }),
            new HtmlWebpackPlugin({
                filename: 'index.html',
                template: path.resolve(__dirname, 'index.html'),
            }),
        ],
    };
};
