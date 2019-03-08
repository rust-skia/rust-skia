extern crate bindgen;
extern crate cc;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use cc::Build;

fn main() {

  if !cfg!(windows) {
    assert!(Command::new("git")
      .arg("submodule")
      .arg("init")
      .stdout(Stdio::inherit())
      .stderr(Stdio::inherit())
      .status().unwrap().success(), "git submodule init fail");

    assert!(Command::new("git")
      .args(&["submodule", "update"])
      .stdout(Stdio::inherit())
      .stderr(Stdio::inherit())
      .status().unwrap().success(), "git submodule update fail");
  }

  assert!(Command::new("python")
    .arg("skia/tools/git-sync-deps")
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .status().unwrap().success(), "git sync deps fail");

  let gn_args = {

    let keep_inline_functions = true;

    let mut args =
      r#"--args=is_official_build=true skia_use_system_expat=false skia_use_system_icu=false skia_use_system_libjpeg_turbo=false skia_use_system_libpng=false skia_use_system_libwebp=false skia_use_system_zlib=false cc="clang" cxx="clang++""#
      .to_owned();

    if cfg!(feature="vulkan") {
      args.push_str(" skia_use_vulkan=true skia_enable_spirv_validation=false");
    }

    if cfg!(windows) {

      let mut flags : Vec<&str> = vec![];
      flags.push(if cfg!(build="debug") { "/MTd" } else { "/MD" });

      if keep_inline_functions {
        // sadly, this also disables inlining completely and is probably a real performance bummer.
        flags.push("/Ob0")
      };

      let flags : String = {
        fn quote(s: &str) -> String { String::from("\"") + s + "\"" }

        let v : Vec<String> =
            flags.into_iter().map(quote).collect();
        v.join(",")
      };

      args.push_str(r#" clang_win="C:\Program Files\LLVM""#);
      args.push_str(&format!(" extra_cflags=[{}]", flags));
    } else {
      if keep_inline_functions {
        args.push_str(r#" extra_cflags=["-fno-inline-functions"]"#)
      }
    }

    args
  };

  let gn_command = if cfg!(windows) {
    "skia/bin/gn"
  } else {
    "bin/gn"
  };

  let skia_out_dir : String =
    PathBuf::from(env::var("OUT_DIR").unwrap())
      .join("skia/Static")
      .to_str().unwrap().into();

  let output = Command::new(gn_command)
    .args(&["gen", &skia_out_dir, &gn_args])
    .envs(env::vars())
    .current_dir(PathBuf::from("./skia"))
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .output()
    .expect("gn error");

  if output.status.code() != Some(0) {
    panic!("{:?}", String::from_utf8(output.stdout).unwrap());
  }

  assert!(Command::new("ninja")
    .current_dir(PathBuf::from("./skia"))
    .args(&["-C", &skia_out_dir])
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .status().unwrap().success(), "ninja error");

  let current_dir = env::current_dir().unwrap();
  let current_dir_name = current_dir.to_str().unwrap();

  println!("cargo:rustc-link-search={}", &skia_out_dir);
  println!("cargo:rustc-link-lib=static=skia");
  println!("cargo:rustc-link-lib=static=skiabinding");

  let target = env::var("TARGET").unwrap();
  if target.contains("unknown-linux-gnu") {
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=bz2");
    println!("cargo:rustc-link-lib=GL");
    println!("cargo:rustc-link-lib=fontconfig");
    println!("cargo:rustc-link-lib=freetype");
  } else if target.contains("eabi") {
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=GLESv2");
  } else if target.contains("apple-darwin") {
    println!("cargo:rustc-link-lib=c++");
    println!("cargo:rustc-link-lib=framework=OpenGL");
    println!("cargo:rustc-link-lib=framework=ApplicationServices");
  } else if target.contains("windows") {
    if target.contains("gnu") {
      println!("cargo:rustc-link-lib=stdc++");
    }
    println!("cargo:rustc-link-lib=usp10");
    println!("cargo:rustc-link-lib=ole32");
    println!("cargo:rustc-link-lib=user32");
    println!("cargo:rustc-link-lib=gdi32");
    println!("cargo:rustc-link-lib=fontsub");

    // required since GrContext::MakeVulkan is linked.
    if cfg!(feature="vulkan") {
      println!("cargo:rustc-link-lib=opengl32");
    }
  }

  // regenerate bindings?
  //
  // The bindings are generated into the src directory to support
  // IDE based symbol lookup in dependent projects, but this has the consequence
  // that the IDE and corgo might be confused by its datestamp, so we
  // avoid the regeneration if possible by implementing our own dependency checks.
  // The results of this is hard to reproduce. What can be said is that CLion's
  // cargo check invocation does from time to time takes a lot longer than expected
  // in dependent projects even though the bindings were not updated.

  let regenerate_bindings = {
    let generated_bindings = PathBuf::from("src/bindings.rs");
    if !generated_bindings.exists() { true } else {

      let skia_lib_filename =
          if cfg!(windows) { "skia.lib" } else { "libskia.a" };

      let skia_lib = PathBuf::from(&skia_out_dir).join(skia_lib_filename);
      let bindings_cpp_src = PathBuf::from("src/bindings.cpp");
      let us = PathBuf::from("build.rs");
      let config = PathBuf::from("Cargo.toml");

      fn mtime(path: &Path) -> std::time::SystemTime {
        fs::metadata(path).unwrap().modified().unwrap()
      }

      let gen_time = mtime(&generated_bindings);

      mtime(&config) > gen_time
      || mtime(&skia_lib) > gen_time
      || mtime(&bindings_cpp_src) > gen_time
      || mtime(&us) > gen_time
    }
  };

  if regenerate_bindings {
    bindgen_gen(&current_dir_name, &skia_out_dir)
  }
}

fn bindgen_gen(current_dir_name: &str, skia_out_dir: &str) {

  let mut builder = bindgen::Builder::default()
    .generate_inline_functions(true)

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

    .whitelist_type("SkColorSpacePrimaries")
    .whitelist_type("SkVector4")
    .whitelist_type("SkPictureRecorder")
    .whitelist_type("SkAutoCanvasRestore")
    .whitelist_var("SK_Color.*")
    .use_core()
    .clang_arg("-std=c++14");

  let enums = [
    "GrMipMapped", "GrSurfaceOrigin",
    "SkPaint_Flags", "SkPaint_Style", "SkPaint_Cap", "SkPaint_Join",
    "SkGammaNamed",
    "SkColorSpace_RenderTargetGamma", "SkColorSpace_Gamut",
    "SkMatrix44_TypeMask", "SkMatrix_TypeMask", "SkMatrix_ScaleToFit",
    "SkAlphaType", "SkColorType",
    "SkYUVColorSpace",
    "SkPixelGeometry",
    "SkSurfaceProps_Flags",
    "SkBitmap_AllocFlags",
    "SkImage_BitDepth", "SkImage_CachingHint",
    "SkColorChannel",
    "SkYUVAIndex_Index",
    "SkEncodedImageFormat",
    "SkRRect_Type", "SkRRect_Corner",
    "SkRegion_Op",
    "SkFontMetrics_FontMetricsFlags",
    "SkTypeface_SerializeBehavior", "SkTypeface_Encoding",
    "SkFontStyle_Weight", "SkFontStyle_Width", "SkFontStyle_Slant",
    "SkFont_Edging",
    "SkTextEncoding",
    "SkFontHinting",
    "SkVertices_VertexMode", "SkVertices_BuilderFlags",
    "SkPictureRecorder_RecordFlags",
    "SkColorFilter_Flags",
    "SkBlendMode",
    "SkStrokeRec_InitStyle", "SkStrokeRec_Style",
    "SkPathEffect_PointData_PointFlags", "SkPathEffect_DashType",
    "SkBlurStyle",
    "SkCoverageMode",
    "SkFilterQuality",
    "SkPath_Direction", "SkPath_FillType", "SkPath_Convexity", "SkPath_ArcSize", "SkPath_AddPathMode", "SkPath_SegmentMask", "SkPath_Verb",
    "SkCanvas_SaveLayerFlagsSet", "SkCanvas_PointMode", "SkCanvas_SrcRectConstraint",
    "SkClipOp"
  ];

  builder = enums.iter().fold(builder, |b, e| b.rustified_enum(e) );

  let mut cc_build = Build::new();

  builder = builder.header("src/bindings.cpp");

  for include_dir in fs::read_dir("skia/include").expect("Unable to read skia/include") {
    let dir = include_dir.unwrap();
    let include_path = format!("{}/{}", &current_dir_name, &dir.path().to_str().unwrap());
    builder = builder.clang_arg(format!("-I{}", &include_path));
    cc_build.include(&include_path);
  }

  if cfg!(feature="vulkan") {
	builder = builder
      .rustified_enum("VkImageTiling")
      .rustified_enum("VkImageLayout")
      .rustified_enum("VkFormat");
	
    cc_build.define("SK_VULKAN", "1");
    builder = builder.clang_arg("-DSK_VULKAN");
    cc_build.define("SKIA_IMPLEMENTATION", "1");
    builder = builder.clang_arg("-DSKIA_IMPLEMENTATION=1");
  }

  cc_build
    .cpp(true)
    .flag("-std=c++14")
    .file("src/bindings.cpp")
    .out_dir(skia_out_dir)
    .compile("skiabinding");

  let bindings = builder.generate().expect("Unable to generate bindings");

  let out_path = PathBuf::from("src");
  bindings
    .write_to_file(out_path.join("bindings.rs"))
    .expect("Couldn't write bindings!");
}
