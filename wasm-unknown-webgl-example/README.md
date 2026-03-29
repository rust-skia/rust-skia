# wasm32-unknown-unknown WebGL2 sample

Draws a black circle that follows the mouse using a GPU surface via `skia-safe`, WebGL2, and `wasm-bindgen`.

## Build

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --locked
make build
```

`make build_release` applies a larger initial WebAssembly memory (`--initial-memory=16777216`) to avoid a release-only allocation trap during Skia initialization.

## Run

```bash
make serve
```

Open <http://localhost:8001/>.

WASI imports used by the generated module are shimmed in `skia-bindings`, so no browser-side `wasi_snapshot_preview1` import map entry is required.
