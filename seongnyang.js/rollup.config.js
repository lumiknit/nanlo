// rollup.config.ts

import outputSize from 'rollup-plugin-output-size';
import terser from '@rollup/plugin-terser';
import typescript from '@rollup/plugin-typescript';
import wasm from '@rollup/plugin-wasm';

export default {
  input: 'src/index.ts',
  output: [
    {
      file: 'dist/seongnyang.cjs.js',
      format: 'cjs',
      sourcemap: true,
    },
    {
      file: 'dist/seongnyang.esm.js',
      format: 'esm',
      sourcemap: true,
    },
    {
      file: 'dist/seongnyang.iife.js',
      format: 'iife',
      sourcemap: true,
      name: 'seongnyang',
    },
  ],
  plugins: [
    outputSize(),
    terser(),
    typescript(),
    wasm(),
  ],
}
