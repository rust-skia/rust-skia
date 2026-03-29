# wasm32-unknown-unknown sample

Draws a black circle that follows the mouse using a CPU raster surface via `skia-safe` and `wasm-bindgen`.

## Build

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --locked
make build
```

`make build_release` applies a larger initial WebAssembly memory (`--initial-memory=16777216`) to avoid a release-only allocation trap during Skia raster initialization.

## Run

```bash
make serve
```

Open <http://localhost:8001/>.
