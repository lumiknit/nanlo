// rollup.config.ts

import outputSize from "rollup-plugin-output-size";
import rust from "@wasm-tool/rollup-plugin-rust";
import terser from "@rollup/plugin-terser";
import typescript from "@rollup/plugin-typescript";

export default {
  input: {
    index: "ts/index.ts",
    dokki: "./Cargo.toml",
  },
  output: {
    dir: "dist",
    sourcemap: true,
  },
  plugins: [
    outputSize(),
    rust({
      inlineWasm: true,
    }),
    terser(),
    typescript(),
  ],
};
