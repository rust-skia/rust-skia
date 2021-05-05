use super::image_info;
use crate::{prelude::*, EncodedOrigin, ISize, YUVAIndex};
use skia_bindings as sb;
use skia_bindings::{SkYUVAInfo, SkYUVAInfo_Subsampling};

use std::ptr;

/// Specifies the structure of planes for a YUV image with optional alpha. The actual planar data
/// is not part of this structure and depending on usage is in external textures or pixmaps.
pub type YUVAInfo = Handle<SkYUVAInfo>;
unsafe impl Send for YUVAInfo {}
unsafe impl Sync for YUVAInfo {}

impl NativeDrop for SkYUVAInfo {
    fn drop(&mut self) {
        unsafe { sb::C_SkYUVAInfo_destruct(self) }
    }
}

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
pub use sb::SkYUVAInfo_PlaneConfig as PlaneConfig;

/// UV subsampling is also specified in the enum value names using J:a:b notation (e.g. 4:2:0 is
/// 1/2 horizontal and 1/2 vertical resolution for U and V). If alpha is present it is not sub-
/// sampled. Note that Subsampling values other than k444 are only valid with [PlaneConfig] values
/// that have U and V in different planes than Y (and A, if present).
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Subsampling {
    Unknown = SkYUVAInfo_Subsampling::kUnknown as _,
    S444 = SkYUVAInfo_Subsampling::k444 as _,
    S422 = SkYUVAInfo_Subsampling::k422 as _,
    S420 = SkYUVAInfo_Subsampling::k420 as _,
    S440 = SkYUVAInfo_Subsampling::k440 as _,
    S411 = SkYUVAInfo_Subsampling::k411 as _,
    S410 = SkYUVAInfo_Subsampling::k410 as _,
}

impl NativeTransmutable<SkYUVAInfo_Subsampling> for Subsampling {}

/// Describes how subsampled chroma values are sited relative to luma values.
///
/// Currently only centered siting is supported but will expand to support additional sitings.
pub use sb::SkYUVAInfo_Siting as Siting;

/// Given image dimensions, a planer configuration, subsampling, and origin, determine the expected
/// size of each plane. Returns the expected planes. The input image dimensions are as displayed
/// (after the planes have been transformed to the intended display orientation). The plane
/// dimensions are output as the planes are stored in memory (may be rotated from image dimensions).
pub fn plane_dimensions(
    image_dimensions: impl Into<ISize>,
    config: PlaneConfig,
    subsampling: Subsampling,
    origin: EncodedOrigin,
) -> Vec<ISize> {
    let mut plane_dimensions = [ISize::default(); YUVAInfo::MAX_PLANES];
    let size: usize = unsafe {
        SkYUVAInfo::PlaneDimensions(
            image_dimensions.into().into_native(),
            config,
            subsampling.into_native(),
            origin.into_native(),
            plane_dimensions.native_mut().as_mut_ptr(),
        )
    }
    .try_into()
    .unwrap();

    plane_dimensions[0..size].to_vec()
}

/// Number of planes for a given [PlaneConfig].
pub fn num_planes(config: PlaneConfig) -> usize {
    unsafe { sb::C_SkYUVAInfo_NumPlanes(config) }
        .try_into()
        .unwrap()
}

/// Number of Y, U, V, A channels in the ith plane for a given [PlaneConfig] (or [None] if i is
/// invalid).
pub fn num_channels_in_plane(config: PlaneConfig, i: usize) -> Option<usize> {
    (i < num_planes(config)).if_true_then_some(|| {
        unsafe { sb::C_SkYUVAInfo_NumChannelsInPlane(config, i.try_into().unwrap()) }
            .try_into()
            .unwrap()
    })
}

/// Given a [PlaneConfig] and a set of channel flags for each plane, convert to [YUVAIndex]
/// representation. Fails if channel flags aren't valid for the [PlaneConfig] (i.e. don't have
/// enough channels in a plane).
pub fn get_yuva_indices(
    config: PlaneConfig,
    channel_flags: &[u32; YUVAInfo::MAX_PLANES],
) -> Option<[YUVAIndex; YUVAIndex::INDEX_COUNT]> {
    let mut indices: [YUVAIndex; YUVAIndex::INDEX_COUNT] = Default::default();
    unsafe {
        sb::C_SkYUVAInfo_GetYUVAIndices(config, channel_flags.as_ptr(), indices[0].native_mut())
    }
    .if_true_some(indices)
}

/// Does the [PlaneConfig] have alpha values?
pub fn has_alpha(config: PlaneConfig) -> bool {
    unsafe { sb::SkYUVAInfo_HasAlpha(config) }
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

    /// `dimensions` should specify the size of the full resolution image (after planes have been
    /// oriented to how the image is displayed as indicated by `origin`).
    pub fn new(
        dimensions: impl Into<ISize>,
        config: PlaneConfig,
        subsampling: Subsampling,
        color_space: image_info::YUVColorSpace,
        origin: impl Into<Option<EncodedOrigin>>,
        siting_xy: impl Into<Option<(Siting, Siting)>>,
    ) -> Option<Self> {
        let origin = origin.into().unwrap_or(EncodedOrigin::TopLeft);
        let (siting_x, siting_y) = siting_xy
            .into()
            .unwrap_or((Siting::Centered, Siting::Centered));

        let n = unsafe {
            SkYUVAInfo::new(
                dimensions.into().into_native(),
                config,
                subsampling.into_native(),
                color_space,
                origin.into_native(),
                siting_x,
                siting_y,
            )
        };
        Self::native_is_valid(&n).if_true_then_some(|| Self::from_native_c(n))
    }

    pub fn plane_config(&self) -> PlaneConfig {
        self.native().fPlaneConfig
    }

    pub fn subsampling(&self) -> Subsampling {
        Subsampling::from_native_c(self.native().fSubsampling)
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

    pub fn siting_xy(&self) -> (Siting, Siting) {
        let n = self.native();
        (n.fSitingX, n.fSitingY)
    }

    pub fn origin(&self) -> EncodedOrigin {
        EncodedOrigin::from_native_c(self.native().fOrigin)
    }

    pub fn has_alpha(&self) -> bool {
        has_alpha(self.plane_config())
    }

    /// Returns the dimensions for each plane. Dimensions are as stored in memory, before
    /// transformation to image display space as indicated by [origin(&self)].
    pub fn plane_dimensions(&self) -> Vec<ISize> {
        self::plane_dimensions(
            self.dimensions(),
            self.plane_config(),
            self.subsampling(),
            self.origin(),
        )
    }

    /// Given a per-plane row bytes, determine size to allocate for all planes. Optionally retrieves
    /// the per-plane byte sizes in planeSizes if not [None]. If total size overflows will return
    /// `SIZE_MAX` and set all planeSizes to `SIZE_MAX`.
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
        num_planes(self.plane_config())
    }

    pub fn num_channels_in_plane(&self, i: usize) -> Option<usize> {
        num_channels_in_plane(self.plane_config(), i)
    }

    /// Given a set of channel flags for each plane, converts `plane_config(&self)` to `YUVAIndex`
    /// representation. Fails if the channel flags aren't valid for the [PlaneConfig] (i.e. don't
    /// have enough channels in a plane).
    pub fn to_yuva_indices(&self, channel_flags: &[u32; 4]) -> Option<[YUVAIndex; 4]> {
        get_yuva_indices(self.plane_config(), channel_flags)
    }

    pub(crate) fn native_is_valid(info: &SkYUVAInfo) -> bool {
        info.fPlaneConfig != PlaneConfig::Unknown
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::NativeTransmutable;

    #[test]
    fn test_plane_config_naming() {
        let _ = super::PlaneConfig::Y_U_V;
    }

    #[test]
    fn test_subsampling_layout() {
        super::Subsampling::test_layout();
    }

    #[test]
    fn test_siting_naming() {
        let _ = super::Siting::Centered;
    }
}
