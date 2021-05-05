use crate::artifact;
use crate::drivers::DrawingDriver;
use ash::version::{DeviceV1_0, EntryV1_0, InstanceV1_0};
use ash::vk;
use ash::vk::Handle;
use ash::{Entry, Instance};
use skia_safe::{gpu, Budgeted, Canvas, ImageInfo, Surface};
use std::convert::TryInto;
use std::ffi::{c_void, CString};
use std::os::raw;
use std::path::Path;
use std::ptr;

#[allow(dead_code)]
pub struct Vulkan {
    // ordered for drop order
    context: gpu::DirectContext,
    ash_graphics: AshGraphics,
}

impl DrawingDriver for Vulkan {
    const NAME: &'static str = "vulkan";

    fn new() -> Self {
        let ash_graphics = unsafe { AshGraphics::new("skia-org") };
        let context = {
            let get_proc = |of| unsafe {
                match ash_graphics.get_proc(of) {
                    Some(f) => f as _,
                    None => {
                        println!("resolve of {} failed", of.name().to_str().unwrap());
                        ptr::null()
                    }
                }
            };

            let backend_context = unsafe {
                gpu::vk::BackendContext::new(
                    ash_graphics.instance.handle().as_raw() as _,
                    ash_graphics.physical_device.as_raw() as _,
                    ash_graphics.device.handle().as_raw() as _,
                    (
                        ash_graphics.queue_and_index.0.as_raw() as _,
                        ash_graphics.queue_and_index.1,
                    ),
                    &get_proc,
                )
            };

            gpu::DirectContext::new_vulkan(&backend_context, None).unwrap()
        };

        Self {
            ash_graphics,
            context,
        }
    }

    fn draw_image(
        &mut self,
        (width, height): (i32, i32),
        path: &Path,
        name: &str,
        func: impl Fn(&mut Canvas),
    ) {
        let image_info = ImageInfo::new_n32_premul((width * 2, height * 2), None);
        let mut surface = Surface::new_render_target(
            &mut self.context,
            Budgeted::Yes,
            &image_info,
            None,
            gpu::SurfaceOrigin::TopLeft,
            None,
            false,
        )
        .unwrap();

        artifact::draw_image_on_surface(&mut surface, path, name, func);
    }
}

pub struct AshGraphics {
    pub entry: Entry,
    pub instance: Instance,
    pub physical_device: vk::PhysicalDevice,
    pub device: ash::Device,
    pub queue_and_index: (vk::Queue, usize),
}

impl Drop for AshGraphics {
    fn drop(&mut self) {
        unsafe {
            self.device.device_wait_idle().unwrap();
            self.device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
    }
}

// most code copied from here: https://github.com/MaikKlein/ash/blob/master/examples/src/lib.rs
impl AshGraphics {
    pub fn vulkan_version() -> Option<(usize, usize, usize)> {
        let entry = Entry::new().unwrap();

        let detected_version = entry.try_enumerate_instance_version().unwrap_or(None);

        detected_version.map(|ver| {
            (
                vk::version_major(ver).try_into().unwrap(),
                vk::version_minor(ver).try_into().unwrap(),
                vk::version_patch(ver).try_into().unwrap(),
            )
        })
    }

    pub unsafe fn new(app_name: &str) -> AshGraphics {
        let entry = Entry::new().unwrap();

        let minimum_version = vk::make_version(1, 0, 0);

        let instance: Instance = {
            let api_version = Self::vulkan_version()
                .map(|(major, minor, patch)| {
                    vk::make_version(
                        major.try_into().unwrap(),
                        minor.try_into().unwrap(),
                        patch.try_into().unwrap(),
                    )
                })
                .unwrap_or(minimum_version);

            let app_name = CString::new(app_name).unwrap();
            let layer_names: [&CString; 0] = []; // [CString::new("VK_LAYER_LUNARG_standard_validation").unwrap()];
            let extension_names_raw = []; // extension_names();

            let app_info = vk::ApplicationInfo::builder()
                .application_name(&app_name)
                .application_version(0)
                .engine_name(&app_name)
                .engine_version(0)
                .api_version(api_version);

            let layers_names_raw: Vec<*const raw::c_char> = layer_names
                .iter()
                .map(|raw_name| raw_name.as_ptr())
                .collect();

            let create_info = vk::InstanceCreateInfo::builder()
                .application_info(&app_info)
                .enabled_layer_names(&layers_names_raw)
                .enabled_extension_names(&extension_names_raw);

            entry
                .create_instance(&create_info, None)
                .expect("Failed to create a Vulkan instance.")
        };

        let (physical_device, queue_family_index) = {
            let physical_devices = instance
                .enumerate_physical_devices()
                .expect("Failed to enumerate Vulkan physical devices.");

            physical_devices
                .iter()
                .map(|physical_device| {
                    instance
                        .get_physical_device_queue_family_properties(*physical_device)
                        .iter()
                        .enumerate()
                        .filter_map(|(index, ref info)| {
                            let supports_graphic =
                                info.queue_flags.contains(vk::QueueFlags::GRAPHICS);
                            if supports_graphic {
                                Some((*physical_device, index))
                            } else {
                                None
                            }
                        })
                        .next()
                })
                .filter_map(|v| v)
                .next()
                .expect("Failed to find a suitable Vulkan device.")
        };

        let device: ash::Device = {
            let features = vk::PhysicalDeviceFeatures::default();

            let priorities = [1.0];

            let queue_info = [vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(queue_family_index as _)
                .queue_priorities(&priorities)
                .build()];

            let device_extension_names_raw = [];

            let device_create_info = vk::DeviceCreateInfo::builder()
                .queue_create_infos(&queue_info)
                .enabled_extension_names(&device_extension_names_raw)
                .enabled_features(&features);

            instance
                .create_device(physical_device, &device_create_info, None)
                .unwrap()
        };

        let queue_index: usize = 0;
        let queue: vk::Queue = device.get_device_queue(queue_family_index as _, queue_index as _);

        AshGraphics {
            queue_and_index: (queue, queue_index),
            device,
            physical_device,
            instance,
            entry,
        }
    }

    pub unsafe fn get_proc(
        &self,
        of: gpu::vk::GetProcOf,
    ) -> Option<unsafe extern "system" fn() -> c_void> {
        match of {
            gpu::vk::GetProcOf::Instance(instance, name) => {
                let ash_instance = vk::Instance::from_raw(instance as _);
                self.entry.get_instance_proc_addr(ash_instance, name)
            }
            gpu::vk::GetProcOf::Device(device, name) => {
                let ash_device = vk::Device::from_raw(device as _);
                self.instance.get_device_proc_addr(ash_device, name)
            }
        }
    }
}
