//! Ganesh and Graphite both draw meshes.

#![cfg(all(
    feature = "vulkan",
    any(feature = "ganesh", feature = "graphite"),
    not(target_os = "android"),
    not(target_os = "emscripten"),
    not(target_os = "ios")
))]

mod common;

use common::vulkan::Vulkan;
use skia_safe::mesh::{Attribute, ChildPtr, Mode, VertexBuffer, attribute};
use skia_safe::{
    AlphaType, BlendMode, Blender, Canvas, Color, ColorType, Data, ImageInfo, Mesh,
    MeshSpecification, Paint, Rect, shaders,
};

const SIZE: (i32, i32) = (32, 32);
const BACKGROUND: Color = Color::BLUE;

#[cfg(feature = "ganesh")]
#[test]
fn ganesh_draws_a_mesh_with_gpu_buffers() {
    use skia_safe::gpu;

    let Some(vulkan) = Vulkan::new() else {
        eprintln!("skipped: no Vulkan device available");
        return;
    };
    let Some(mut context) = vulkan.direct_context() else {
        eprintln!("skipped: no Vulkan context available");
        return;
    };
    let mut surface = gpu::surfaces::render_target(
        &mut context,
        gpu::Budgeted::Yes,
        &image_info(),
        None,
        None,
        None,
        None,
        None,
    )
    .unwrap();

    let vertex_buffer = gpu::meshes::make_vertex_buffer(&mut context, &left_half_vertices())
        .expect("GPU vertex buffer");
    draw(surface.canvas(), vertex_buffer);
    context.flush_and_submit().unwrap();

    let mut pixels = vec![0u8; row_bytes() * SIZE.1 as usize];
    assert!(surface.read_pixels(&image_info(), &mut pixels, row_bytes(), (0, 0)));
    assert_left_half_is_green(&pixels);
}

#[cfg(feature = "graphite")]
#[test]
fn graphite_draws_a_mesh() {
    use skia_safe::gpu::{self, graphite};
    use skia_safe::meshes;

    let Some(vulkan) = Vulkan::new() else {
        eprintln!("skipped: no Vulkan device available");
        return;
    };
    let Some(mut context) = vulkan.graphite_context() else {
        eprintln!("skipped: no Vulkan context available");
        return;
    };
    let mut recorder = context.make_recorder(None).unwrap();
    let mut surface = graphite::surfaces::render_target(
        &mut recorder,
        &image_info(),
        gpu::Mipmapped::No,
        None,
        None,
    )
    .unwrap();

    draw(
        surface.canvas(),
        meshes::make_vertex_buffer(&left_half_vertices()).unwrap(),
    );
    let mut recording = recorder.snap().unwrap();
    context.insert_recording(&graphite::InsertRecordingInfo::new(&mut recording));
    context.submit_and_wait();

    let mut pixels = vec![0u8; row_bytes() * SIZE.1 as usize];
    assert!(context.read_pixels(
        &mut surface,
        &image_info(),
        &mut pixels,
        row_bytes(),
        (0, 0)
    ));
    assert_left_half_is_green(&pixels);
}

// The mesh color is the yellow child shader multiplied by a cyan uniform; the `Dst` blender keeps
// it and drops the white paint color, so the left half turns green.
fn draw(canvas: &Canvas, vertex_buffer: VertexBuffer) {
    let spec = MeshSpecification::make(
        &[Attribute::new(attribute::Type::Float2, 0, "pos")],
        8,
        &[],
        "Varyings main(const Attributes a) { Varyings v; v.position = a.pos; return v; }",
        "uniform shader child; uniform half4 tint;
        float2 main(const Varyings v, out half4 color) {
            color = child.eval(v.position) * tint; return v.position; }",
    )
    .unwrap();
    let mesh = Mesh::make(
        &spec,
        Mode::TriangleStrip,
        vertex_buffer,
        4,
        0,
        Data::new_copy(&f32_bytes(&[0.0, 1.0, 1.0, 1.0])),
        &[ChildPtr::from(shaders::color(Color::YELLOW))],
        Rect::from_wh(SIZE.0 as f32 / 2.0, SIZE.1 as f32),
    )
    .unwrap();

    canvas.clear(BACKGROUND);
    canvas.draw_mesh(&mesh, Blender::mode(BlendMode::Dst), &Paint::default());
}

fn assert_left_half_is_green(pixels: &[u8]) {
    let pixel = |x: i32, y: i32| {
        let offset = y as usize * row_bytes() + x as usize * 4;
        &pixels[offset..offset + 4]
    };
    assert_eq!(pixel(SIZE.0 / 4, SIZE.1 / 2), [0, 255, 0, 255]);
    assert_eq!(pixel(SIZE.0 * 3 / 4, SIZE.1 / 2), [0, 0, 255, 255]);
}

fn left_half_vertices() -> Vec<u8> {
    let (w, h) = (SIZE.0 as f32 / 2.0, SIZE.1 as f32);
    f32_bytes(&[0.0, 0.0, w, 0.0, 0.0, h, w, h])
}

fn image_info() -> ImageInfo {
    ImageInfo::new(SIZE, ColorType::RGBA8888, AlphaType::Premul, None)
}

fn row_bytes() -> usize {
    SIZE.0 as usize * 4
}

fn f32_bytes(values: &[f32]) -> Vec<u8> {
    values.iter().flat_map(|v| v.to_ne_bytes()).collect()
}
