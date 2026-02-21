//https://raw.githubusercontent.com/rust-skia/rust-skia/refs/heads/master/skia-safe/examples/vulkan-window/renderer.rs
use ash::vk::Handle;
use std::{ptr, sync::Arc};
use vulkano::{
    device::Queue,
    image::{view::ImageView, ImageUsage},
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass},
    swapchain::{
        acquire_next_image, PresentMode, Surface, Swapchain, SwapchainAcquireFuture,
        SwapchainCreateInfo, SwapchainPresentInfo,
    },
    sync::{self, GpuFuture},
    Validated, VulkanError, VulkanObject,
};

use skia_safe::{
    gpu::{self, backend_render_targets, direct_contexts, surfaces, vk},
    ColorType,
};
use vulkano::format::Format;
use vulkano::image::view::ImageViewCreateInfo;
use vulkano::image::ImageSubresourceRange;
use winit::{dpi::LogicalSize, dpi::PhysicalSize, window::Window};

pub struct VulkanRenderer {
    pub window: Arc<Window>,
    queue: Arc<Queue>,
    swapchain: Arc<Swapchain>,
    framebuffers: Vec<Arc<Framebuffer>>,
    render_pass: Arc<RenderPass>,
    last_render: Option<Box<dyn GpuFuture>>,
    skia_ctx: gpu::DirectContext,
    swapchain_is_valid: bool,
}

impl Drop for VulkanRenderer {
    fn drop(&mut self) {
        // prevent in-flight commands from trying to draw to the window after it's gone
        self.skia_ctx.abandon();
    }
}

impl VulkanRenderer {
    pub fn new(window: Arc<Window>, queue: Arc<Queue>) -> Self {
        // Extract references to key structs from the queue
        let library = queue.device().instance().library();
        let instance = queue.device().instance();
        let device = queue.device();
        let queue = queue.clone();

        // Before we can render to a window, we must first create a `vulkano::swapchain::Surface`
        // object from it, which represents the drawable surface of a window. For that we must wrap
        // the `winit::window::Window` in an `Arc`.
        let surface = Surface::from_window(instance.clone(), window.clone()).unwrap();
        let window_size = window.inner_size();

        // Before we can draw on the surface, we have to create what is called a swapchain.
        // Creating a swapchain allocates the color buffers that will contain the image that will
        // ultimately be visible on the screen. These images are returned alongside the swapchain.
        let (swapchain, _images) = {
            // Querying the capabilities of the surface. When we create the swapchain we can only
            // pass values that are allowed by the capabilities.
            let physical_device = device.physical_device();
            let surface_capabilities = physical_device
                .surface_capabilities(&surface, Default::default())
                .unwrap();

            // Choosing the internal format that the images will have.
            let (image_format, _) = device
                .physical_device()
                .surface_formats(&surface, Default::default())
                .unwrap()
                .into_iter()
                .find(|a| a.0 == Format::B8G8R8A8_UNORM)
                .unwrap();
            let present_modes = physical_device
                .surface_present_modes(&surface, Default::default())
                .unwrap();

            // Please take a look at the docs for the meaning of the parameters we didn't mention.
            Swapchain::new(
                device.clone(),
                surface,
                SwapchainCreateInfo {
                    // Some drivers report an `min_image_count` of 1, but fullscreen mode requires
                    // at least 2. Therefore we must ensure the count is at least 2, otherwise the
                    // program would crash when entering fullscreen mode on those drivers.
                    min_image_count: surface_capabilities.min_image_count.max(2),

                    // The size of the window, only used to initially setup the swapchain.
                    //
                    // NOTE:
                    // On some drivers the swapchain extent is specified by
                    // `surface_capabilities.current_extent` and the swapchain size must use this
                    // extent. This extent is always the same as the window size.
                    //
                    // However, other drivers don't specify a value, i.e.
                    // `surface_capabilities.current_extent` is `None`. These drivers will allow
                    // anything, but the only sensible value is the window size.
                    //
                    // Both of these cases need the swapchain to use the window size, so we just
                    // use that.
                    image_extent: window_size.into(),

                    image_usage: ImageUsage::COLOR_ATTACHMENT,

                    image_format,

                    // The present_mode affects what is commonly known as "vertical sync" or "vsync" for short.
                    // The `Immediate` mode is equivalent to disabling vertical sync, while the others enable
                    // vertical sync in various forms. An important aspect of the present modes is their potential
                    // *latency*: the time between when an image is presented, and when it actually appears on
                    // the display.
                    //
                    // Only `Fifo` is guaranteed to be supported on every device. For the others, you must call
                    // [`surface_present_modes`] to see if they are supported.
                    present_mode: if present_modes.contains(&PresentMode::Immediate) {
                        PresentMode::Immediate
                    } else {
                        PresentMode::Fifo
                    },

                    // The alpha mode indicates how the alpha value of the final image will behave.
                    // For example, you can choose whether the window will be
                    // opaque or transparent.
                    composite_alpha: surface_capabilities
                        .supported_composite_alpha
                        .into_iter()
                        .next()
                        .unwrap(),

                    ..Default::default()
                },
            )
            .unwrap()
        };

        // The next step is to create a *render pass*, which is an object that describes where the
        // output of the graphics pipeline will go. It describes the layout of the images where the
        // colors (and in other use-cases depth and/or stencil information) will be written.
        let render_pass = vulkano::single_pass_renderpass!(
            device.clone(),
            attachments: {
                // `color` is a custom name we give to the first and only attachment.
                color: {
                    // `format: <ty>` indicates the type of the format of the image. This has to be
                    // one of the types of the `vulkano::format` module (or alternatively one of
                    // your structs that implements the `FormatDesc` trait). Here we use the same
                    // format as the swapchain.
                    format: swapchain.image_format(),
                    // `samples: 1` means that we ask the GPU to use one sample to determine the
                    // value of each pixel in the color attachment. We could use a larger value
                    // (multisampling) for antialiasing. An example of this can be found in
                    // msaa-renderpass.rs.
                    samples: 1,
                    // `load_op: DontCare` means that the initial contents of the attachment haven't been
                    // 'cleared' ahead of time (i.e., the pixels haven't all been set to a single color).
                    // This is fine since we'll be filling the entire framebuffer with skia's output
                    load_op: DontCare,
                    // `store_op: Store` means that we ask the GPU to store the output of the draw
                    // in the actual image. We could also ask it to discard the result.
                    store_op: Store,
                },
            },
            pass: {
                // We use the attachment named `color` as the one and only color attachment.
                color: [color],
                // No depth-stencil attachment is indicated with empty brackets.
                depth_stencil: {},
            },
        )
        .unwrap();

        // The render pass we created above only describes the layout of our framebuffers. Before
        // we can draw we also need to create the actual framebuffers.
        //
        // Since we need to draw to multiple images, we are going to create a different framebuffer
        // for each image. We'll wait until the first `prepare_swapchain` call to actually allocate them.
        let framebuffers = vec![];

        // In some situations, the swapchain will become invalid by itself. This includes for
        // example when the window is resized (as the images of the swapchain will no longer match
        // the window's) or, on Android, when the application went to the background and goes back
        // to the foreground.
        //
        // In this situation, acquiring a swapchain image or presenting it will return an error.
        // Rendering to an image of that swapchain will not produce any error, but may or may not
        // work. To continue rendering, we need to recreate the swapchain by creating a new
        // swapchain. Here, we remember that we need to do this for the next loop iteration.
        //
        // Since we haven't allocated framebuffers yet, we'll start in an invalid state to flag that
        // they need to be recreated before we render.
        let swapchain_is_valid = false;

        // In the `draw_and_present` method below we are going to submit commands to the GPU.
        // Submitting a command produces an object that implements the `GpuFuture` trait, which
        // holds the resources for as long as they are in use by the GPU.
        //
        // Destroying the `GpuFuture` blocks until the GPU is finished executing it. In order to
        // avoid that, we store the submission of the previous frame here.
        let last_render = Some(sync::now(device.clone()).boxed());

        // Next we need to connect Skia's gpu backend to the device & queue we've set up.
        let skia_ctx = unsafe {
            // In order to access the vulkan api, we need to give skia some lookup routines
            // to find the expected function pointers for our configured instance & device.
            let get_proc = |gpo| {
                let get_device_proc_addr = instance.fns().v1_0.get_device_proc_addr;

                match gpo {
                    vk::GetProcOf::Instance(instance, name) => {
                        let vk_instance = ash::vk::Instance::from_raw(instance as _);
                        library.get_instance_proc_addr(vk_instance, name)
                    }
                    vk::GetProcOf::Device(device, name) => {
                        let vk_device = ash::vk::Device::from_raw(device as _);
                        get_device_proc_addr(vk_device, name)
                    }
                }
                .map(|f| f as _)
                .unwrap_or_else(|| {
                    println!("Vulkan: failed to resolve {}", gpo.name().to_str().unwrap());
                    ptr::null()
                })
            };

            // We then pass skia_safe references to the whole shebang, resulting in a DirectContext
            // from which we'll be able to get a canvas reference that draws directly to framebuffers
            // on the swapchain.
            let direct_context = direct_contexts::make_vulkan(
                &vk::BackendContext::new(
                    instance.handle().as_raw() as _,
                    device.physical_device().handle().as_raw() as _,
                    device.handle().as_raw() as _,
                    (
                        queue.handle().as_raw() as _,
                        queue.queue_family_index() as usize,
                    ),
                    &get_proc,
                ),
                None,
            )
            .unwrap();

            direct_context
        };

        VulkanRenderer {
            skia_ctx,
            queue,
            window,
            swapchain,
            swapchain_is_valid,
            render_pass,
            framebuffers,
            last_render,
        }
    }

    pub fn invalidate_swapchain(&mut self) {
        // Typically called when the window size changes and we need to recreate framebufffers
        self.swapchain_is_valid = false;
    }

    pub fn prepare_swapchain(&mut self) {
        // It is important to call this function from time to time, otherwise resources
        // will keep accumulating and you will eventually reach an out of memory error.
        // Calling this function polls various fences in order to determine what the GPU
        // has already processed, and frees the resources that are no longer needed.
        if let Some(last_render) = self.last_render.as_mut() {
            last_render.cleanup_finished();
        }

        // Whenever the window resizes we need to recreate everything dependent on the
        // window size. In this example that includes the swapchain & the framebuffers
        let window_size: PhysicalSize<u32> = self.window.inner_size();
        if window_size.width > 0 && window_size.height > 0 && !self.swapchain_is_valid {
            // Use the new dimensions of the window.
            let (new_swapchain, new_images) = self
                .swapchain
                .recreate(SwapchainCreateInfo {
                    image_extent: window_size.into(),
                    image_format: Format::B8G8R8A8_UNORM,
                    ..self.swapchain.create_info()
                })
                .expect("failed to recreate swapchain");

            self.swapchain = new_swapchain;

            // Because framebuffers contains a reference to the old swapchain, we need to
            // recreate framebuffers as well.
            // self.framebuffers = allocate_framebuffers(&new_images, &self.render_pass);
            self.framebuffers = new_images
                .iter()
                .map(|image| {
                    let mut info = ImageViewCreateInfo::default();
                    info.format = Format::B8G8R8A8_UNORM;
                    info.subresource_range =
                        ImageSubresourceRange::from_parameters(info.format, 1, 1);
                    let view = ImageView::new(image.clone(), info).unwrap();
                    Framebuffer::new(
                        self.render_pass.clone(),
                        FramebufferCreateInfo {
                            attachments: vec![view],
                            ..Default::default()
                        },
                    )
                    .unwrap()
                })
                .collect::<Vec<_>>();

            self.swapchain_is_valid = true;
        }
    }

    fn get_next_frame(&mut self) -> Option<(u32, SwapchainAcquireFuture)> {
        // prepare to render by identifying the next framebuffer to draw to and acquiring the
        // GpuFuture that we'll be replacing `last_render` with once we submit the frame
        let (image_index, suboptimal, acquire_future) =
            match acquire_next_image(self.swapchain.clone(), None).map_err(Validated::unwrap) {
                Ok(r) => r,
                Err(VulkanError::OutOfDate) => {
                    self.swapchain_is_valid = false;
                    return None;
                }
                Err(e) => panic!("failed to acquire next image: {e}"),
            };

        // `acquire_next_image` can be successful, but suboptimal. This means that the
        // swapchain image will still work, but it may not display correctly. With some
        // drivers this can be when the window resizes, but it may not cause the swapchain
        // to become out of date.
        if suboptimal {
            self.swapchain_is_valid = false;
        }

        if self.swapchain_is_valid {
            Some((image_index, acquire_future))
        } else {
            None
        }
    }

    pub fn draw_and_present<F>(&mut self, f: F)
    where
        F: FnOnce(&skia_safe::Canvas, LogicalSize<f32>),
    {
        // find the next framebuffer to render into and acquire a new GpuFuture to block on
        let next_frame = self.get_next_frame().or_else(|| {
            // if suboptimal or out-of-date, recreate the swapchain and try once more
            self.prepare_swapchain();
            self.get_next_frame()
        });

        if let Some((image_index, acquire_future)) = next_frame {
            // pull the appropriate framebuffer from the swapchain and attach a skia Surface to it
            let framebuffer = self.framebuffers[image_index as usize].clone();
            let mut surface = surface_for_framebuffer(&mut self.skia_ctx, framebuffer.clone());
            let canvas = surface.canvas();

            // use the display's DPI to convert the window size to logical coords and pre-scale the
            // canvas's matrix to match
            let extent: PhysicalSize<u32> = self.window.inner_size();
            let size: LogicalSize<f32> = extent.to_logical(self.window.scale_factor());

            let scale = (
                (f64::from(extent.width) / size.width as f64) as f32,
                (f64::from(extent.height) / size.height as f64) as f32,
            );
            canvas.reset_matrix();
            canvas.scale(scale);

            // pass the suface's canvas and canvas size to the user-provided callback
            f(canvas, size);

            // flush the canvas's contents to the framebuffer
            self.skia_ctx.flush_and_submit();

            // send the framebuffer to the gpu and display it on screen
            self.last_render = self
                .last_render
                .take()
                .unwrap()
                .join(acquire_future)
                .then_swapchain_present(
                    self.queue.clone(),
                    SwapchainPresentInfo::swapchain_image_index(
                        self.swapchain.clone(),
                        image_index,
                    ),
                )
                .then_signal_fence_and_flush()
                .map(|f| Box::new(f) as _)
                .ok();
        }
    }
}

// Create a skia `Surface` (and its associated `.canvas()`) whose render target is the specified `Framebuffer`.
fn surface_for_framebuffer(
    skia_ctx: &mut gpu::DirectContext,
    framebuffer: Arc<Framebuffer>,
) -> skia_safe::Surface {
    let [width, height] = framebuffer.extent();
    let image_access = &framebuffer.attachments()[0];
    let image_object = image_access.image().handle().as_raw();

    let format = image_access.format();

    let (vk_format, color_type) = match format {
        vulkano::format::Format::B8G8R8A8_UNORM => (
            skia_safe::gpu::vk::Format::B8G8R8A8_UNORM,
            ColorType::BGRA8888,
        ),
        _ => panic!("Unsupported color format {format:?}"),
    };

    let alloc = vk::Alloc::default();
    let image_info = &unsafe {
        vk::ImageInfo::new(
            image_object as _,
            alloc,
            vk::ImageTiling::OPTIMAL,
            vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            vk_format,
            1,
            None,
            None,
            None,
            None,
        )
    };

    let render_target = &backend_render_targets::make_vk(
        (width.try_into().unwrap(), height.try_into().unwrap()),
        image_info,
    );

    surfaces::wrap_backend_render_target(
        skia_ctx,
        render_target,
        gpu::SurfaceOrigin::TopLeft,
        color_type,
        None,
        None,
    )
    .unwrap()
}
