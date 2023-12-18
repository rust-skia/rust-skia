#[cfg(not(all(target_os = "windows", feature = "d3d")))]
fn main() {
    println!("This example requires the `d3d` feature to be enabled on Windows.");
    println!("Run it with `cargo run --example d3d-window --features d3d`");
}

#[cfg(all(target_os = "windows", feature = "d3d"))]
fn main() -> anyhow::Result<()> {
    // NOTE: Most of code is from https://github.com/microsoft/windows-rs/blob/02db74cf5c4796d970e6d972cdc7bc3967380079/crates/samples/windows/direct3d12/src/main.rs
    let event_loop = winit::event_loop::EventLoop::new()?;
    let winit_window_builder = winit::window::WindowBuilder::new()
        .with_title("rust-skia-gl-window")
        .with_inner_size(winit::dpi::LogicalSize::new(800, 800));

    let window = winit_window_builder.build(&event_loop)?;

    use anyhow::Result;
    use skia_safe::Color;
    use std::sync::{Arc, Mutex, OnceLock};
    use windows::Win32::Graphics::{
        Direct3D12::D3D12_RESOURCE_STATE_COMMON,
        Dxgi::Common::DXGI_STANDARD_MULTISAMPLE_QUALITY_PATTERN,
    };
    use windows::{
        core::ComInterface,
        Win32::{
            Foundation::HWND,
            Graphics::{
                Direct3D::D3D_FEATURE_LEVEL_11_0,
                Direct3D12::{
                    D3D12CreateDevice, ID3D12CommandQueue, ID3D12DescriptorHeap, ID3D12Device,
                    ID3D12Resource, D3D12_COMMAND_LIST_TYPE_DIRECT, D3D12_COMMAND_QUEUE_DESC,
                    D3D12_COMMAND_QUEUE_FLAG_NONE, D3D12_CPU_DESCRIPTOR_HANDLE,
                    D3D12_DESCRIPTOR_HEAP_DESC, D3D12_DESCRIPTOR_HEAP_TYPE_RTV,
                },
                Dxgi::{
                    Common::{DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_SAMPLE_DESC},
                    CreateDXGIFactory1, IDXGIAdapter1, IDXGIFactory4, IDXGISwapChain3,
                    DXGI_ADAPTER_FLAG, DXGI_ADAPTER_FLAG_NONE, DXGI_ADAPTER_FLAG_SOFTWARE,
                    DXGI_SWAP_CHAIN_DESC1, DXGI_SWAP_EFFECT_FLIP_DISCARD,
                    DXGI_USAGE_RENDER_TARGET_OUTPUT,
                },
            },
        },
    };

    const FRAME_COUNT: u32 = 2;
    let id: u64 = window.id().into();
    let hwnd = HWND(id as isize);

    let factory = unsafe { CreateDXGIFactory1::<IDXGIFactory4>() }?;
    let adapter = get_hardware_adapter(&factory)?;

    let mut device: Option<ID3D12Device> = None;
    unsafe { D3D12CreateDevice(&adapter, D3D_FEATURE_LEVEL_11_0, &mut device) }?;
    let device = device.unwrap();

    let command_queue = unsafe {
        device.CreateCommandQueue::<ID3D12CommandQueue>(&D3D12_COMMAND_QUEUE_DESC {
            Flags: D3D12_COMMAND_QUEUE_FLAG_NONE,
            Type: D3D12_COMMAND_LIST_TYPE_DIRECT,
            ..Default::default()
        })
    }?;

    let swap_chain_desc = DXGI_SWAP_CHAIN_DESC1 {
        BufferCount: FRAME_COUNT,
        Width: window.inner_size().width,
        Height: window.inner_size().height,
        Format: DXGI_FORMAT_R8G8B8A8_UNORM,
        BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
        SwapEffect: DXGI_SWAP_EFFECT_FLIP_DISCARD,
        SampleDesc: DXGI_SAMPLE_DESC {
            Count: 1,
            ..Default::default()
        },
        ..Default::default()
    };

    let swap_chain: IDXGISwapChain3 = unsafe {
        factory.CreateSwapChainForHwnd(&command_queue, hwnd, &swap_chain_desc, None, None)?
    }
    .cast()?;

    let frame_index = unsafe { swap_chain.GetCurrentBackBufferIndex() };

    let rtv_heap: ID3D12DescriptorHeap = unsafe {
        device.CreateDescriptorHeap(&D3D12_DESCRIPTOR_HEAP_DESC {
            NumDescriptors: FRAME_COUNT,
            Type: D3D12_DESCRIPTOR_HEAP_TYPE_RTV,
            ..Default::default()
        })
    }?;

    let rtv_descriptor_size =
        unsafe { device.GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_RTV) } as usize;

    let rtv_handle = D3D12_CPU_DESCRIPTOR_HANDLE {
        ptr: unsafe { rtv_heap.GetCPUDescriptorHandleForHeapStart() }.ptr
            + frame_index as usize * rtv_descriptor_size,
    };

    let render_targets: Vec<ID3D12Resource> = {
        let mut render_targets = vec![];
        for i in 0..FRAME_COUNT {
            let render_target: ID3D12Resource = unsafe { swap_chain.GetBuffer(i)? };
            unsafe {
                device.CreateRenderTargetView(
                    &render_target,
                    None,
                    D3D12_CPU_DESCRIPTOR_HANDLE {
                        ptr: rtv_handle.ptr + i as usize * rtv_descriptor_size,
                    },
                )
            };
            render_targets.push(render_target);
        }
        render_targets
    };

    let backend_context = skia_safe::gpu::d3d::BackendContext {
        adapter,
        device: device.clone(),
        queue: command_queue,
        memory_allocator: None,
        protected_context: skia_safe::gpu::Protected::No,
    };

    let mut context =
        unsafe { skia_safe::gpu::DirectContext::new_d3d(&backend_context, None).unwrap() };

    let mut surfaces = render_targets
        .iter()
        .map(|render_target| {
            let backend_render_target = skia_safe::gpu::BackendRenderTarget::new_d3d(
                (
                    window.inner_size().width as i32,
                    window.inner_size().height as i32,
                ),
                &skia_safe::gpu::d3d::TextureResourceInfo {
                    resource: render_target.clone(),
                    alloc: None,
                    resource_state: D3D12_RESOURCE_STATE_COMMON,
                    format: DXGI_FORMAT_R8G8B8A8_UNORM,
                    sample_count: 1,
                    level_count: 0,
                    sample_quality_pattern: DXGI_STANDARD_MULTISAMPLE_QUALITY_PATTERN,
                    protected: skia_safe::gpu::Protected::No,
                },
            );

            skia_safe::gpu::surfaces::wrap_backend_render_target(
                &mut context,
                &backend_render_target,
                skia_safe::gpu::SurfaceOrigin::BottomLeft,
                skia_safe::ColorType::RGBA8888,
                None,
                None,
            )
            .ok_or(anyhow::anyhow!("wrap_backend_render_target failed"))
        })
        .collect::<Result<Vec<_>>>()?;

    fn get_hardware_adapter(factory: &IDXGIFactory4) -> Result<IDXGIAdapter1> {
        for i in 0.. {
            let adapter = unsafe { factory.EnumAdapters1(i)? };

            let mut desc = Default::default();
            unsafe { adapter.GetDesc1(&mut desc)? };

            if (DXGI_ADAPTER_FLAG(desc.Flags as i32) & DXGI_ADAPTER_FLAG_SOFTWARE)
                != DXGI_ADAPTER_FLAG_NONE
            {
                // Don't select the Basic Render Driver adapter. If you want a
                // software adapter, pass in "/warp" on the command line.
                continue;
            }

            // Check to see whether the adapter supports Direct3D 12, but don't
            // create the actual device yet.
            if unsafe {
                D3D12CreateDevice(
                    &adapter,
                    D3D_FEATURE_LEVEL_11_0,
                    std::ptr::null_mut::<Option<ID3D12Device>>(),
                )
            }
            .is_ok()
            {
                return Ok(adapter);
            }
        }

        unreachable!()
    }

    let mut skia_context = context;

    println!("cool. skia inited");

    let mut render = |state: &State| {
        // I believe you can do better than this. It's very rough code.
        static NEXT_SURFACE_INDEX: OnceLock<Arc<Mutex<usize>>> = OnceLock::new();
        let mut surface_index = NEXT_SURFACE_INDEX
            .get_or_init(|| Arc::new(Mutex::new(0)))
            .lock()
            .unwrap();
        {
            let this_index = *surface_index;
            *surface_index += 1;
            if *surface_index >= surfaces.len() {
                *surface_index = 0;
            }
            let surface = &mut surfaces[this_index];
            let canvas = surface.canvas();

            canvas.clear(skia_safe::Color::BLUE);

            let mut paint = skia_safe::Paint::default();
            paint.set_color(Color::RED);
            paint.set_style(skia_safe::paint::Style::StrokeAndFill);
            paint.set_anti_alias(true);
            paint.set_stroke_width(10.0);

            canvas.draw_rect(
                skia_safe::Rect::from_xywh(state.x, state.y, 200.0, 200.0),
                &paint,
            );
            skia_context.flush_surface(surface);
        }

        skia_context.submit(None);

        unsafe { swap_chain.Present(1, 0).ok().unwrap() };

        // NOTE: If you get some error when you render, you can check it with:
        // unsafe {
        //     device.GetDeviceRemovedReason().ok().unwrap();
        // }
    };

    let mut handle_event = |event, state: &mut State| match event {
        winit::event::WindowEvent::RedrawRequested => {
            render(state);
        }
        winit::event::WindowEvent::KeyboardInput {
            device_id: _,
            event,
            is_synthetic: _,
        } => {
            if event.logical_key
                == winit::keyboard::Key::Named(winit::keyboard::NamedKey::ArrowLeft)
            {
                state.x -= 10.0;
            } else if event.logical_key
                == winit::keyboard::Key::Named(winit::keyboard::NamedKey::ArrowRight)
            {
                state.x += 10.0;
            } else if event.logical_key
                == winit::keyboard::Key::Named(winit::keyboard::NamedKey::ArrowUp)
            {
                state.y -= 10.0;
            } else if event.logical_key
                == winit::keyboard::Key::Named(winit::keyboard::NamedKey::ArrowDown)
            {
                state.y += 10.0;
            }

            render(&state);
        }
        _ => {}
    };

    struct State {
        x: f32,
        y: f32,
    }

    let mut state = State { x: 100.0, y: 100.0 };

    event_loop.run(move |event, _| {
        if let winit::event::Event::WindowEvent { event, .. } = event {
            handle_event(event, &mut state);
        }
    })?;

    Ok(())
}
