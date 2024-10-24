# nanlo

Experimental project to create nano-size ML style language and interpreter base on WASM.

## Documentation

See `/docs` directory.

## Build

### Dependencies

- Rust
- wasm-pack (for building web runtime)
  - Install with `cargo install wasm-pack`
- Node.js and pnpm (for building web runtime)
  - Install with `npm install -g pnpm`

### Build for Native

The root directory is the project to build for native environment.

```sh
# Run cargo build at root directory.
# It'll automatically link `dokki` and `seongnyang` as dependencies.
cargo build --release
```

### Build for Web

The `seongnyang.js` directory is the project to build for web environment.
The directory contains the wrapper for `dokki` to build as a web assembly,
and the web runtime to run the web assembly.s

```sh
# Then, build the web runtime at `seongnyang` directory.
pushd seongnyang.js
pnpm install
pnpm build
popd
```

