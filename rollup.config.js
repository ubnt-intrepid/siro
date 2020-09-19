import glob from 'glob';
import path from 'path';
import rust from '@wasm-tool/rollup-plugin-rust';

export default glob.sync('./examples/**/Cargo.toml')
    .map(manifest => {
        const manifestDir = path.dirname(manifest);
        return {
            input: {
                app: manifest,
            },
            output: {
                dir: path.resolve(manifestDir, 'dist/js'),
                format: 'iife',
                sourcemap: true,
            },
            plugins: [
                rust({
                    serverPath: 'js/',
                }),
            ]
        };
    });