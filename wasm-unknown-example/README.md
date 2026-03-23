# wasm32-unknown-unknown sample

This sample shows a minimal CPU/raster `skia-safe` rendering path for `wasm32-unknown-unknown` and exposes one function (`render_scene`) via `wasm-bindgen`.

No GL/WebGL or textlayout features are used by this example crate.

## Build

From this directory:

```bash
rustup target add wasm32-unknown-unknown
cargo check --target wasm32-unknown-unknown
make build
```

`make build` uses `wasm-bindgen` and requires `wasm-bindgen-cli`:

```bash
cargo install wasm-bindgen-cli --locked
```

`make build_release` applies a larger initial WebAssembly memory (`--initial-memory=16777216`) to avoid a release-only allocation trap during Skia raster initialization on this target.

## Run in browser

```bash
make serve
```

Open <http://localhost:8000/>.
