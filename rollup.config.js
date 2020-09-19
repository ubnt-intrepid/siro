import glob from 'glob';
import path from 'path';
import rust from '@wasm-tool/rollup-plugin-rust';
import postcss from 'rollup-plugin-postcss';

export default glob.sync('./examples/**/index.js')
    .map(manifest => {
        const manifestDir = path.dirname(manifest);
        return {
            input: {
                app: manifest,
            },
            output: {
                dir: path.resolve(manifestDir, 'dist'),
                format: 'iife',
                sourcemap: true,
            },
            plugins: [
                rust(),
                postcss(),
            ]
        };
    });
