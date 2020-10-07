const path = require('path');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

module.exports = (env, arg) => {
    return {
        mode: 'development',
        entry: path.resolve(__dirname, 'index.js'),
        output: {
            publicPath: 'dist/',
            path: path.resolve(__dirname, 'dist'),
            filename: 'bundle.js',
        },
        plugins: [
            new CleanWebpackPlugin(),
            new WasmPackPlugin({
                crateDirectory: __dirname,
                extraArgs: '--no-typescript',
                outName: 'index',
                outDir: path.resolve(__dirname, 'pkg'),
            }),
        ],
        module: {
            rules: [
                {
                    test: /\.css$/i,
                    use: [ 'style-loader', 'css-loader' ],
                }
            ],
        },
    };
}
