# WebAssembly Sample with Graphite on WebGPU

Draws the animated rust-skia icon with Graphite on the browser's WebGPU, through `wgpu`.
The module is built for `wasm32-unknown-unknown` and processed by [wasm-bindgen](https://wasm-bindgen.github.io/wasm-bindgen/), so it needs no Emscripten runtime in the browser.
This target is experimental.

## Build and Run the Example

Skia itself is still compiled by the Emscripten SDK, so install it, and point `EMSDK` at it.
Bindgen needs a libclang that parses the SDK's libc++ headers, version 20 or later.
Set `LIBCLANG_PATH` if your default one is older.

Install the target and the wasm-bindgen CLI, in the version of the `wasm-bindgen` crate in `Cargo.lock`:

```shell
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
```

Then build the example:

```shell
cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web --out-dir web/pkg target/wasm32-unknown-unknown/debug/wasm_wgpu_example.wasm
```

Start a web server (requires Python 3):

```shell
python3 -m http.server --directory web
```

Then open http://localhost:8000 in a browser with WebGPU.

## Notes

- A page can't wait for the GPU, so the Graphite context never does.
  `Context::submit_and_wait` isn't available, and the example calls `Context::check_async_work_completion` every frame.
  Before you drop a context, wait until `Context::has_unfinished_gpu_work` returns `false`.
- `skia-bindings` links Emscripten's libc and libc++, and supplies the allocator and an empty file system for them.
  Set `SKIA_WASM_NO_LIBC` when you build to supply these yourself.
- The browser reports the errors of a shader after Skia has used it, so they only show in the console.
