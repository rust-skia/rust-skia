pub mod pdf {
    use std::{ffi::CString, fmt, io, mem, ptr};

    use skia_bindings::{
        self as sb, SkPDF_AttributeList, SkPDF_DateTime, SkPDF_Metadata, SkPDF_StructureElementNode,
    };

    use crate::{
        interop::{AsStr, RustWStream, SetStr},
        prelude::*,
        scalar, Document, MILESTONE,
    };

    pub type AttributeList = Handle<SkPDF_AttributeList>;
    unsafe_send_sync!(AttributeList);

    impl NativeDrop for SkPDF_AttributeList {
        fn drop(&mut self) {
            unsafe { sb::C_SkPDF_AttributeList_destruct(self) }
        }
    }

    impl Default for AttributeList {
        fn default() -> Self {
            AttributeList::from_native_c(unsafe { SkPDF_AttributeList::new() })
        }
    }

    impl fmt::Debug for AttributeList {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("AttributeList").finish()
        }
    }

    /// Attributes for nodes in the PDF tree.
    ///
    /// Each attribute must have an owner (e.g. "Layout", "List", "Table", etc)
    /// and an attribute name (e.g. "BBox", "RowSpan", etc.) from PDF32000_2008 14.8.5,
    /// and then a value of the proper type according to the spec.
    impl AttributeList {
        pub fn append_int(
            &mut self,
            owner: impl AsRef<str>,
            name: impl AsRef<str>,
            value: i32,
        ) -> &mut Self {
            let owner = CString::new(owner.as_ref()).unwrap();
            let name = CString::new(name.as_ref()).unwrap();
            unsafe {
                self.native_mut()
                    .appendInt(owner.as_ptr(), name.as_ptr(), value)
            }
            self
        }

        pub fn append_float(
            &mut self,
            owner: impl AsRef<str>,
            name: impl AsRef<str>,
            value: f32,
        ) -> &mut Self {
            let owner = CString::new(owner.as_ref()).unwrap();
            let name = CString::new(name.as_ref()).unwrap();
            unsafe {
                self.native_mut()
                    .appendFloat(owner.as_ptr(), name.as_ptr(), value)
            }
            self
        }

        pub fn append_float_array(
            &mut self,
            owner: impl AsRef<str>,
            name: impl AsRef<str>,
            value: &[f32],
        ) -> &mut Self {
            let owner = CString::new(owner.as_ref()).unwrap();
            let name = CString::new(name.as_ref()).unwrap();
            unsafe {
                sb::C_SkPDF_AttributeList_appendFloatArray(
                    self.native_mut(),
                    owner.as_ptr(),
                    name.as_ptr(),
                    value.as_ptr(),
                    value.len(),
                )
            }
            self
        }
    }

    #[repr(transparent)]
    pub struct StructureElementNode(ptr::NonNull<SkPDF_StructureElementNode>);

    impl NativeAccess for StructureElementNode {
        type Native = SkPDF_StructureElementNode;

        fn native(&self) -> &SkPDF_StructureElementNode {
            unsafe { self.0.as_ref() }
        }
        fn native_mut(&mut self) -> &mut SkPDF_StructureElementNode {
            unsafe { self.0.as_mut() }
        }
    }

    impl Drop for StructureElementNode {
        fn drop(&mut self) {
            unsafe { sb::C_SkPDF_StructureElementNode_delete(self.native_mut()) }
        }
    }

    impl Default for StructureElementNode {
        fn default() -> Self {
            Self(ptr::NonNull::new(unsafe { sb::C_SkPDF_StructureElementNode_new() }).unwrap())
        }
    }

    impl fmt::Debug for StructureElementNode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("StructureElementNode")
                .field("type_string", &self.type_string())
                .field("child_vector", &self.child_vector())
                .field("node_id", &self.node_id())
                .field("attributes", &self.attributes())
                .field("alt", &self.alt())
                .field("lang", &self.lang())
                .finish()
        }
    }

    /// A node in a PDF structure tree, giving a semantic representation
    /// of the content.  Each node ID is associated with content
    /// by passing the [`crate::Canvas`] and node ID to [`Self::set_node_id()`] when drawing.
    /// NodeIDs should be unique within each tree.
    impl StructureElementNode {
        pub fn new(type_string: impl AsRef<str>) -> Self {
            let mut node = Self::default();
            node.set_type_string(type_string);
            node
        }

        pub fn set_type_string(&mut self, type_string: impl AsRef<str>) -> &mut Self {
            self.native_mut().fTypeString.set_str(type_string);
            self
        }

        pub fn type_string(&self) -> &str {
            self.native().fTypeString.as_str()
        }

        pub fn set_child_vector(
            &mut self,
            mut child_vector: Vec<StructureElementNode>,
        ) -> &mut Self {
            // strategy is to move them out by setting them to nullptr (drop() will handle a nullptr on the rust side)
            unsafe {
                sb::C_SkPDF_StructureElementNode_setChildVector(
                    self.native_mut(),
                    child_vector.as_mut_ptr() as _,
                    child_vector.len(),
                )
            }
            self
        }

        pub fn append_child(&mut self, node: StructureElementNode) -> &mut Self {
            unsafe {
                sb::C_SkPDF_StructElementNode_appendChild(self.native_mut(), node.0.as_ptr());
            }
            mem::forget(node);
            self
        }

        pub fn child_vector(&self) -> &[StructureElementNode] {
            let mut ptr = ptr::null_mut();
            unsafe {
                let len = sb::C_SkPDF_StructureElementNode_getChildVector(self.native(), &mut ptr);
                safer::from_raw_parts(ptr as *const StructureElementNode, len)
            }
        }

        pub fn set_node_id(&mut self, node_id: i32) -> &mut Self {
            self.native_mut().fNodeId = node_id;
            self
        }

        pub fn node_id(&self) -> i32 {
            self.native().fNodeId
        }

        pub fn attributes(&self) -> &AttributeList {
            AttributeList::from_native_ref(&self.native().fAttributes)
        }

        pub fn attributes_mut(&mut self) -> &mut AttributeList {
            AttributeList::from_native_ref_mut(&mut self.native_mut().fAttributes)
        }

        pub fn set_alt(&mut self, alt: impl AsRef<str>) -> &mut Self {
            self.native_mut().fAlt.set_str(alt);
            self
        }

        pub fn alt(&self) -> &str {
            self.native().fAlt.as_str()
        }

        pub fn set_lang(&mut self, lang: impl AsRef<str>) -> &mut Self {
            self.native_mut().fLang.set_str(lang);
            self
        }

        pub fn lang(&self) -> &str {
            self.native().fLang.as_str()
        }
    }

    #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
    #[repr(C)]
    pub struct DateTime {
        /// The number of minutes that this is ahead of or behind UTC.
        pub time_zone_minutes: i16,
        /// e.g. 2005
        pub year: u16,
        /// 1..12
        pub month: u8,
        /// 0..6, 0==Sunday
        pub day_of_week: u8,
        /// 1..31
        pub day: u8,
        /// 0..23
        pub hour: u8,
        /// 0..59
        pub minute: u8,
        /// 0..59
        pub second: u8,
    }

    native_transmutable!(SkPDF_DateTime, DateTime, date_time_layout);

    /// Optional metadata to be passed into the PDF factory function.
    #[derive(Debug)]
    pub struct Metadata {
        /// The document's title.
        pub title: String,
        /// The name of the person who created the document.
        pub author: String,
        /// The subject of the document.
        pub subject: String,
        /// Keywords associated with the document. Commas may be used to delineate keywords within
        /// the string.
        pub keywords: String,
        /// If the document was converted to PDF from another format, the name of the conforming
        /// product that created the original document from which it was converted.
        pub creator: String,
        /// The product that is converting this document to PDF.
        pub producer: String,
        /// The date and time the document was created.
        pub creation: Option<DateTime>,
        /// The date and time the document was most recently modified.
        pub modified: Option<DateTime>,
        /// The natural language of the text in the PDF. If `lang` is empty, the root
        /// StructureElementNode::lang will be used (if not empty). Text not in
        /// this language should be marked with StructureElementNode::lang.
        pub lang: String,
        /// The DPI (pixels-per-inch) at which features without native PDF support
        /// will be rasterized (e.g. draw image with perspective, draw text with
        /// perspective, ...)  A larger DPI would create a PDF that reflects the
        /// original intent with better fidelity, but it can make for larger PDF
        /// files too, which would use more memory while rendering, and it would be
        /// slower to be processed or sent online or to printer.
        pub raster_dpi: Option<scalar>,
        /// If `true`, include XMP metadata, a document UUID, and `s_rgb` output intent
        /// information.  This adds length to the document and makes it
        /// non-reproducible, but are necessary features for PDF/A-2b conformance
        pub pdf_a: bool,
        /// Encoding quality controls the trade-off between size and quality. By default this is set
        /// to 101 percent, which corresponds to lossless encoding. If this value is set to a value
        /// <= 100, and the image is opaque, it will be encoded (using JPEG) with that quality
        /// setting.
        pub encoding_quality: Option<i32>,

        pub structure_element_tree_root: Option<StructureElementNode>,

        /// PDF streams may be compressed to save space.
        /// Use this to specify the desired compression vs time tradeoff.
        pub compression_level: CompressionLevel,
    }

    impl Default for Metadata {
        fn default() -> Self {
            Self {
                title: Default::default(),
                author: Default::default(),
                subject: Default::default(),
                keywords: Default::default(),
                creator: Default::default(),
                producer: format!("Skia/PDF m{}", MILESTONE),
                creation: Default::default(),
                modified: Default::default(),
                lang: Default::default(),
                raster_dpi: Default::default(),
                pdf_a: Default::default(),
                encoding_quality: Default::default(),
                structure_element_tree_root: None,
                compression_level: Default::default(),
            }
        }
    }

    pub type CompressionLevel = skia_bindings::SkPDF_Metadata_CompressionLevel;
    variant_name!(CompressionLevel::HighButSlow);

    /// Create a PDF-backed document.
    ///
    /// PDF pages are sized in point units. 1 pt == 1/72 inch == 127/360 mm.
    ///
    /// * `metadata` - a PDFmetadata object.  Any fields may be left empty.
    ///
    /// @returns `None` if there is an error, otherwise a newly created PDF-backed [`Document`].
    pub fn new_document<'a>(
        writer: &'a mut impl io::Write,
        metadata: Option<&Metadata>,
    ) -> Document<'a> {
        let mut md = InternalMetadata::default();
        if let Some(metadata) = metadata {
            let internal = md.native_mut();
            internal.fTitle.set_str(&metadata.title);
            internal.fAuthor.set_str(&metadata.author);
            internal.fSubject.set_str(&metadata.subject);
            internal.fKeywords.set_str(&metadata.keywords);
            internal.fCreator.set_str(&metadata.creator);
            internal.fProducer.set_str(&metadata.producer);
            if let Some(creation) = metadata.creation {
                internal.fCreation = creation.into_native();
            }
            if let Some(modified) = metadata.modified {
                internal.fModified = modified.into_native();
            }
            internal.fLang.set_str(&metadata.lang);
            if let Some(raster_dpi) = metadata.raster_dpi {
                internal.fRasterDPI = raster_dpi;
            }
            internal.fPDFA = metadata.pdf_a;
            if let Some(encoding_quality) = metadata.encoding_quality {
                internal.fEncodingQuality = encoding_quality
            }
            internal.fCompressionLevel = metadata.compression_level
        }

        // We enable harfbuzz font sub-setting in PDF documents if textlayout is enabled.
        #[cfg(all(feature = "textlayout", feature = "embed-icudtl"))]
        crate::icu::init();

        let mut stream = RustWStream::new(writer);
        let document = RCHandle::from_ptr(unsafe {
            sb::C_SkPDF_MakeDocument(stream.stream_mut(), md.native())
        })
        .unwrap();

        Document::new(stream, document)
    }

    //
    // Helper for constructing the internal metadata struct and setting associated strings.
    //

    type InternalMetadata = Handle<SkPDF_Metadata>;

    impl NativeDrop for SkPDF_Metadata {
        fn drop(&mut self) {
            unsafe { sb::C_SkPDF_Metadata_destruct(self) }
        }
    }

    impl Default for Handle<SkPDF_Metadata> {
        fn default() -> Self {
            Self::construct(|pdf_md| unsafe { sb::C_SkPDF_Metadata_Construct(pdf_md) })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::pdf;

    #[test]
    fn create_attribute_list() {
        let mut _al = pdf::AttributeList::default();
        _al.append_float_array("Owner", "Name", &[1.0, 2.0, 3.0]);
    }
}
