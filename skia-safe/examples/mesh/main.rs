//! Draws custom meshes ([`skia_safe::Mesh`]) into an offscreen GPU surface, reads the pixels
//! back, asserts them, and saves the result as a PNG.
//!
//! Meshes are drawn by the Ganesh GPU backend only, so this example needs a GL context. It
//! creates one with a hidden window.
//!
//! `cargo run --example mesh --features gl [output.png]`

#[cfg(any(
    target_os = "android",
    target_os = "emscripten",
    target_os = "ios",
    not(feature = "gl")
))]
fn main() {
    println!("To run this example, invoke cargo with --features \"gl\" on a desktop platform.")
}

#[cfg(all(
    not(target_os = "android"),
    not(target_os = "emscripten"),
    not(target_os = "ios"),
    feature = "gl"
))]
fn main() {
    use std::{ffi::CString, num::NonZeroU32, path::PathBuf};

    use glutin::{
        config::{ConfigTemplateBuilder, GlConfig},
        context::{ContextApi, ContextAttributesBuilder},
        display::{GetGlDisplay, GlDisplay},
        prelude::NotCurrentGlContext,
        surface::{SurfaceAttributesBuilder, WindowSurface},
    };
    use glutin_winit::DisplayBuilder;
    use raw_window_handle::HasWindowHandle;
    use winit::{dpi::LogicalSize, event_loop::EventLoop, window::WindowAttributes};

    use skia_safe::{
        Bitmap, BlendMode, Blender, Color, Data, EncodedImageFormat, ImageInfo, Mesh,
        MeshSpecification, Paint, Rect, gpu,
        mesh::{Attribute, ChildPtr, Mode, Varying, attribute, varying},
        meshes, shaders, surfaces,
    };

    const SIZE: (i32, i32) = (420, 900);
    const BACKGROUND: Color = Color::from_rgb(0x1a, 0x1a, 0x2e);

    //
    // A GL context on a hidden window (see the gl-window example).
    //

    let el = EventLoop::new().expect("Failed to create event loop");
    let window_attributes = WindowAttributes::default()
        .with_title("rust-skia-mesh")
        .with_visible(false)
        .with_inner_size(LogicalSize::new(64, 64));
    let (window, gl_config) = DisplayBuilder::new()
        .with_window_attributes(window_attributes.into())
        .build(&el, ConfigTemplateBuilder::new(), |configs| {
            configs
                .reduce(|accum, config| {
                    if config.num_samples() < accum.num_samples() {
                        config
                    } else {
                        accum
                    }
                })
                .unwrap()
        })
        .unwrap();
    let window = window.expect("Could not create window with OpenGL context");
    let raw_window_handle = window
        .window_handle()
        .expect("Failed to retrieve RawWindowHandle")
        .as_raw();

    let context_attributes = ContextAttributesBuilder::new().build(Some(raw_window_handle));
    let fallback_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::Gles(None))
        .build(Some(raw_window_handle));
    let not_current_gl_context = unsafe {
        gl_config
            .display()
            .create_context(&gl_config, &context_attributes)
            .unwrap_or_else(|_| {
                gl_config
                    .display()
                    .create_context(&gl_config, &fallback_context_attributes)
                    .expect("failed to create context")
            })
    };
    let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
        raw_window_handle,
        NonZeroU32::new(64).unwrap(),
        NonZeroU32::new(64).unwrap(),
    );
    let gl_surface = unsafe {
        gl_config
            .display()
            .create_window_surface(&gl_config, &attrs)
            .expect("Could not create gl window surface")
    };
    let _gl_context = not_current_gl_context
        .make_current(&gl_surface)
        .expect("Could not make GL context current");

    let interface = gpu::gl::Interface::new_load_with(|name| {
        if name == "eglGetCurrentDisplay" {
            return std::ptr::null();
        }
        gl_config
            .display()
            .get_proc_address(CString::new(name).unwrap().as_c_str())
    })
    .expect("Could not create interface");
    let mut gr_context =
        gpu::direct_contexts::make_gl(interface, None).expect("Could not create direct context");

    let mut surface = gpu::surfaces::render_target(
        &mut gr_context,
        gpu::Budgeted::Yes,
        &ImageInfo::new_n32_premul(SIZE, None),
        None,
        gpu::SurfaceOrigin::TopLeft,
        None,
        false,
        None,
    )
    .expect("Could not create an offscreen GPU surface");

    //
    // Helpers
    //

    fn f32_bytes(values: &[f32]) -> Vec<u8> {
        values.iter().flat_map(|v| v.to_ne_bytes()).collect()
    }

    fn panel(index: usize) -> Rect {
        Rect::from_xywh(20.0, 20.0 + 220.0 * index as f32, 380.0, 200.0)
    }

    let canvas = surface.canvas();
    canvas.clear(BACKGROUND);
    let mut panel_paint = Paint::default();
    panel_paint.set_color(Color::BLACK);
    for i in 0..4 {
        canvas.draw_rect(panel(i), &panel_paint);
    }

    let mut white = Paint::default();
    white.set_color(Color::WHITE).set_anti_alias(true);

    const POSITION_VS: &str = "Varyings main(const Attributes attrs) {
            Varyings v; v.position = attrs.position; return v; }";
    const YELLOW_FS: &str = "float2 main(const Varyings v, out half4 color) {
            color = half4(1.0, 0.85, 0.2, 1.0); return v.position; }";

    //
    // 1. Solid yellow triangle, a `float2` position only.
    //

    let triangle_spec = MeshSpecification::make(
        &[Attribute::new(attribute::Type::Float2, 0, "position")],
        8,
        &[],
        POSITION_VS,
        YELLOW_FS,
    )
    .expect("triangle spec");
    assert_eq!(triangle_spec.stride(), 8);

    let dest = panel(0);
    let (cx, cy) = (dest.center_x(), dest.center_y());
    let r = dest.width().min(dest.height()) * 0.4;
    let triangle = |cx: f32, cy: f32, r: f32| {
        let vertices = f32_bytes(&[cx, cy - r, cx + r, cy + r, cx - r, cy + r]);
        Mesh::make(
            &triangle_spec,
            Mode::Triangles,
            meshes::make_vertex_buffer(&vertices).expect("triangle vertex buffer"),
            3,
            0,
            None,
            &[],
            Rect::new(cx - r, cy - r, cx + r, cy + r),
        )
        .expect("triangle mesh")
    };
    canvas.draw_mesh(&triangle(cx, cy, r), None, &white);

    //
    // 2. Quad, two triangles, RGB color from a uv varying.
    //

    let uv_attributes = [
        Attribute::new(attribute::Type::Float2, 0, "position"),
        Attribute::new(attribute::Type::Float2, 8, "uv"),
    ];
    let uv_varyings = [Varying::new(varying::Type::Float2, "uv")];
    let quad_spec = MeshSpecification::make(
        &uv_attributes,
        16,
        &uv_varyings,
        "Varyings main(const Attributes a) {
            Varyings v;
            v.position = a.position;
            v.uv = a.uv;
            return v;
        }",
        "float2 main(const Varyings v, out half4 color) {
            color = half4(v.uv.x, v.uv.y, 1.0 - v.uv.x, 1.0);
            return v.position;
        }",
    )
    .expect("uv quad spec");
    assert_eq!(quad_spec.uniform_size(), 0);

    let quad = panel(1).with_inset((20.0, 20.0));
    {
        let (l, t, r, b) = (quad.left, quad.top, quad.right, quad.bottom);
        #[rustfmt::skip]
        let vertices = f32_bytes(&[
            l, t, 0.0, 0.0,
            r, t, 1.0, 0.0,
            r, b, 1.0, 1.0,

            l, t, 0.0, 0.0,
            r, b, 1.0, 1.0,
            l, b, 0.0, 1.0,
        ]);
        let mesh = Mesh::make(
            &quad_spec,
            Mode::Triangles,
            meshes::make_vertex_buffer(&vertices).expect("uv quad vertex buffer"),
            6,
            0,
            None,
            &[],
            quad,
        )
        .expect("uv quad mesh");
        canvas.draw_mesh(&mesh, None, &white);
    }

    //
    // 3. Triangle strip displaced in the vertex program by uniforms.
    //

    const COLS: usize = 24;
    const TIME: f32 = 1.0;
    const AMPLITUDE: f32 = 18.0;
    const FREQUENCY: f32 = 0.05;

    let wave_spec = MeshSpecification::make(
        &uv_attributes,
        16,
        &uv_varyings,
        "uniform float uTime;
        uniform float uAmp;
        uniform float uFreq;
        Varyings main(const Attributes a) {
            Varyings v;
            float dy = sin(a.position.x * uFreq + uTime) * uAmp;
            v.position = a.position + float2(0.0, dy);
            v.uv = a.uv;
            return v;
        }",
        "float2 main(const Varyings v, out half4 color) {
            float band = abs(sin(v.uv.x * 12.0));
            color = half4(band, 0.4 + 0.5 * v.uv.y, 1.0 - band, 1.0);
            return v.position;
        }",
    )
    .expect("wave spec");
    assert_eq!(wave_spec.uniform_size(), 12);
    assert_eq!(
        wave_spec
            .uniforms()
            .iter()
            .map(|u| (u.name(), u.offset()))
            .collect::<Vec<_>>(),
        [("uTime", 0), ("uAmp", 4), ("uFreq", 8)]
    );

    let wave = panel(2).with_inset((20.0, 20.0));
    {
        let vertices: Vec<f32> = (0..=COLS)
            .flat_map(|i| {
                let u = i as f32 / COLS as f32;
                let x = wave.left + u * wave.width();
                [x, wave.top, u, 0.0, x, wave.bottom, u, 1.0]
            })
            .collect();
        let mesh = Mesh::make(
            &wave_spec,
            Mode::TriangleStrip,
            meshes::make_vertex_buffer(&f32_bytes(&vertices)).expect("wave vertex buffer"),
            (COLS + 1) * 2,
            0,
            Data::new_copy(&f32_bytes(&[TIME, AMPLITUDE, FREQUENCY])),
            &[],
            // Inflated to allow the vertex displacement.
            wave.with_outset((0.0, 30.0)),
        )
        .expect("wave mesh");
        canvas.draw_mesh(&mesh, None, &white);
    }

    //
    // 4. Indexed quad with a position and a color attribute, GPU-backed buffers, a child shader,
    //    a uniform, a blender, and a vertex buffer update.
    //

    let color_spec = MeshSpecification::make(
        &[
            Attribute::new(attribute::Type::Float2, 0, "position"),
            Attribute::new(attribute::Type::UByte4_unorm, 8, "color"),
        ],
        12,
        &[Varying::new(varying::Type::Half4, "color")],
        "Varyings main(const Attributes a) {
            Varyings v;
            v.position = a.position;
            v.color = a.color;
            return v;
        }",
        "uniform shader tint;
        uniform half4 gain;
        float2 main(const Varyings v, out half4 color) {
            color = v.color * tint.eval(v.position) * gain;
            return v.position;
        }",
    )
    .expect("color spec");
    assert_eq!(color_spec.children().len(), 1);
    assert_eq!(color_spec.find_child("tint").unwrap().index(), 0);

    let color_quad_vertices = |rect: Rect| -> Vec<u8> {
        [
            (rect.left, rect.top),
            (rect.right, rect.top),
            (rect.left, rect.bottom),
            (rect.right, rect.bottom),
        ]
        .iter()
        .flat_map(|(x, y)| {
            let mut vertex = f32_bytes(&[*x, *y]);
            vertex.extend([255, 128, 0, 255]);
            vertex
        })
        .collect()
    };

    let left_quad = Rect::from_xywh(40.0, panel(3).top + 20.0, 140.0, 160.0);
    let right_quad = left_quad.with_offset((200.0, 0.0));
    {
        let mut vertex_buffer =
            gpu::meshes::make_vertex_buffer(&mut gr_context, &color_quad_vertices(left_quad))
                .expect("GPU vertex buffer");
        assert_eq!(vertex_buffer.size(), 48);
        // The contents of a GPU-backed buffer can't be read back.
        assert!(meshes::copy_vertex_buffer(&vertex_buffer).is_none());
        let indices: Vec<u8> = [0u16, 1, 2, 2, 1, 3]
            .iter()
            .flat_map(|i| i.to_ne_bytes())
            .collect();
        let index_buffer =
            gpu::meshes::make_index_buffer(&mut gr_context, &indices).expect("GPU index buffer");

        let children = [ChildPtr::from(shaders::color(Color::from_rgb(
            128, 255, 255,
        )))];
        let color_quad = |vertex_buffer: &skia_safe::mesh::VertexBuffer, bounds: Rect| {
            Mesh::make_indexed(
                &color_spec,
                Mode::Triangles,
                vertex_buffer,
                4,
                0,
                &index_buffer,
                6,
                0,
                Data::new_copy(&f32_bytes(&[1.0, 0.5, 1.0, 1.0])),
                &children,
                bounds,
            )
            .expect("color quad mesh")
        };

        // Too few uniform bytes are reported as an error.
        let error = Mesh::make(
            &color_spec,
            Mode::TriangleStrip,
            &vertex_buffer,
            4,
            0,
            None,
            &children,
            left_quad,
        )
        .unwrap_err();
        println!("expected mesh error: {error}");

        let mut blue = Paint::default();
        blue.set_color(Color::BLUE);

        // `Dst`: the mesh color only.
        canvas.draw_mesh(
            &color_quad(&vertex_buffer, left_quad),
            Blender::mode(BlendMode::Dst),
            &blue,
        );
        gr_context
            .flush_and_submit()
            .expect("flush the buffer update");

        // Move the quad to the right, and draw it with the paint color only (`Src`).
        assert!(vertex_buffer.update(&mut gr_context, &color_quad_vertices(right_quad), 0));
        // A GPU-backed buffer can't be updated without its context.
        assert!(!vertex_buffer.update(None, &color_quad_vertices(right_quad), 0));
        canvas.draw_mesh(
            &color_quad(&vertex_buffer, right_quad),
            Blender::mode(BlendMode::Src),
            &blue,
        );
    }

    gr_context.flush_and_submit().expect("flush");

    //
    // Read back, assert, and save.
    //

    let mut bitmap = Bitmap::new();
    bitmap.alloc_n32_pixels(SIZE, false);
    assert!(surface.read_pixels_to_bitmap(&bitmap, (0, 0)));

    let expect = |what: &str, (x, y): (f32, f32), (r, g, b): (u8, u8, u8)| {
        let color = bitmap.get_color((x as i32, y as i32));
        let actual = (color.r(), color.g(), color.b());
        let close = |a: u8, b: u8| a.abs_diff(b) <= 6;
        println!("{what}: ({x}, {y}) = {actual:?}, expected {:?}", (r, g, b));
        assert!(
            close(actual.0, r) && close(actual.1, g) && close(actual.2, b) && color.a() == 255,
            "{what}: pixel at ({x}, {y}) is {color:?}"
        );
    };
    let background = (BACKGROUND.r(), BACKGROUND.g(), BACKGROUND.b());
    let black = (0, 0, 0);
    let unit = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;

    expect("outside of all panels", (10.0, 10.0), background);

    let yellow = (255, 217, 51);
    expect("triangle center", (cx, cy), yellow);
    expect("triangle near top", (cx, cy - r + 8.0), yellow);
    expect("triangle bottom left", (cx - r + 8.0, cy + r - 3.0), yellow);
    expect("left of the triangle", (cx - r + 8.0, cy - r + 8.0), black);
    expect("below the triangle", (cx, cy + r + 8.0), black);

    let uv_color = |x: f32, y: f32| {
        let (u, v) = (
            (x + 0.5 - quad.left) / quad.width(),
            (y + 0.5 - quad.top) / quad.height(),
        );
        (unit(u), unit(v), unit(1.0 - u))
    };
    for (x, y) in [
        (quad.center_x(), quad.center_y()),
        (quad.left + 2.0, quad.top + 2.0),
        (quad.right - 3.0, quad.top + 2.0),
        (quad.left + 2.0, quad.bottom - 3.0),
        (quad.right - 3.0, quad.bottom - 3.0),
    ] {
        expect("uv quad", (x, y), uv_color(x, y));
    }
    expect(
        "left of the uv quad",
        (quad.left - 8.0, quad.center_y()),
        black,
    );

    // At the center column of the strip the displacement is `sin(x * uFreq + uTime) * uAmp`.
    let x = wave.center_x();
    let dy = (x * FREQUENCY + TIME).sin() * AMPLITUDE;
    println!("wave displacement at x = {x}: {dy}");
    assert!(dy < -10.0);
    let band = ((x + 0.5 - wave.left) / wave.width() * 12.0).sin().abs();
    let wave_color = |y: f32| {
        let v = (y + 0.5 - (wave.top + dy)) / wave.height();
        (unit(band), unit(0.4 + 0.5 * v), unit(1.0 - band))
    };
    let y = wave.top + dy + 80.0;
    expect("wave center", (x, y), wave_color(y));
    // Above the undisplaced top edge, but covered by the displaced strip.
    let y = wave.top - 8.0;
    expect("wave above its undisplaced top", (x, y), wave_color(y));
    // Inside of the undisplaced strip, but below the displaced one.
    expect(
        "wave below its displaced bottom",
        (x, wave.bottom - 5.0),
        black,
    );

    // (255, 128, 0) * (128, 255, 255) * (1, 0.5, 1)
    expect(
        "color quad (mesh color only)",
        (left_quad.center_x(), left_quad.center_y()),
        (128, 64, 0),
    );
    expect(
        "color quad corner",
        (left_quad.right - 3.0, left_quad.bottom - 3.0),
        (128, 64, 0),
    );
    expect(
        "updated color quad (paint color only)",
        (right_quad.center_x(), right_quad.center_y()),
        (0, 0, 255),
    );
    expect(
        "between the color quads",
        (210.0, left_quad.center_y()),
        black,
    );

    //
    // The raster backend ignores meshes in this Skia version.
    //

    let mut raster = surfaces::raster_n32_premul((200, 200)).expect("raster surface");
    raster.canvas().clear(Color::BLACK);
    raster
        .canvas()
        .draw_mesh(&triangle(100.0, 100.0, 80.0), None, &white);
    let raster_pixel = raster.peek_pixels().unwrap().get_color((100, 100));
    println!(
        "raster backend: triangle center = {:?} -> mesh {}",
        (raster_pixel.r(), raster_pixel.g(), raster_pixel.b()),
        if raster_pixel == Color::BLACK {
            "NOT drawn"
        } else {
            "drawn"
        }
    );

    let path = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../target/mesh-example.png")
        });
    let png = bitmap
        .encode(EncodedImageFormat::PNG, None)
        .expect("PNG encoding failed");
    if let Some(folder) = path.parent() {
        std::fs::create_dir_all(folder).expect("Failed to create the PNG's folder");
    }
    std::fs::write(&path, png).expect("Failed to write the PNG");
    println!("all pixel assertions passed, saved {}", path.display());

    // `DirectContext` must be dropped before the GL context and the window.
    drop(surface);
    gr_context.release_resources_and_abandon();
}
