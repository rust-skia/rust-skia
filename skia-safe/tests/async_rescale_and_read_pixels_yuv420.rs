#![cfg(all(
    feature = "ganesh",
    feature = "vulkan",
    not(target_os = "android"),
    not(target_os = "emscripten"),
    not(target_os = "ios")
))]

use std::{cell::RefCell, ffi::c_void, ptr, rc::Rc, slice};

use ash::vk::{self, Handle};
use skia_safe::{
    AlphaType, Color, ColorType, IRect, ISize, ImageInfo, Paint, Rect, Surface, YUVColorSpace,
    gpu::{self, DirectContext, SyncCpu},
    image::{AsyncReadResult, RescaleGamma, RescaleMode},
};

const SIZE: ISize = ISize::new(64, 32);
const RED_YUV: [u8; 3] = [76, 85, 255];
const BLUE_YUV: [u8; 3] = [29, 255, 107];

struct Vulkan {
    entry: ash::Entry,
    instance: ash::Instance,
    physical_device: vk::PhysicalDevice,
    device: ash::Device,
    queue: vk::Queue,
    queue_family_index: u32,
}

impl Vulkan {
    fn new() -> Option<Self> {
        let entry = unsafe { ash::Entry::load() }.ok()?;
        let app_info = vk::ApplicationInfo::default().api_version(vk::API_VERSION_1_1);
        let instance_info = vk::InstanceCreateInfo::default().application_info(&app_info);
        let instance = unsafe { entry.create_instance(&instance_info, None) }.ok()?;

        let device = unsafe { instance.enumerate_physical_devices() }
            .ok()
            .and_then(|physical_devices| {
                physical_devices.into_iter().find_map(|physical_device| {
                    let queue_family_index = unsafe {
                        instance.get_physical_device_queue_family_properties(physical_device)
                    }
                    .iter()
                    .position(|family| family.queue_flags.contains(vk::QueueFlags::GRAPHICS))?
                        as u32;
                    let queue_infos = [vk::DeviceQueueCreateInfo::default()
                        .queue_family_index(queue_family_index)
                        .queue_priorities(&[1.0])];
                    let device_info =
                        vk::DeviceCreateInfo::default().queue_create_infos(&queue_infos);
                    let device =
                        unsafe { instance.create_device(physical_device, &device_info, None) }
                            .ok()?;
                    Some((physical_device, device, queue_family_index))
                })
            });

        let Some((physical_device, device, queue_family_index)) = device else {
            unsafe { instance.destroy_instance(None) };
            return None;
        };
        let queue = unsafe { device.get_device_queue(queue_family_index, 0) };

        Some(Self {
            entry,
            instance,
            physical_device,
            device,
            queue,
            queue_family_index,
        })
    }

    fn direct_context(&self) -> Option<DirectContext> {
        let get_proc = |of: gpu::vk::GetProcOf| -> *const c_void {
            unsafe {
                match of {
                    gpu::vk::GetProcOf::Instance(instance, name) => self
                        .entry
                        .get_instance_proc_addr(vk::Instance::from_raw(instance as _), name),
                    gpu::vk::GetProcOf::Device(device, name) => self
                        .instance
                        .get_device_proc_addr(vk::Device::from_raw(device as _), name),
                }
            }
            .map_or(ptr::null(), |proc| proc as _)
        };

        let backend_context = unsafe {
            gpu::vk::BackendContext::new_builder(
                self.instance.handle().as_raw() as _,
                self.physical_device.as_raw() as _,
                self.device.handle().as_raw() as _,
                (self.queue.as_raw() as _, self.queue_family_index as usize),
                &get_proc,
                Some(gpu::vk::Version::new(1, 1, 0)),
            )
            .build()
        };
        gpu::direct_contexts::make_vulkan(&backend_context, None)
    }
}

impl Drop for Vulkan {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
    }
}

struct Plane {
    row_bytes: usize,
    pixels: Vec<u8>,
}

impl Plane {
    fn at(&self, x: usize, y: usize) -> u8 {
        self.pixels[y * self.row_bytes + x]
    }
}

fn copy_planes(result: &AsyncReadResult, size: ISize) -> Vec<Plane> {
    (0..result.count())
        .map(|i| {
            let height = if i == 0 { size.height } else { size.height / 2 } as usize;
            let row_bytes = result.row_bytes(i);
            let pixels =
                unsafe { slice::from_raw_parts(result.data(i) as *const u8, row_bytes * height) };
            Plane {
                row_bytes,
                pixels: pixels.to_vec(),
            }
        })
        .collect()
}

fn red_and_blue_surface(context: &mut DirectContext) -> Surface {
    let image_info = ImageInfo::new(SIZE, ColorType::RGBA8888, AlphaType::Premul, None);
    let mut surface = gpu::surfaces::render_target(
        context,
        gpu::Budgeted::Yes,
        &image_info,
        None,
        None,
        None,
        None,
        None,
    )
    .unwrap();

    let half = SIZE.width as f32 / 2.0;
    let height = SIZE.height as f32;
    let mut paint = Paint::default();
    paint.set_color(Color::RED);
    surface
        .canvas()
        .draw_rect(Rect::from_xywh(0.0, 0.0, half, height), &paint);
    paint.set_color(Color::BLUE);
    surface
        .canvas()
        .draw_rect(Rect::from_xywh(half, 0.0, half, height), &paint);
    surface
}

fn read_yuv420(
    context: &mut DirectContext,
    surface: &mut Surface,
    dst_size: ISize,
    rescale_mode: RescaleMode,
) -> Option<Vec<Plane>> {
    let planes = Rc::new(RefCell::new(None));
    surface.async_rescale_and_read_pixels_yuv420(
        YUVColorSpace::JPEG,
        None,
        IRect::from_size(SIZE),
        dst_size,
        RescaleGamma::Src,
        rescale_mode,
        {
            let planes = planes.clone();
            move |result| {
                *planes.borrow_mut() = Some(result.map(|result| copy_planes(&result, dst_size)));
            }
        },
    );

    context.submit(SyncCpu::Yes);
    for _ in 0..1000 {
        if planes.borrow().is_some() {
            break;
        }
        context.check_async_work_completion();
    }

    assert_eq!(Rc::strong_count(&planes), 1);
    planes.take().expect("callback was not called")
}

fn assert_yuv(planes: &[Plane], (x, y): (usize, usize), expected: [u8; 3]) {
    let actual = [
        planes[0].at(x, y),
        planes[1].at(x / 2, y / 2),
        planes[2].at(x / 2, y / 2),
    ];
    for (actual, expected) in actual.into_iter().zip(expected) {
        assert!(actual.abs_diff(expected) <= 2, "{actual} != {expected}");
    }
}

fn assert_red_and_blue_planes(planes: &[Plane], size: ISize) {
    let (width, height) = (size.width as usize, size.height as usize);

    assert_eq!(planes.len(), 3);
    assert!(planes[0].row_bytes >= width);
    assert!(planes[1].row_bytes >= width / 2);
    assert!(planes[2].row_bytes >= width / 2);

    for y in [0, height / 2, height - 1] {
        assert_yuv(planes, (0, y), RED_YUV);
        assert_yuv(planes, (width / 4, y), RED_YUV);
        assert_yuv(planes, (width * 3 / 4, y), BLUE_YUV);
        assert_yuv(planes, (width - 1, y), BLUE_YUV);
    }
}

fn with_gpu_surface(test: impl FnOnce(DirectContext, Surface)) {
    let Some(vulkan) = Vulkan::new() else {
        eprintln!("skipped: no Vulkan device available");
        return;
    };
    let Some(mut context) = vulkan.direct_context() else {
        eprintln!("skipped: no Vulkan context available");
        return;
    };
    let surface = red_and_blue_surface(&mut context);
    test(context, surface);
}

#[test]
fn reads_yuv420_planes_from_a_gpu_surface() {
    with_gpu_surface(|mut context, mut surface| {
        let planes = read_yuv420(&mut context, &mut surface, SIZE, RescaleMode::Nearest).unwrap();
        assert_red_and_blue_planes(&planes, SIZE);

        let half = ISize::new(SIZE.width / 2, SIZE.height / 2);
        let planes = read_yuv420(&mut context, &mut surface, half, RescaleMode::Nearest).unwrap();
        assert_red_and_blue_planes(&planes, half);

        let double = ISize::new(SIZE.width * 2, SIZE.height * 2);
        let planes = read_yuv420(
            &mut context,
            &mut surface,
            double,
            RescaleMode::RepeatedLinear,
        )
        .unwrap();
        assert_eq!(planes.len(), 3);
        assert_yuv(&planes, (0, 0), RED_YUV);
        assert_yuv(&planes, (double.width as usize - 1, 0), BLUE_YUV);
    });
}

#[test]
fn fails_for_an_odd_dst_size_on_a_gpu_surface() {
    with_gpu_surface(|mut context, mut surface| {
        let odd = ISize::new(SIZE.width - 1, SIZE.height);
        assert!(read_yuv420(&mut context, &mut surface, odd, RescaleMode::Nearest).is_none());
    });
}

#[test]
fn calls_back_when_the_context_is_dropped_before_submitting() {
    with_gpu_surface(|context, mut surface| {
        let calls = Rc::new(RefCell::new(0));
        surface.async_rescale_and_read_pixels_yuv420(
            YUVColorSpace::JPEG,
            None,
            IRect::from_size(SIZE),
            SIZE,
            RescaleGamma::Src,
            RescaleMode::Nearest,
            {
                let calls = calls.clone();
                move |_| *calls.borrow_mut() += 1
            },
        );

        drop(surface);
        drop(context);

        assert_eq!(*calls.borrow(), 1);
        assert_eq!(Rc::strong_count(&calls), 1);
    });
}

#[test]
fn result_outlives_the_context() {
    with_gpu_surface(|mut context, mut surface| {
        let result = Rc::new(RefCell::new(None));
        surface.async_rescale_and_read_pixels_yuv420(
            YUVColorSpace::JPEG,
            None,
            IRect::from_size(SIZE),
            SIZE,
            RescaleGamma::Src,
            RescaleMode::Nearest,
            {
                let result = result.clone();
                move |read| *result.borrow_mut() = read
            },
        );
        context.submit(SyncCpu::Yes);
        context.check_async_work_completion();

        drop(surface);
        drop(context);

        let result = result.take().unwrap();
        assert_eq!(result.count(), 3);
        assert!(result.row_bytes(0) >= SIZE.width as usize);
    });
}
