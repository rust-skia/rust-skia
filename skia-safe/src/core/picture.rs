use crate::{Canvas, Data, FilterMode, Matrix, Rect, Shader, TileMode, prelude::*};
use skia_bindings::{self as sb, SkPicture, SkRefCntBase};
use std::fmt;

pub type Picture = RCHandle<SkPicture>;
unsafe_send_sync!(Picture);

impl NativeRefCountedBase for SkPicture {
    type Base = SkRefCntBase;
}

impl fmt::Debug for Picture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Picture")
            .field("cull_rect", &self.cull_rect())
            .field("unique_id", &self.unique_id())
            .field("approximate_op_count", &self.approximate_op_count())
            .field("approximate_bytes_used", &self.approximate_bytes_used())
            .finish()
    }
}

impl Picture {
    // TODO: wrap MakeFromStream

    // TODO: may support SkSerialProcs in MakeFromData?

    pub fn from_data(data: &Data) -> Option<Picture> {
        Picture::from_ptr(unsafe { sb::C_SkPicture_MakeFromData(data.native()) })
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Picture> {
        Picture::from_ptr(unsafe {
            sb::C_SkPicture_MakeFromData2(bytes.as_ptr() as _, bytes.len())
        })
    }

    // TODO: AbortCallback and the function that use it.

    /// Replays the drawing commands on the specified canvas. In the case that the commands are
    /// recorded, each command in the [`Picture`] is sent separately to canvas.
    ///
    /// To add a single command to draw [`Picture`] to recording canvas, call
    /// [`crate::Canvas::draw_picture()`] instead.
    ///
    /// - `canvas` receiver of drawing commands
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Picture_playback>
    pub fn playback(&self, canvas: &Canvas) {
        unsafe { sb::C_SkPicture_playback(self.native(), canvas.native_mut()) }
    }

    /// Returns cull [`Rect`] for this picture, passed in when [`Picture`] was created. Returned
    /// [`Rect`] does not specify clipping [`Rect`] for [`Picture`]; cull is hint of [`Picture`]
    /// bounds.
    ///
    /// [`Picture`] is free to discard recorded drawing commands that fall outside cull.
    ///
    /// Returns: bounds passed when [`Picture`] was created
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Picture_cullRect>
    pub fn cull_rect(&self) -> Rect {
        Rect::construct(|r| unsafe { sb::C_SkPicture_cullRect(self.native(), r) })
    }

    pub fn unique_id(&self) -> u32 {
        unsafe { sb::C_SkPicture_uniqueID(self.native()) }
    }

    // TODO: support SkSerialProcs in serialize()?

    /// Returns storage containing [`Data`] describing [`Picture`], using optional custom encoders.
    ///
    /// Returns: storage containing serialized [`Picture`]
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Picture_serialize>
    /// Example (C++): <https://fiddle.skia.org/c/@Picture_serialize_2>
    pub fn serialize(&self) -> Data {
        Data::from_ptr(unsafe { sb::C_SkPicture_serialize(self.native()) }).unwrap()
    }

    /// Returns a placeholder [`Picture`]. Result does not draw, and contains only cull [`Rect`], a
    /// hint of its bounds. Result is immutable; it cannot be changed later. Result identifier is
    /// unique.
    ///
    /// Returned placeholder can be intercepted during playback to insert other commands into
    /// [`crate::Canvas`] draw stream.
    ///
    /// - `cull` placeholder dimensions
    ///
    /// Returns: placeholder with unique identifier
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Picture_MakePlaceholder>
    pub fn new_placeholder(cull: impl AsRef<Rect>) -> Picture {
        Picture::from_ptr(unsafe { sb::C_SkPicture_MakePlaceholder(cull.as_ref().native()) })
            .unwrap()
    }

    /// Returns the approximate number of operations in [`Picture`]. Returned value may be greater
    /// or less than the number of [`crate::Canvas`] calls recorded: some calls may be recorded as
    /// more than one operation, other calls may be optimized away.
    ///
    /// Returns: approximate operation count. If 0 is returned, we say the [`Picture`] is "empty"
    /// meaning its [`Picture::cull_rect()`] is the result of an [`crate::Rect::new_empty()`].
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Picture_approximateOpCount>
    pub fn approximate_op_count(&self) -> usize {
        self.approximate_op_count_nested(false)
    }

    /// Version of [`Picture::approximate_op_count()`] that includes the op-counts of nested
    /// pictures.
    ///
    /// - `nested` if true, include the op-counts of nested pictures as well, else just return count
    ///   the ops in the top-level picture.
    pub fn approximate_op_count_nested(&self, nested: impl Into<Option<bool>>) -> usize {
        let nested = nested.into().unwrap_or(false);
        unsafe {
            sb::C_SkPicture_approximateOpCount(self.native(), nested)
                .try_into()
                .unwrap()
        }
    }

    /// Returns the approximate byte size of [`Picture`]. Does not include large objects referenced
    /// by [`Picture`].
    ///
    /// Returns: approximate size
    ///
    /// Example (C++): <https://fiddle.skia.org/c/@Picture_approximateBytesUsed>
    pub fn approximate_bytes_used(&self) -> usize {
        unsafe {
            let mut value = 0;
            sb::C_SkPicture_approximateBytesUsed(self.native(), &mut value);
            value
        }
    }

    pub fn to_shader<'a, 'b>(
        &self,
        tm: impl Into<Option<(TileMode, TileMode)>>,
        mode: FilterMode,
        local_matrix: impl Into<Option<&'a Matrix>>,
        tile_rect: impl Into<Option<&'b Rect>>,
    ) -> Shader {
        let tm = tm.into();
        let local_matrix = local_matrix.into();
        let tile_rect = tile_rect.into();
        let tmx = tm.map(|tm| tm.0).unwrap_or_default();
        let tmy = tm.map(|tm| tm.1).unwrap_or_default();

        Shader::from_ptr(unsafe {
            sb::C_SkPicture_makeShader(
                self.native(),
                tmx,
                tmy,
                mode,
                local_matrix.native_ptr_or_null(),
                tile_rect.native_ptr_or_null(),
            )
        })
        .unwrap()
    }
}
