# WebAssembly examples

This directory groups the repository's WebAssembly samples and browser-side stress tests.

## Samples

- [`emscripten/`](emscripten/README.md): the original `wasm32-unknown-emscripten` WebGL sample.
- [`unknown/`](unknown/README.md): a `wasm32-unknown-unknown` sample with a default raster backend and an optional `gl` feature for WebGL2.

## Stress tests

- `testing/emscripten/`: allocation stress app for `wasm32-unknown-emscripten`.
- `testing/unknown/`: allocation stress app for `wasm32-unknown-unknown`.
