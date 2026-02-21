# Rust-Skia Contribution Guidelines

Thank you for considering to contribute to rust-skia.

We welcome all contributions. Most likely, a large part of your contribution will be Rust code.

## Repository organization

The repository consists of two primary cargo packages in the folders `/skia-bindings` and `/skia-safe`. `skia-bindings` contains the build support for Skia and the C++ bindings. `skia-safe` contains all the Rust code that wraps the Skia APIs.

## Contributing Bindings & Wrappers

We did our best to cover most of the Skia API, but you'll find a lot of blind spots by looking closer:

- GPU API support is incomplete, specifically functions that use callbacks.
- The new rendering backend Graphite isn't supported yet.

For larger contributions, familiarize yourself with the [various wrapper types](https://github.com/rust-skia/rust-skia/wiki) and consider filing an issue beforehand to give us a heads up and to receive additional directions.

## Contributing Examples

Examples should be added to `/skia-safe/examples` or directly to the `/skia-org` executable which provides a minimal infrastructure to render to PNG, PDF, and SVG files using the CPU or the GPU backends.

## Contribution Checklist

To contribute Rust code, format the code with `cargo fmt` and be sure that there are no warnings with `cargo clippy`. Don't try too hard follow Clippy suggestions. If a warning does not make sense, add a comment that explains why and mark the code with a `#[allow(clippy::*)]` attribute.

If possible, add a small test case for your changes.

Your PR will be built for and tested on a number of targets on the CI before it can be merged. If that fails, we will do our best to help out.

If you'd like the changes in your PR to be released to [crates.io](https://crates.io/) timely, please say so, because we prefer to align crate releases with major [updates to Chrome stable](https://chromestatus.com/roadmap).

## AI Assistance Disclosure

If you used AI assistance for substantial parts of your contribution (for example, code generation, refactoring suggestions, or documentation drafting), clearly state that in your PR description.

## Licensing

Please ensure that the code you contribute is compatible with the MIT license.

