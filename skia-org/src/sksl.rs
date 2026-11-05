//! Ports the examples of Skia's page about SkSL and runtime effects
//! (`site/docs/user/sksl.md` in the Skia repository, sources in `docs/examples/SkSL_*.cpp`).
//!
//! A runtime effect is a shader written in SkSL, Skia's shading language, that Skia compiles at
//! runtime. The examples show how such a shader samples other shaders, which coordinate and color
//! spaces it works in, and how it handles premultiplied alpha.

use std::path;

use skia_safe::{
    AlphaType, BlendMode, Canvas, Color, ColorSpace, ColorType, Data, FilterMode, Image, ImageInfo,
    Matrix, Paint, Rect, RuntimeEffect, SamplingOptions, Shader, TileMode, gradient, named_gamut,
    named_transfer_fn,
    runtime_effect::{ChildPtr, RuntimeShaderBuilder},
    surfaces,
};

use crate::{DrawingDriver, resources};

pub fn draw(driver: &mut impl DrawingDriver, path: &path::Path) {
    let path = &path.join("SkSL");

    driver.draw_image(
        (128, 128),
        path,
        "01-evaluating-image-shader",
        draw_evaluating_image_shader,
    );
    driver.draw_image(
        (128, 128),
        path,
        "02-evaluating-two-shaders",
        draw_evaluating_two_shaders,
    );
    driver.draw_image(
        (128, 128),
        path,
        "03-evaluating-nested-shaders",
        draw_evaluating_nested_shaders,
    );
    driver.draw_image(
        (128, 128),
        path,
        "04-coordinate-spaces",
        draw_coordinate_spaces,
    );
    driver.draw_image((512, 256), path, "05-color-spaces", draw_color_spaces);
    driver.draw_image((128, 64), path, "06-uniforms", draw_uniforms);
    driver.draw_image(
        (384, 128),
        path,
        "07-raw-image-shaders",
        draw_raw_image_shaders,
    );
    driver.draw_image((256, 128), path, "08-linear-srgb", draw_linear_srgb);
    driver.draw_image(
        (200, 200),
        path,
        "09-premultiplied-alpha",
        draw_premultiplied_alpha,
    );
    driver.draw_image((200, 200), path, "10-minified-sksl", draw_minified_sksl);
}

/// Swaps the red and blue channels of an image.
fn draw_evaluating_image_shader(canvas: &Canvas) {
    let sksl = "
        uniform shader image;
        half4 main(float2 coord) {
            return image.eval(coord).bgra;
        }";
    fill(canvas, sksl_shader(sksl, &[image_shader().into()]));
}

/// Multiplies an image with a white-to-black gradient shader.
fn draw_evaluating_two_shaders(canvas: &Canvas) {
    let points = ((0.0, 0.0), (128.0, 0.0));
    let colors = [Color::WHITE.into(), Color::BLACK.into()];
    let colors = gradient::Colors::new_evenly_spaced(&colors, TileMode::Clamp, None);
    let gradient = gradient::Gradient::new(colors, gradient::Interpolation::default());
    let gradient_shader = gradient::shaders::linear_gradient(points, &gradient, None).unwrap();

    let children = [image_shader().into(), gradient_shader.into()];
    fill(canvas, sksl_shader(MULTIPLY_SKSL, &children));
}

/// Multiplies an image with a white-to-black gradient that is itself written in SkSL.
fn draw_evaluating_nested_shaders(canvas: &Canvas) {
    let gradient_sksl = "
        half4 main(float2 coord) {
            float t = coord.x / 128;
            half4 white = half4(1);
            half4 black = half4(0, 0, 0, 1);
            return mix(white, black, t);
        }";
    let children = [
        image_shader().into(),
        sksl_shader(gradient_sksl, &[]).into(),
    ];
    fill(canvas, sksl_shader(MULTIPLY_SKSL, &children));
}

/// Displaces each row of an image horizontally by up to 4 pixels.
fn draw_coordinate_spaces(canvas: &Canvas) {
    let sksl = "
        uniform shader image;
        half4 main(float2 coord) {
            coord.x += sin(coord.y / 3) * 4;
            return image.eval(coord);
        }";
    fill(canvas, sksl_shader(sksl, &[image_shader().into()]));
}

/// Draws an image once as it is, and once with its pixels reinterpreted as Rec. 2020 colors, which
/// makes them more saturated.
fn draw_color_spaces(canvas: &Canvas) {
    let image = resources::mandrill();
    let reinterpreted = image.reinterpret_color_space(rec2020()).unwrap();
    canvas.draw_image(&image, (0, 0), None);
    canvas.draw_image(&reinterpreted, (256, 0), None);
}

/// Draws red into a wide-gamut surface twice: from a plain uniform, which Skia passes unchanged,
/// and from a uniform marked as a color, which Skia converts into the surface's color space.
fn draw_uniforms(canvas: &Canvas) {
    let linear_rec2020 =
        ColorSpace::new_rgb(&named_transfer_fn::LINEAR, &named_gamut::REC2020).unwrap();
    let mut surface =
        surfaces::raster(&wide_gamut_info((128, 64), linear_rec2020), None, None).unwrap();

    let sksl = "
                      uniform vec4 not_a_color;
        layout(color) uniform vec4 color;

        vec4 main(vec2 xy) {
            vec4 c = xy.y < 32 ? not_a_color : color;
            return (c * (xy.x / 128)).rgb1;
        }";
    let mut builder = RuntimeShaderBuilder::new(sksl_effect(sksl));
    builder
        .set_uniform_float("not_a_color", &[1.0, 0.0, 0.0, 1.0])
        .unwrap();
    builder
        .set_uniform_float("color", &[1.0, 0.0, 0.0, 1.0])
        .unwrap();

    let mut paint = Paint::default();
    paint.set_shader(builder.make_shader(&Matrix::new_identity()).unwrap());
    surface.canvas().draw_paint(&paint);
    canvas.draw_image(surface.image_snapshot(), (0, 0), None);
}

/// Lights a sphere from a normal map, three times:
/// 1. On the canvas, which is not color managed. The lighting is correct.
/// 2. On a wide-gamut surface. Skia converts the normals into the surface's color space as if
///    they were colors, so the light comes from the wrong direction.
/// 3. On the same surface, with the normal map sampled by a raw image shader, which skips the
///    color conversion. The lighting is correct again.
fn draw_raw_image_shaders(canvas: &Canvas) {
    let normals = normal_map();
    let mut builder = RuntimeShaderBuilder::new(sksl_effect(
        "
        uniform shader normals;
        vec4 main(vec2 p) {
            vec3 n = normalize(normals.eval(p).xyz * 2 - 1);
            vec3 l = normalize(vec3(-1, -1, 0.5));
            return saturate(dot(n, l)).xxx1;
        }",
    ));
    let mut paint = Paint::default();

    let normals_shader = normals
        .to_shader(None, SamplingOptions::default(), None)
        .unwrap();
    builder.set_child("normals", normals_shader).unwrap();
    paint.set_shader(builder.make_shader(&Matrix::new_identity()).unwrap());
    canvas.draw_rect(Rect::from_wh(128.0, 128.0), &paint);

    let mut surface =
        surfaces::raster(&wide_gamut_info((128, 128), rec2020()), None, None).unwrap();
    surface.canvas().draw_paint(&paint);
    canvas.draw_image(surface.image_snapshot(), (128, 0), None);

    let raw_normals_shader = normals
        .to_raw_shader(None, SamplingOptions::default(), None)
        .unwrap();
    builder.set_child("normals", raw_normals_shader).unwrap();
    paint.set_shader(builder.make_shader(&Matrix::new_identity()).unwrap());
    surface.canvas().draw_paint(&paint);
    canvas.draw_image(surface.image_snapshot(), (256, 0), None);
}

/// Renders the normals of a hemisphere into an image, encoding each normal component from
/// [-1, 1] as a color channel in [0, 1].
fn normal_map() -> Image {
    let sksl = "
        vec4 main(vec2 p) {
            p = (p / 128) * 2 - 1;
            float len2 = dot(p, p);
            vec3 v = (len2 > 1) ? vec3(0, 0, 1) : vec3(p, sqrt(1 - len2));
            return (v * 0.5 + 0.5).xyz1;
        }";
    let mut surface = surfaces::raster_n32_premul((128, 128)).unwrap();
    let mut paint = Paint::default();
    paint
        .set_shader(sksl_shader(sksl, &[]))
        .set_blend_mode(BlendMode::Src);
    surface.canvas().draw_paint(&paint);
    surface.image_snapshot()
}

/// Lights a gray sphere twice: with the lighting math done on the sRGB-encoded color, which gives
/// a too soft falloff, and in linear sRGB, which gives the correct, sharper falloff.
fn draw_linear_srgb(canvas: &Canvas) {
    let sksl = "
        layout(color) uniform vec3 surfaceColor;
        uniform int doLinearLighting;

        vec3 normal_at(vec2 p) {
            p = (p / 128) * 2 - 1;
            float len2 = dot(p, p);
            vec3 n = (len2 > 1) ? vec3(0, 0, 1) : vec3(p, sqrt(1 - len2));
            return normalize(n);
        }

        vec4 main(vec2 p) {
            vec3 n = normal_at(p);
            vec3 l = normalize(vec3(-1, -1, 0.5));
            vec3 C = surfaceColor;

            if (doLinearLighting != 0) { C = toLinearSrgb(C); }
            C *= saturate(dot(n, l));
            if (doLinearLighting != 0) { C = fromLinearSrgb(C); }

            return C.rgb1;
        }";
    let mut builder = RuntimeShaderBuilder::new(sksl_effect(sksl));
    builder
        .set_uniform_float("surfaceColor", &[0.8, 0.8, 0.8])
        .unwrap();

    for linear in [0, 1] {
        builder
            .set_uniform_int("doLinearLighting", &[linear])
            .unwrap();
        let mut paint = Paint::default();
        paint.set_shader(builder.make_shader(&Matrix::new_identity()).unwrap());
        canvas.draw_rect(Rect::from_wh(128.0, 128.0), &paint);
        canvas.translate((128.0, 0.0));
    }
}

/// Fades a color out towards the bottom, correctly premultiplied on the left half and with only its
/// alpha lowered on the right half, which comes out too bright.
fn draw_premultiplied_alpha(canvas: &Canvas) {
    let sksl = "
        const half3 iColor = half3(0, 0.5, 0.75);
        half4 main(float2 coord) {
            float alpha = 1 - (coord.y / 150);
            if (coord.x < 100) {
                return iColor.rgb1 * alpha;
            } else {
                return half4(iColor, alpha);
            }
        }";
    draw_over_gray(canvas, sksl);
}

/// Draws the shader of the premultiplied alpha example from its minified source, which Skia's
/// `sksl-minify` tool produces.
fn draw_minified_sksl(canvas: &Canvas) {
    let sksl = "half4 main(float2 a){float b=1.-a.y*.006666667;if(a.x<100.)return half4(float4\
                (0.,.5,.75,1.)*b);else return half4(half3(0.,.5,.75),half(b));}";
    draw_over_gray(canvas, sksl);
}

/// Multiplies the colors of two child shaders.
const MULTIPLY_SKSL: &str = "
    uniform shader input_1;
    uniform shader input_2;
    half4 main(float2 coord) {
        return input_1.eval(coord) * input_2.eval(coord);
    }";

/// Fills the canvas with gray and blends the shader of `sksl` over it.
fn draw_over_gray(canvas: &Canvas, sksl: &str) {
    canvas.draw_color(Color::GRAY, None);
    fill(canvas, sksl_shader(sksl, &[]));
}

/// Fills the whole canvas with `shader`.
fn fill(canvas: &Canvas, shader: Shader) {
    let mut paint = Paint::default();
    paint.set_shader(shader);
    canvas.draw_paint(&paint);
}

/// The color wheel as a shader with linear filtering, the image that most examples sample.
fn image_shader() -> Shader {
    resources::color_wheel()
        .to_shader(None, FilterMode::Linear, None)
        .unwrap()
}

/// Compiles `sksl` into a shader without uniforms and binds `children` to its `uniform shader`
/// declarations in order.
fn sksl_shader(sksl: &str, children: &[ChildPtr]) -> Shader {
    sksl_effect(sksl)
        .make_shader(Data::new_empty(), children, None)
        .unwrap()
}

/// Compiles `sksl` into a shader effect and panics with the compiler's message if it fails.
fn sksl_effect(sksl: &str) -> RuntimeEffect {
    RuntimeEffect::make_for_shader(sksl, None).unwrap()
}

/// The Rec. 2020 gamut with the sRGB transfer function.
fn rec2020() -> ColorSpace {
    ColorSpace::new_rgb(&named_transfer_fn::SRGB, &named_gamut::REC2020).unwrap()
}

/// The image info of a half-float surface in `color_space`, so the surface holds colors outside
/// the sRGB gamut.
fn wide_gamut_info(size: (i32, i32), color_space: ColorSpace) -> ImageInfo {
    ImageInfo::new(size, ColorType::RGBAF16, AlphaType::Premul, color_space)
}
