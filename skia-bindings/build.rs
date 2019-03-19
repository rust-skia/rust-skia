extern crate bindgen;
extern crate cc;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use bindgen::EnumVariation;
use cc::Build;

fn main() {
  let platform = if cfg!(target_os = "windows") {
    "win"
  } else if cfg!(target_os = "linux") {
    "linux"
  } else if cfg!(target_os = "macos") {
    "osx"
  } else {
    panic!("Unsupport platform");
  };

  let commit_sha_short = "a7a87538b2";

  let tar_name = format!("skia-static-{}.tgz", platform);

  if fs::metadata("./static/libskia.a").is_err() || fs::metadata("./include/core/SkCanvas.h").is_err() || fs::metadata("./third_party").is_err() {
    fs::remove_file(format!("./{}", tar_name)).unwrap_or(());
    fs::remove_file("./skia-include.tgz").unwrap_or(());
    fs::remove_file("./skia-third-party.tgz").unwrap_or(());
    fs::remove_dir("./third_party").unwrap_or(());
    fs::remove_dir("./include").unwrap_or(());
    fs::remove_dir("./static").unwrap_or(());
    assert!(Command::new("curl")
      .arg("-L")
      .arg(&format!("https://github.com/rust-skia/skia/releases/download/{}/{}", commit_sha_short, tar_name))
      .arg("--output")
      .arg(&tar_name)
      .stdin(Stdio::inherit())
      .stderr(Stdio::inherit())
      .status().unwrap().success());

    assert!(Command::new("curl")
      .arg("-L")
      .arg(&format!("https://github.com/rust-skia/skia/releases/download/{}/skia-include.tgz", commit_sha_short))
      .arg("--output")
      .arg("skia-include.tgz")
      .stdin(Stdio::inherit())
      .stderr(Stdio::inherit())
      .status().unwrap().success());

    assert!(Command::new("curl")
      .arg("-L")
      .arg(&format!("https://github.com/rust-skia/skia/releases/download/{}/skia-third-party.tgz", commit_sha_short))
      .arg("--output")
      .arg("skia-third-party.tgz")
      .stdin(Stdio::inherit())
      .stderr(Stdio::inherit())
      .status().unwrap().success());

    assert!(
      Command::new("tar")
        .args(&["-xvf", &tar_name])
        .stdin(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status().unwrap().success()
    );

    assert!(
      Command::new("tar")
        .args(&["-xvf", "skia-include.tgz"])
        .stdin(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status().unwrap().success()
    );

    assert!(
      Command::new("tar")
        .args(&["-xvf", "skia-third-party.tgz"])
        .stdin(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status().unwrap().success()
    );
  }

  let mut skia_out_dir = env::current_dir().unwrap();
  skia_out_dir.push("static");
  let skia_out_dir = skia_out_dir.to_str().unwrap();
  let current_dir = env::current_dir().unwrap();
  let current_dir_name = current_dir.to_str().unwrap();

  println!("cargo:rustc-link-search={}", &skia_out_dir);
  cargo::add_link_libs(&["static=skia", "static=skiabinding"]);

  let target = env::var("TARGET").unwrap();
  if target.contains("unknown-linux-gnu") {
    cargo::add_link_libs(&["stdc++", "bz2", "GL", "fontconfig", "freetype"]);
  } else if target.contains("eabi") {
    cargo::add_link_libs(&["stdc++", "GLESv2"]);
  } else if target.contains("apple-darwin") {
    cargo::add_link_libs(&["c++", "framework=OpenGL", "framework=ApplicationServices"]);
  } else if target.contains("windows") {
    if target.contains("gnu") {
      cargo::add_link_lib("stdc++");
    }
    cargo::add_link_libs(&["usp10", "ole32", "user32", "gdi32", "fontsub", "opengl32"]);
  }

  bindgen_gen(&current_dir_name, &skia_out_dir)
}

fn bindgen_gen(current_dir_name: &str, skia_out_dir: &str) {

  let mut builder = bindgen::Builder::default()
    .generate_inline_functions(true)

    .default_enum_style(EnumVariation::Rust)

    .constified_enum(".*Mask")
    .constified_enum(".*Flags")
    .constified_enum("SkCanvas_SaveLayerFlagsSet")
    .constified_enum("GrVkAlloc_Flag")
    .constified_enum("GrGLBackendState")

    .whitelist_function("C_.*")
    .whitelist_function("SkColorTypeBytesPerPixel")
    .whitelist_function("SkColorTypeIsAlwaysOpaque")
    .whitelist_function("SkColorTypeValidateAlphaType")
    .whitelist_function("SkRGBToHSV")
    // this function does not whitelist (probably because of inlining):
    .whitelist_function("SkColorToHSV")
    .whitelist_function("SkHSVToColor")
    .whitelist_function("SkPreMultiplyARGB")
    .whitelist_function("SkPreMultiplyColor")
    .whitelist_function("SkBlendMode_Name")

    // functions for which the doc generation fails.
    .blacklist_function("SkColorFilter_asComponentTable")

    .whitelist_type("SkColorSpacePrimaries")
    .whitelist_type("SkVector4")
    .whitelist_type("SkPictureRecorder")
    .whitelist_type("SkAutoCanvasRestore")

    .whitelist_type("SkPath1DPathEffect")
    .whitelist_type("SkLine2DPathEffect")
    .whitelist_type("SkPath2DPathEffect")
    .whitelist_type("SkCornerPathEffect")
    .whitelist_type("SkDashPathEffect")
    .whitelist_type("SkDiscretePathEffect")
    .whitelist_type("SkGradientShader")
    .whitelist_type("SkPerlinNoiseShader")
    .whitelist_type("SkTableColorFilter")

    .whitelist_type("GrGLBackendState")

    .whitelist_type("GrVkDrawableInfo")
    .whitelist_type("GrVkExtensionFlags")
    .whitelist_type("GrVkFeatureFlags")

    .whitelist_var("SK_Color.*")
    .whitelist_var("kAll_GrBackendState")

    .use_core()
    .clang_arg("-std=c++14");

  let mut cc_build = Build::new();

  let bindings_source = "src/bindings.cpp";
  cargo::add_dependent_path(bindings_source);

  builder = builder.header(bindings_source);

  for include_dir in fs::read_dir("include").expect("Unable to read ./include") {
    let dir = include_dir.unwrap();
    cargo::add_dependent_path(dir.path().to_str().unwrap());
    let include_path = format!("{}/{}", &current_dir_name, &dir.path().to_str().unwrap());
    builder = builder.clang_arg(format!("-I{}", &include_path));
    cc_build.include(&include_path);
  }

  if cfg!(feature="vulkan") {
    cc_build.define("SK_VULKAN", "1");
    builder = builder.clang_arg("-DSK_VULKAN");
    cc_build.define("SKIA_IMPLEMENTATION", "1");
    builder = builder.clang_arg("-DSKIA_IMPLEMENTATION=1");
  }

  let cc_build = cc_build
    .cpp(true)
    .file(bindings_source)
    .out_dir(skia_out_dir);

  let cc_build = if !cfg!(windows) { cc_build.flag("-std=c++14") } else { cc_build };

  cc_build.compile("skiabinding");

  let bindings = builder.generate().expect("Unable to generate bindings");

  let out_path = PathBuf::from("src");
  bindings
    .write_to_file(out_path.join("bindings.rs"))
    .expect("Couldn't write bindings!");
}

mod cargo {
  pub fn add_dependent_path(path: &str) {
    println!("cargo:rerun-if-changed={}", path);
  }

  pub fn add_link_libs<'a, L: IntoIterator<Item = &'a &'a str>>(libs: L) {
    libs.into_iter().for_each(|s| add_link_lib(*s))
  }

  pub fn add_link_lib(lib: &str) {
    println!("cargo:rustc-link-lib={}", lib);
  }
}
