//! Graphite on an application's `wgpu` device.
//!
//! Skia renders with its Dawn backend, and `skia-wgpu` runs that backend on the `wgpu` crate
//! (ADR 0003), so Skia and the application share one device, its queue, and its textures. The
//! `wgpu` crate of the matching version is [`crate::wgpu`].

use super::{BackendTexture, Context, ContextOptions, Recorder, dawn};
use crate::{AlphaType, ColorSpace, ColorType, Image, Surface, SurfaceProps, wgpu};

pub mod context_factory {
    //! Factory functions to create a Graphite [`Context`] on a `wgpu` device.

    use super::{Context, ContextOptions, dawn, wgpu};

    /// Creates a Graphite context that renders with `device` and submits to `queue`.
    ///
    /// Skia submits its work to the same queue as the application, so a texture that Skia draws
    /// into is ready for every command the application submits after the Graphite submission.
    /// Returns `None` if Skia cannot use the device.
    pub fn make_wgpu<'a>(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        options: impl Into<Option<&'a ContextOptions>>,
    ) -> Option<Context> {
        let handles = skia_wgpu::BackendHandles::new(device, queue);
        // The context takes its own references to the handles, so they can go after creation.
        let backend_context = unsafe {
            dawn::BackendContext::new(handles.instance(), handles.device(), handles.queue())
        };
        dawn::context_factory::make_dawn(&backend_context, options)
    }
}

pub mod surfaces {
    //! Functions to create Graphite [`Surface`]s on `wgpu` textures.

    use super::{
        ColorSpace, ColorType, Recorder, Surface, SurfaceProps, wgpu, wrap_backend_texture,
    };
    use crate::gpu::graphite;

    /// Wraps `texture` as a surface, so that drawing to its canvas renders into the texture.
    ///
    /// `color_type` describes how Skia interprets the texture's format. Returns `None` if the
    /// format has no WebGPU equivalent, or if Skia cannot render to the texture with `color_type`.
    /// The surface keeps the texture alive.
    pub fn wrap_texture(
        recorder: &mut Recorder,
        texture: &wgpu::Texture,
        color_type: ColorType,
        color_space: impl Into<Option<ColorSpace>>,
        surface_props: Option<&SurfaceProps>,
    ) -> Option<Surface> {
        wrap_backend_texture(texture, |backend_texture| {
            graphite::surfaces::wrap_backend_texture(
                recorder,
                backend_texture,
                color_type,
                color_space,
                surface_props,
            )
        })
    }
}

pub mod images {
    //! Functions to create Graphite [`Image`]s from `wgpu` textures.

    use super::{AlphaType, ColorSpace, ColorType, Image, Recorder, wgpu, wrap_backend_texture};
    use crate::gpu::graphite;

    /// Wraps `texture` as an image, so that Skia samples the texture when it draws the image.
    ///
    /// The texture needs `TEXTURE_BINDING` usage. `color_type` and `alpha_type` describe how
    /// Skia interprets the texture's contents. Returns `None` if the format has no WebGPU
    /// equivalent, or if Skia cannot sample the texture with `color_type`. The image keeps the
    /// texture alive.
    ///
    /// Skia submits to the application's queue, so it reads the texture when Graphite submits the
    /// recording that draws the image. The application submits the commands that fill the
    /// texture before that, and none that change it in between.
    pub fn wrap_texture(
        recorder: &mut Recorder,
        texture: &wgpu::Texture,
        color_type: ColorType,
        alpha_type: AlphaType,
        color_space: impl Into<Option<ColorSpace>>,
    ) -> Option<Image> {
        wrap_backend_texture(texture, |backend_texture| {
            graphite::images::wrap_texture(
                recorder,
                backend_texture,
                color_type,
                alpha_type,
                color_space,
            )
        })
    }
}

/// Wraps `texture` as a Dawn backend texture and passes it to `wrap`, which returns the surface or
/// image that wraps the backend texture. Returns `None` if the texture's format has no WebGPU
/// equivalent.
fn wrap_backend_texture<T>(
    texture: &wgpu::Texture,
    wrap: impl FnOnce(&BackendTexture) -> Option<T>,
) -> Option<T> {
    let handle = skia_wgpu::TextureHandle::new(texture).ok()?;
    // The backend texture takes no reference to the texture, and `handle` keeps the texture alive
    // while `wrap` runs. The surface or image that `wrap` returns takes its own reference, so the
    // handle can go afterwards.
    let backend_texture = unsafe { dawn::backend_textures::make_dawn(handle.handle()) };
    wrap(&backend_texture)
}
