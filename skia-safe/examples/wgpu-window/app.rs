//! The window, the cube, and the Skia layer that draws the cube's texture.

use std::pin::pin;
use std::sync::Arc;
use std::task::{Context, Poll, Waker};
use std::time::Instant;

use skia_safe::wgpu;
use wgpu::util::DeviceExt;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

use skia_safe::gpu::graphite;
use skia_safe::{Color, ColorType, M44, V3};

/// The width and height of the texture that Skia draws into.
const TEXTURE_SIZE: u32 = 512;

pub fn run() {
    let event_loop = EventLoop::new().expect("creating the event loop failed");
    event_loop
        .run_app(&mut App::default())
        .expect("running the event loop failed");
}

#[derive(Default)]
struct App {
    state: Option<State>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_none() {
            let window = event_loop
                .create_window(Window::default_attributes().with_title("Skia on wgpu"))
                .expect("creating the window failed");
            self.state = Some(State::new(Arc::new(window)));
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        let Some(state) = &mut self.state else {
            return;
        };
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                state.render();
                state.window.request_redraw();
            }
            _ => {}
        }
    }
}

/// Everything that renders a frame.
struct State {
    skia: SkiaLayer,
    cube: Cube,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
    window: Arc<Window>,
    start: Instant,
}

impl State {
    fn new(window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::default();
        let surface = instance
            .create_surface(window.clone())
            .expect("creating the wgpu surface failed");
        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        }))
        .expect("no wgpu adapter");
        let (device, queue) = block_on(adapter.request_device(&wgpu::DeviceDescriptor::default()))
            .expect("no wgpu device");

        let size = window.inner_size();
        let mut config = surface
            .get_default_config(&adapter, size.width.max(1), size.height.max(1))
            .expect("the surface does not support the adapter");
        // Skia writes sRGB-encoded values into its texture, and the cube shader passes them
        // through. A non-sRGB surface keeps them from being encoded a second time.
        let capabilities = surface.get_capabilities(&adapter);
        if let Some(format) = capabilities.formats.iter().find(|f| !f.is_srgb()) {
            config.format = *format;
        }
        surface.configure(&device, &config);

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("skia texture"),
            size: wgpu::Extent3d {
                width: TEXTURE_SIZE,
                height: TEXTURE_SIZE,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let cube = Cube::new(&device, &config, &texture);
        let skia = SkiaLayer::new(&device, &queue, &texture);

        Self {
            skia,
            cube,
            surface,
            config,
            device,
            queue,
            window,
            start: Instant::now(),
        }
    }

    fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
        self.cube.resize(&self.device, width, height);
    }

    fn render(&mut self) {
        let seconds = self.start.elapsed().as_secs_f32();
        self.skia.draw(logo_frame(seconds));

        let frame = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(frame)
            | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
            wgpu::CurrentSurfaceTexture::Outdated | wgpu::CurrentSurfaceTexture::Lost => {
                self.surface.configure(&self.device, &self.config);
                return;
            }
            wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded => return,
            wgpu::CurrentSurfaceTexture::Validation => panic!("acquiring the frame failed"),
        };
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let aspect = self.config.width as f32 / self.config.height as f32;
        self.cube
            .draw(&self.device, &self.queue, &view, seconds, aspect);
        self.queue.present(frame);
    }
}

/// Skia's Graphite context on the application's device, and the surface on the cube's texture.
struct SkiaLayer {
    surface: skia_safe::Surface,
    recorder: graphite::Recorder,
    context: graphite::Context,
}

impl SkiaLayer {
    fn new(device: &wgpu::Device, queue: &wgpu::Queue, texture: &wgpu::Texture) -> Self {
        let mut context = graphite::wgpu::context_factory::make_wgpu(device, queue, None)
            .expect("no Graphite context for the wgpu device");
        let mut recorder = context.make_recorder(None).expect("no recorder");
        let surface = graphite::wgpu::surfaces::wrap_texture(
            &mut recorder,
            texture,
            ColorType::RGBA8888,
            None,
            None,
        )
        .expect("no Skia surface for the wgpu texture");
        Self {
            surface,
            recorder,
            context,
        }
    }

    /// Draws the logo's animation frame `frame` into the texture and submits the work to the
    /// queue.
    fn draw(&mut self, frame: usize) {
        let canvas = self.surface.canvas();
        canvas.clear(Color::WHITE);
        skia_icon::render_frame(frame, LOGO_FPS, LOGO_BPM, canvas);

        let mut recording = self.recorder.snap().expect("no recording");
        self.context
            .insert_recording(&graphite::InsertRecordingInfo::new(&mut recording));
        self.context.submit(None);
    }
}

/// The frame rate and tempo of the logo animation, as in the `gl-window` example of `skia-safe`.
const LOGO_FPS: usize = 180;
const LOGO_BPM: usize = 60;
/// The number of frames after which the animation repeats seamlessly: one revolution of the
/// chain ring, 360 / (12 · 60/60/180).
const LOGO_FRAMES: usize = 5400;

/// The logo animation frame at `seconds` after the start.
fn logo_frame(seconds: f32) -> usize {
    (seconds * LOGO_FPS as f32) as usize % LOGO_FRAMES
}

/// A cube whose six faces sample the Skia texture.
struct Cube {
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    vertices: wgpu::Buffer,
    indices: wgpu::Buffer,
    index_count: u32,
    uniforms: wgpu::Buffer,
    depth: wgpu::TextureView,
}

impl Cube {
    fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        texture: &wgpu::Texture,
    ) -> Self {
        let (vertex_data, index_data) = cube_geometry();
        let vertices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("cube vertices"),
            contents: &floats_to_bytes(&vertex_data),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let indices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("cube indices"),
            contents: &index_data
                .iter()
                .flat_map(|i| i.to_ne_bytes())
                .collect::<Vec<u8>>(),
            usage: wgpu::BufferUsages::INDEX,
        });
        let uniforms = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("cube uniforms"),
            size: 64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("cube shader"),
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("cube"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &module,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[Some(wgpu::VertexBufferLayout {
                    array_stride: 5 * 4,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2],
                })],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: DEPTH_FORMAT,
                depth_write_enabled: Some(true),
                depth_compare: Some(wgpu::CompareFunction::Less),
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: Default::default(),
            fragment: Some(wgpu::FragmentState {
                module: &module,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(config.format.into())],
            }),
            multiview_mask: None,
            cache: None,
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("cube bindings"),
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniforms.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        Self {
            pipeline,
            bind_group,
            vertices,
            indices,
            index_count: index_data.len() as u32,
            uniforms,
            depth: depth_view(device, config.width, config.height),
        }
    }

    fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        self.depth = depth_view(device, width, height);
    }

    fn draw(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        target: &wgpu::TextureView,
        seconds: f32,
        aspect: f32,
    ) {
        let model = M44::rotate(V3::new(1.0, 1.0, 0.3), seconds * 0.8);
        let view = M44::look_at(
            &V3::new(0.0, 0.0, 4.0),
            &V3::new(0.0, 0.0, 0.0),
            &V3::new(0.0, 1.0, 0.0),
        );
        let projection = perspective(45f32.to_radians(), aspect, 0.1, 100.0);
        let model_view_projection = M44::concat(&projection, &M44::concat(&view, &model));
        let mut columns = [0.0; 16];
        model_view_projection.get_col_major(&mut columns);
        queue.write_buffer(&self.uniforms, 0, &floats_to_bytes(&columns));

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("cube"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: target,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.05,
                            g: 0.05,
                            b: 0.07,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Discard,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.bind_group, &[]);
            pass.set_vertex_buffer(0, self.vertices.slice(..));
            pass.set_index_buffer(self.indices.slice(..), wgpu::IndexFormat::Uint16);
            pass.draw_indexed(0..self.index_count, 0, 0..1);
        }
        queue.submit([encoder.finish()]);
    }
}

const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

/// The cube shader, which samples the Skia texture.
const SHADER: &str = include_str!("cube.wgsl");

fn depth_view(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
    device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("depth"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        })
        .create_view(&wgpu::TextureViewDescriptor::default())
}

/// A right-handed perspective projection into `wgpu`'s clip space, whose depth runs from 0 at
/// `near` to 1 at `far`. Skia's `M44::perspective` maps depth to OpenGL's range of -1 to 1.
fn perspective(fov_y: f32, aspect: f32, near: f32, far: f32) -> M44 {
    let f = 1.0 / (fov_y / 2.0).tan();
    M44::row_major(&[
        f / aspect,
        0.0,
        0.0,
        0.0,
        0.0,
        f,
        0.0,
        0.0,
        0.0,
        0.0,
        far / (near - far),
        near * far / (near - far),
        0.0,
        0.0,
        -1.0,
        0.0,
    ])
}

/// The cube's vertices, as position and texture coordinate, and its triangles. Each face has its
/// own four vertices, so that each shows the whole texture.
fn cube_geometry() -> (Vec<f32>, Vec<u16>) {
    // The corners of each face, counterclockwise as seen from outside the cube.
    const FACES: [[[f32; 3]; 4]; 6] = [
        [[-1., -1., 1.], [1., -1., 1.], [1., 1., 1.], [-1., 1., 1.]],
        [
            [1., -1., -1.],
            [-1., -1., -1.],
            [-1., 1., -1.],
            [1., 1., -1.],
        ],
        [[1., -1., 1.], [1., -1., -1.], [1., 1., -1.], [1., 1., 1.]],
        [
            [-1., -1., -1.],
            [-1., -1., 1.],
            [-1., 1., 1.],
            [-1., 1., -1.],
        ],
        [[-1., 1., 1.], [1., 1., 1.], [1., 1., -1.], [-1., 1., -1.]],
        [
            [-1., -1., -1.],
            [1., -1., -1.],
            [1., -1., 1.],
            [-1., -1., 1.],
        ],
    ];
    const UVS: [[f32; 2]; 4] = [[0., 1.], [1., 1.], [1., 0.], [0., 0.]];
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    for (face, corners) in FACES.iter().enumerate() {
        for (corner, uv) in corners.iter().zip(UVS) {
            vertices.extend_from_slice(corner);
            vertices.extend_from_slice(&uv);
        }
        let first = (face * 4) as u16;
        indices.extend_from_slice(&[first, first + 1, first + 2, first, first + 2, first + 3]);
    }
    (vertices, indices)
}

fn floats_to_bytes(floats: &[f32]) -> Vec<u8> {
    floats.iter().flat_map(|f| f.to_ne_bytes()).collect()
}

/// `wgpu`'s native backends resolve adapter and device requests without polling.
fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = pin!(future);
    let mut context = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(output) = future.as_mut().poll(&mut context) {
            return output;
        }
        std::thread::yield_now();
    }
}
