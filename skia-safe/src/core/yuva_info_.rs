use super::image_info;
use crate::{prelude::*, EncodedOrigin, ISize};
use skia_bindings as sb;
use skia_bindings::SkYUVAInfo;
use std::ptr;

/// Specifies the structure of planes for a YUV image with optional alpha. The actual planar data
/// is not part of this structure and depending on usage is in external textures or pixmaps.
pub type YUVAInfo = Handle<SkYUVAInfo>;

impl NativeDrop for SkYUVAInfo {
    fn drop(&mut self) {
        unsafe { sb::C_SkYUVAInfo_destruct(self) }
    }
}

impl Default for YUVAInfo {
    fn default() -> Self {
        Self::construct(|yi| unsafe { sb::C_SkYUVAInfo_Construct(yi) })
    }
}

impl NativePartialEq for YUVAInfo {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { sb::C_SkYUVAInfo_equals(self.native(), rhs.native()) }
    }
}

impl YUVAInfo {
    pub const MAX_PLANES: usize = sb::SkYUVAInfo_kMaxPlanes as _;

    /// 'dimensions' should specify the size of the full resolution image (after planes have been
    /// oriented to how the image is displayed as indicated by 'origin').
    pub fn new(
        dimensions: impl Into<ISize>,
        config: yuva_info::PlanarConfig,
        color_space: image_info::YUVColorSpace,
        origin: impl Into<Option<EncodedOrigin>>,
        siting: impl Into<Option<(yuva_info::Siting, yuva_info::Siting)>>,
    ) -> Self {
        let origin = origin.into().unwrap_or(EncodedOrigin::TopLeft);
        let (siting_x, siting_y) = siting
            .into()
            .unwrap_or((yuva_info::Siting::Centered, yuva_info::Siting::Centered));

        Self::from_native_c(unsafe {
            SkYUVAInfo::new(
                dimensions.into().into_native(),
                config,
                color_space,
                origin.into_native(),
                siting_x,
                siting_y,
            )
        })
    }

    pub fn planar_config(&self) -> yuva_info::PlanarConfig {
        return self.native().fPlanarConfig;
    }

    /// Dimensions of the full resolution image (after planes have been oriented to how the image
    /// is displayed as indicated by fOrigin).
    pub fn dimensions(&self) -> ISize {
        ISize::from_native_c(self.native().fDimensions)
    }

    pub fn width(&self) -> i32 {
        self.dimensions().width
    }

    pub fn height(&self) -> i32 {
        self.dimensions().height
    }

    pub fn yuv_color_space(&self) -> image_info::YUVColorSpace {
        self.native().fYUVColorSpace
    }

    pub fn siting_x(&self) -> yuva_info::Siting {
        self.native().fSitingX
    }

    pub fn siting_y(&self) -> yuva_info::Siting {
        self.native().fSitingY
    }

    pub fn origin(&self) -> EncodedOrigin {
        EncodedOrigin::from_native_c(self.native().fOrigin)
    }

    pub fn has_alpha(&self) -> bool {
        yuva_info::has_alpha(self.planar_config())
    }

    /// Returns the number of planes and initializes planeDimensions[0]..planeDimensions[<ret>] to
    /// the expected dimensions for each plane. Dimensions are as stored in memory, before
    /// transformation to image display space as indicated by origin().
    pub fn plane_dimensions(&self, plane_dimensions: &mut [ISize; Self::MAX_PLANES]) -> usize {
        yuva_info::plane_dimensions(
            self.dimensions(),
            self.planar_config(),
            self.origin(),
            plane_dimensions,
        )
    }

    /// Given a per-plane row bytes, determine size to allocate for all planes. Optionally retrieves
    /// the per-plane byte sizes in planeSizes if not null. If total size overflows will return
    /// SIZE_MAX and set all planeSizes to SIZE_MAX.
    pub fn compute_total_bytes(
        &self,
        row_bytes: &[usize; Self::MAX_PLANES],
        plane_sizes: Option<&mut [usize; Self::MAX_PLANES]>,
    ) -> usize {
        unsafe {
            self.native().computeTotalBytes(
                row_bytes.as_ptr(),
                plane_sizes
                    .map(|v| v.as_mut_ptr())
                    .unwrap_or(ptr::null_mut()),
            )
        }
    }

    pub fn num_planes(&self) -> usize {
        yuva_info::num_planes(self.planar_config())
    }

    pub fn num_channels_in_plane(&self, i: usize) -> Option<usize> {
        yuva_info::num_channels_in_plane(self.planar_config(), i)
    }
}

pub mod yuva_info {
    use crate::{prelude::*, EncodedOrigin, ISize, YUVAInfo};
    use skia_bindings as sb;
    use skia_bindings::SkYUVAInfo;

    /// Specifies how YUV (and optionally A) are divided among planes. Planes are separated by
    /// underscores in the enum value names. Within each plane the pixmap/texture channels are
    /// mapped to the YUVA channels in the order specified, e.g. for kY_UV Y is in channel 0 of plane
    /// 0, U is in channel 0 of plane 1, and V is in channel 1 of plane 1. Channel ordering
    /// within a pixmap/texture given the channels it contains:
    /// A:               0:A
    /// Luminance/Gray:  0:Gray
    /// RG               0:R,    1:G
    /// RGB              0:R,    1:G, 2:B
    /// RGBA             0:R,    1:G, 2:B, 3:A
    ///
    /// UV subsampling is also specified in the enum value names using J:a:b notation (e.g. 4:2:0 is
    /// 1/2 horizontal and 1/2 vertical resolution for U and V). A fourth number is added if alpha
    /// is present (always 4 as only full resolution alpha is supported).
    ///
    /// Currently this only has three-plane formats but more will be added as usage and testing of
    ///  this expands.
    pub use sb::SkYUVAInfo_PlanarConfig as PlanarConfig;

    /// Describes how subsampled chroma values are sited relative to luma values.
    ///
    /// Currently only centered siting is supported but will expand to support additional sitings.
    pub use sb::SkYUVAInfo_Siting as Siting;

    /// Given image dimensions, a planar configuration, and origin, determine the expected size of
    /// each plane. Returns the number of expected planes. planeDimensions[0] through
    /// planeDimensons[<ret>] are written. The input image dimensions are as displayed (after the
    /// planes have been transformed to the intended display orientation). The plane dimensions
    /// are output as stored in memory.
    pub fn plane_dimensions(
        image_dimensions: impl Into<ISize>,
        config: PlanarConfig,
        origin: EncodedOrigin,
        plane_dimensions: &mut [ISize; YUVAInfo::MAX_PLANES],
    ) -> usize {
        unsafe {
            SkYUVAInfo::PlaneDimensions(
                image_dimensions.into().into_native(),
                config,
                origin.into_native(),
                plane_dimensions.native_mut().as_mut_ptr(),
            )
        }
        .try_into()
        .unwrap()
    }

    /// Number of planes for a given PlanarConfig.
    pub fn num_planes(config: PlanarConfig) -> usize {
        unsafe { sb::C_SkYUVAInfo_NumPlanes(config) }
            .try_into()
            .unwrap()
    }

    /// Number of Y, U, V, A channels in the ith plane for a given PlanarConfig (or `None` if i is
    /// invalid).
    pub fn num_channels_in_plane(config: PlanarConfig, i: usize) -> Option<usize> {
        (i < num_planes(config)).if_true_then_some(|| {
            unsafe { sb::C_SkYUVAInfo_NumChannelsInPlane(config, i.try_into().unwrap()) }
                .try_into()
                .unwrap()
        })
    }

    /// Does the PlanarConfig have alpha values?
    pub fn has_alpha(config: PlanarConfig) -> bool {
        unsafe { sb::SkYUVAInfo_HasAlpha(config) }
    }
}

#[cfg(test)]

mod tests {
    use crate::yuva_info;

    #[test]
    fn test_planar_config_naming() {
        let _ = yuva_info::PlanarConfig::Y_U_V_410;
    }
    #[test]
    fn test_siting_naming() {
        let _ = yuva_info::Siting::Centered;
    }
}
