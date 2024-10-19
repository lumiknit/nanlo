# nanlo

Experimental project to create nano-size ML style language and interpreter base on WASM.

## Documentation

See `/docs` directory.

## Build

### Dependencies

- Rust
- wasm-pack
  - Install with `cargo install wasm-pack`
- Node.js and pnpm
  - Install with `npm install -g pnpm`

### Build for Native

```sh
# Run cargo build at root directory.
# It'll automatically link `dokki` the compiler front-end.
cargo build --release
```

### Build for Web

```sh
# Run wasm-pack build at `seongnyang` directory.
pushd dokki
wasm-pack build
popd

# Then, build the web runtime at `seongnyang` directory.
pushd seongnyang.js
pnpm install
pnpm build
popd
```

