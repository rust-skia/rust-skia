extern crate bindgen;
extern crate cc;

mod build_support;
use build_support::skia;

fn main() {
    let configuration = skia::Configuration::from_cargo_env();
    skia::build(&configuration);
    configuration.commit_to_cargo();
}
