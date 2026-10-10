//! A headless Vulkan device for GPU tests. Tests skip when no device is available.

use std::ffi::{CStr, c_void};
use std::ptr;

use ash::vk::{self, Handle};
use skia_safe::gpu;

pub struct Vulkan {
    entry: ash::Entry,
    instance: ash::Instance,
    physical_device: vk::PhysicalDevice,
    device: ash::Device,
    queue: vk::Queue,
    queue_family_index: u32,
}

impl Vulkan {
    pub fn new() -> Option<Self> {
        let entry = unsafe { ash::Entry::load() }.ok()?;
        let app_info = vk::ApplicationInfo::default().api_version(vk::API_VERSION_1_1);
        // Portability drivers such as MoltenVK are only enumerated when the instance opts in.
        let portability = has_extension(
            &unsafe { entry.enumerate_instance_extension_properties(None) }.ok()?,
            ash::khr::portability_enumeration::NAME,
        );
        let (instance_extensions, instance_flags) = if portability {
            (
                vec![ash::khr::portability_enumeration::NAME.as_ptr()],
                vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR,
            )
        } else {
            (Vec::new(), vk::InstanceCreateFlags::empty())
        };
        let instance_info = vk::InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_extension_names(&instance_extensions)
            .flags(instance_flags);
        let instance = unsafe { entry.create_instance(&instance_info, None) }.ok()?;

        let device = unsafe { instance.enumerate_physical_devices() }
            .ok()
            .and_then(|physical_devices| {
                physical_devices
                    .into_iter()
                    .find_map(|physical_device| create_device(&instance, physical_device))
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

    #[cfg(feature = "ganesh")]
    pub fn direct_context(&self) -> Option<gpu::DirectContext> {
        self.with_backend_context(|backend_context| {
            gpu::direct_contexts::make_vulkan(backend_context, None)
        })
    }

    #[cfg(feature = "graphite")]
    // Each test target compiles its own copy of this module, and not every one uses Graphite.
    #[allow(dead_code)]
    pub fn graphite_context(&self) -> Option<gpu::graphite::Context> {
        self.with_backend_context(|backend_context| {
            gpu::graphite::vk::context_factory::make_vulkan(backend_context, None)
        })
    }

    // The proc resolver borrows `self`, so the backend context can not outlive this call.
    fn with_backend_context<R>(&self, f: impl FnOnce(&gpu::vk::BackendContext) -> R) -> R {
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
        f(&backend_context)
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

fn create_device(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
) -> Option<(vk::PhysicalDevice, ash::Device, u32)> {
    let queue_family_index =
        unsafe { instance.get_physical_device_queue_family_properties(physical_device) }
            .iter()
            .position(|family| family.queue_flags.contains(vk::QueueFlags::GRAPHICS))?
            as u32;
    let queue_infos = [vk::DeviceQueueCreateInfo::default()
        .queue_family_index(queue_family_index)
        .queue_priorities(&[1.0])];
    // The spec requires enabling VK_KHR_portability_subset wherever it is offered.
    let device_extensions = if has_extension(
        &unsafe { instance.enumerate_device_extension_properties(physical_device) }.ok()?,
        ash::khr::portability_subset::NAME,
    ) {
        vec![ash::khr::portability_subset::NAME.as_ptr()]
    } else {
        Vec::new()
    };
    let device_info = vk::DeviceCreateInfo::default()
        .queue_create_infos(&queue_infos)
        .enabled_extension_names(&device_extensions);
    let device = unsafe { instance.create_device(physical_device, &device_info, None) }.ok()?;
    Some((physical_device, device, queue_family_index))
}

fn has_extension(extensions: &[vk::ExtensionProperties], name: &CStr) -> bool {
    extensions
        .iter()
        .any(|extension| extension.extension_name_as_c_str() == Ok(name))
}
