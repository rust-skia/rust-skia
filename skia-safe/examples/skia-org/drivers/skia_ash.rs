use std::ffi::{CString, c_void};
use ash::{Instance, Entry};
use ash::vk;
use ash::vk::Handle;
use ash::version::{EntryV1_0, InstanceV1_0, DeviceV1_0};
use skia_safe::graphics::vulkan;

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

        let detected_version =
            entry
                .try_enumerate_instance_version()
                .unwrap_or(None);

        detected_version.map(|ver| {
            (vk_version_major!(ver) as _, vk_version_minor!(ver) as _, vk_version_patch!(ver) as _)
        })
    }

    pub unsafe fn new(app_name: &str) -> AshGraphics {
        let entry = Entry::new().unwrap();

        let minimum_version = vk_make_version!(1, 0, 0);

        let instance: Instance = {

            let api_version =
                    Self::vulkan_version().map(|(major, minor, patch)| {
                        vk_make_version!(major, minor, patch)
                    }).unwrap_or(minimum_version);

            let app_name = CString::new(app_name).unwrap();
            let layer_names : [&CString;0] = []; // [CString::new("VK_LAYER_LUNARG_standard_validation").unwrap()];
            let extension_names_raw = []; // extension_names();

            let app_info = vk::ApplicationInfo::builder()
                .application_name(&app_name)
                .application_version(0)
                .engine_name(&app_name)
                .engine_version(0)
                .api_version(api_version);

            let layers_names_raw: Vec<*const i8> = layer_names
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
                            let supports_graphic = info.queue_flags.contains(vk::QueueFlags::GRAPHICS);
                            match supports_graphic {
                                true => Some((*physical_device, index)),
                                _ => None,
                            }
                        })
                        .nth(0)
                })
                .filter_map(|v| v)
                .nth(0)
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
        let queue: vk::Queue =
            device.get_device_queue(queue_family_index as _, queue_index as _);

        AshGraphics {
            queue_and_index: (queue, queue_index),
            device,
            physical_device,
            instance,
            entry,
        }
    }

    pub unsafe fn get_proc(&self, of: vulkan::GetProcOf)
        -> Option<unsafe extern "system" fn() -> c_void> {

        match of {
            vulkan::GetProcOf::Instance(instance, name) => {
                let ash_instance = vk::Instance::from_raw(instance as _);
                self.entry.get_instance_proc_addr(ash_instance, name)
            },
            vulkan::GetProcOf::Device(device, name) => {
                let ash_device = vk::Device::from_raw(device as _);
                self.instance.get_device_proc_addr(ash_device, name)
            }
        }
    }
}
