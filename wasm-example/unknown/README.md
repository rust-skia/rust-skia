# wasm32-unknown-unknown sample

Draws a black circle that follows the pointer using `skia-safe` and `wasm-bindgen`.

The default build uses a CPU raster surface. Enable the `gl` feature to switch the same sample to a
WebGL2-backed GPU surface.

## Build

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --locked
make build
```

Build the WebGL2 variant with:

```bash
FEATURES=gl make build
```

`make build_release` applies a larger initial WebAssembly memory (`--initial-memory=16777216`) to avoid a release-only allocation trap during Skia initialization.

## Run

```bash
make serve
```

Open <http://localhost:8001/>.

When built with `FEATURES=gl`, the generated module uses rust-skia's WASI and WebGL shims, so no
browser-side `wasi_snapshot_preview1` import map entry is required.
