use crate::{prelude::*, ColorType, ImageInfo, Pixmap};
use crate::{YUVAInfo, YUVColorSpace};
use skia_bindings as sb;
use skia_bindings::SkYUVAPixmapInfo;
use std::{ffi::c_void, ptr};

pub type YUVAPixmapInfo = Handle<SkYUVAPixmapInfo>;

impl NativeDrop for SkYUVAPixmapInfo {
    fn drop(&mut self) {
        unsafe { sb::C_SkYUVAPixmapInfo_destruct(self) }
    }
}

impl Default for YUVAPixmapInfo {
    /// Default SkYUVAPixmapInfo is invalid.
    fn default() -> Self {
        Self::construct(|pi| unsafe { sb::C_SkYUVAPixmapInfo_construct(pi) })
    }
}

impl NativePartialEq for SkYUVAPixmapInfo {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { sb::C_SkYUVAPixmapInfo_equals(self, rhs) }
    }
}

/// `YUVAInfo` combined with per-plane `ColorTypes` and row bytes. Fully specifies the `Pixmap`s
/// for a YUVA image without the actual pixel memory and data.
impl YUVAPixmapInfo {
    pub const MAX_PLANES: usize = sb::SkYUVAInfo_kMaxPlanes as _;
    pub const DATA_TYPE_CNT: usize = yuva_pixmap_info::DataType::Last as _;

    /// Initializes the `YUVAPixmapInfo` from a `YUVAInfo` with per-plane color types and row bytes.
    /// This will be invalid if the colorTypes aren't compatible with the `YUVAInfo` or if a
    /// rowBytes entry is not valid for the plane dimensions and color type. Color type and
    /// row byte values beyond the number of planes in `YUVAInfo` are ignored. All `ColorType`s
    /// must have the same `DataType` or this will be invalid.
    ///
    /// If `rowBytes` is `None` then bpp*width is assumed for each plane.
    pub fn new(
        info: &YUVAInfo,
        color_types: &[ColorType; Self::MAX_PLANES],
        row_bytes: Option<&[usize; Self::MAX_PLANES]>,
    ) -> Self {
        Self::from_native_c(unsafe {
            SkYUVAPixmapInfo::new(
                info.native(),
                color_types.native().as_ptr(),
                row_bytes.map(|rb| rb.as_ptr()).unwrap_or(ptr::null()),
            )
        })
    }

    /// Like above but uses DefaultColorTypeForDataType to determine each plane's `ColorType`. If
    /// `rowBytes` is `None` then bpp*width is assumed for each plane.
    pub fn from_data_type(
        info: &YUVAInfo,
        data_type: yuva_pixmap_info::DataType,
        row_bytes: Option<&[usize; Self::MAX_PLANES]>,
    ) -> Self {
        Self::from_native_c(unsafe {
            SkYUVAPixmapInfo::new1(
                info.native(),
                data_type,
                row_bytes.map(|rb| rb.as_ptr()).unwrap_or(ptr::null()),
            )
        })
    }

    pub fn yuva_info(&self) -> &YUVAInfo {
        YUVAInfo::from_native_ref(&self.native().fYUVAInfo)
    }

    pub fn yuv_color_space(&self) -> YUVColorSpace {
        self.yuva_info().yuv_color_space()
    }

    /// The number of `Pixmap` planes, `None` if this `YUVAPixmapInfo` is invalid.
    pub fn num_planes(&self) -> Option<usize> {
        self.is_valid()
            .if_true_then_some(|| self.yuva_info().num_planes())
    }

    /// The per-YUV[A] channel data type.
    pub fn data_type(&self) -> yuva_pixmap_info::DataType {
        self.native().fDataType
    }

    /// Row bytes for the ith plane. Returns `None` if `i` >= `numPlanes()` or this `YUVAPixmapInfo` is
    /// invalid.
    pub fn row_bytes(&self, i: usize) -> Option<usize> {
        self.num_planes()
            .filter(|planes| i < *planes)
            .map(|_| unsafe {
                sb::C_SkYUVAPixmapInfo_rowBytes(self.native(), i.try_into().unwrap())
            })
    }

    /// Image info for the ith plane, or `None` if `i` >= `numPlanes()`
    pub fn plane_info(&self, i: usize) -> Option<&ImageInfo> {
        self.num_planes().filter(|planes| i < *planes).map(|_| {
            ImageInfo::from_native_ref(unsafe {
                &*sb::C_SkYUVAPixmapInfo_planeInfo(self.native(), i.try_into().unwrap())
            })
        })
    }

    /// Determine size to allocate for all planes. Optionally retrieves the per-plane sizes in
    /// planeSizes if not `None`. If total size overflows will return SIZE_MAX and set all planeSizes
    /// to SIZE_MAX. Returns `None` if this `YUVAPixmapInfo` is not valid.
    pub fn compute_total_bytes(
        &self,
        plane_sizes: Option<&mut [usize; Self::MAX_PLANES]>,
    ) -> Option<usize> {
        if self.is_valid() {
            Some(unsafe {
                self.native().computeTotalBytes(
                    plane_sizes
                        .map(|ps| ps.as_mut_ptr())
                        .unwrap_or(ptr::null_mut()),
                )
            })
        } else {
            None
        }
    }

    /// Takes an allocation that is assumed to be at least `computeTotalBytes()` in size and configures
    /// the first `numPlanes()` entries in pixmaps array to point into that memory. The remaining
    /// entries of pixmaps are default initialized. Returns `None` if this `YUVAPixmapInfo` not valid.
    pub unsafe fn init_pixmaps_from_single_allocation(
        &self,
        memory: *mut c_void,
    ) -> Option<[Pixmap; Self::MAX_PLANES]> {
        let mut pixmaps: [Pixmap; Self::MAX_PLANES] = Default::default();
        self.native()
            .initPixmapsFromSingleAllocation(memory, pixmaps.native_mut().as_mut_ptr())
            .if_true_some(pixmaps)
    }

    /// Returns `true` if this has been configured with a non-empty dimensioned `YUVAInfo` with
    /// compatible color types and row bytes.
    pub fn is_valid(&self) -> bool {
        unsafe { sb::C_SkYUVAPixmapInfo_isValid(self.native()) }
    }

    /// Is this valid and does it use color types allowed by the passed SupportedDataTypes?
    pub fn is_supported(&self, data_types: &yuva_pixmap_info::SupportedDataTypes) -> bool {
        unsafe { self.native().isSupported(data_types.native()) }
    }
}

pub mod yuva_pixmap_info {
    use crate::{prelude::*, ColorType};
    use skia_bindings as sb;
    use skia_bindings::SkYUVAPixmapInfo_SupportedDataTypes;

    pub use crate::yuva_info::PlanarConfig;

    /// Data type for Y, U, V, and possibly A channels independent of how values are packed into
    /// planes.
    pub use skia_bindings::SkYUVAPixmapInfo_DataType as DataType;

    #[test]
    fn test_data_type_naming() {
        let _ = DataType::Float16;
    }

    pub type SupportedDataTypes = Handle<SkYUVAPixmapInfo_SupportedDataTypes>;

    impl NativeDrop for SkYUVAPixmapInfo_SupportedDataTypes {
        fn drop(&mut self) {
            unsafe { sb::C_SkYUVAPixmapInfo_SupportedDataTypes_destruct(self) }
        }
    }

    impl Default for SupportedDataTypes {
        /// Defaults to nothing supported.
        fn default() -> Self {
            Self::construct(|sdt| unsafe {
                sb::C_SkYUVAPixmapInfo_SupportedDataTypes_Construct(sdt)
            })
        }
    }

    impl SupportedDataTypes {
        #[cfg(feature = "gpu")]
        /// Init based on texture formats supported by the context.
        pub fn from_context(context: &crate::gpu::RecordingContext) -> Self {
            Handle::from_native_c(unsafe {
                sb::SkYUVAPixmapInfo_SupportedDataTypes::new(
                    context.native() as *const _ as *const sb::GrImageContext
                )
            })
        }

        /// All legal combinations of PlanarConfig and DataType are supported.
        pub fn all() -> Self {
            Handle::construct(|sdt| unsafe { sb::C_SkYUVAPixmapInfo_SupportedDataTypes_All(sdt) })
        }

        /// Checks whether there is a supported combination of color types for planes structured
        /// as indicated by PlanarConfig with channel data types as indicated by DataType.
        pub fn supported(&self, pc: PlanarConfig, dt: DataType) -> bool {
            unsafe { sb::C_SkYUVAPixmapInfo_SupportedDataTypes_supported(self.native(), pc, dt) }
        }

        /// Update to add support for pixmaps with numChannel channels where each channel is
        /// represented as DataType.
        pub fn enable_data_type(&mut self, dt: DataType, num_channels: usize) {
            unsafe {
                self.native_mut()
                    .enableDataType(dt, num_channels.try_into().unwrap())
            }
        }
    }

    /// Gets the default SkColorType to use with numChannels channels, each represented as DataType.
    /// Returns `ColorType::Unknown` if no such color type.
    pub fn default_color_type_for_data_type(dt: DataType, num_channels: usize) -> ColorType {
        ColorType::from_native_c(unsafe {
            sb::C_SkYUVAPixmapInfo_DefaultColorTypeForDataType(dt, num_channels.try_into().unwrap())
        })
    }

    /// If the `ColorType` is supported for YUVA pixmaps this will return the number of YUVA channels
    /// that can be stored in a plane of this color type and what the `DataType` is of those channels.
    /// If the `ColorType` is not supported as a YUVA plane the number of channels is reported as 0
    /// and the `DataType` returned should be ignored.
    pub fn num_channels_and_data_type(color_type: ColorType) -> (usize, DataType) {
        let mut data_type = DataType::Float16;
        let channels = unsafe {
            sb::C_SkYUVAPixmapInfo_NumChannelsAndDataType(color_type.into_native(), &mut data_type)
        };
        (channels.try_into().unwrap(), data_type)
    }
}
