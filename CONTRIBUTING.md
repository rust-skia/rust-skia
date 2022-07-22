# Rust-Skia Contribution Guidelines

Thank you for considering to contribute to rust-skia.

We welcome all contributions, but most likely, a large part of your contribution will be Rust code.

To contribute Rust code, format the code with `cargo fmt` and be sure that there are no warnings with `cargo clippy`. Don't try too hard follow Clippy suggestions. If a warning does not make sense to be adhered to, add a comment that explains why, and mark the code with a `#[allow(clippy::*)]` attribute.

If possible, add a small test case for you changes. 

Your PR will be built for and tested on a number of targets on the CI before it can be merged. If that fails, we are probably able to help out.

If you'd like the changes in your PR to be released to [crates.io](https://crates.io/) timely, please say so, because we prefer to align crate releases with major [updates to Chrome stable](https://chromestatus.com/roadmap).

## Repository organization

The repository consists of two cargo packages in the folders `/skia-bindings` and `/skia-safe`. `skia-bindings` contains the build support for Skia and the C++ bindings. `skia-safe` contains all the Rust code that wraps the Skia APIs.

## Contributing Bindings & Wrappers

We did our best to cover most of the Skia API, but you'll find a lot of blind spots by looking closer:

- GPU API support is incomplete, specifically functions that use callbacks.
- Supporting [Skottie](https://skia.org/user/modules/skottie) would be a nice addition.

For larger contributions, make yourself familiar with the [various wrapper types](https://github.com/rust-skia/rust-skia/wiki) of skia-safe and consider to file an issue beforehand to give us a heads up and to receive additional directions.

## Contributing Examples

Examples should be added to `/skia-safe/examples` or directly to the `/skia-safe/examples/skia-org` executable which provides a minimal infrastructure to render to PNG , PDF, SVG files with the CPU or GPU.

## Updating Skia

Since the beginning of this project, the maintainer team is updating rust-skia to match the Skia API version in the latest stable Chrome release. A new Chrome release [is scheduled](https://chromium.googlesource.com/chromium/src/+/master/docs/process/release_cycle.md) about every 6 weeks and most changes for the upcoming version are already settled when the current release is out, so there is enough time to update rust-skia. For most new Skia releases, the changes are trivial and can be integrated without breaking backwards compatibility on the Rust side.

Even though this puts some pressure on dependent projects, we'd try to keep rust-skia updated, because falling behind Chrome's and Flutter's development to keep Skia _modern_ is not an option.

But keeping up might not always be possible. So if you or your company uses rust-skia, you might be interested to take a look at the [template used](https://github.com/rust-skia/rust-skia/wiki/Template:-Skia-Milestone-Update-PR) to update to the latest Skia version and perhaps even try to help us out from time to time.

## Licensing

Please ensure that the material you contribute is compatible with the MIT license.

