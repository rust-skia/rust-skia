pub mod pdf {
    use crate::interop::{self, DynamicMemoryWStream, SetStr};
    use crate::prelude::*;
    use crate::{scalar, DateTime, Document};
    use skia_bindings as sb;
    use skia_bindings::{SkPDF_AttributeList, SkPDF_Metadata};

    pub use sb::SkPDF_DocumentStructureType as DocumentStructureType;
    use std::ffi::CString;
    #[test]
    fn document_structure_type_naming() {
        let _ = DocumentStructureType::BibEntry;
    }

    pub type AttributeList = Handle<SkPDF_AttributeList>;
    unsafe impl Send for AttributeList {}
    unsafe impl Sync for AttributeList {}

    impl NativeDrop for SkPDF_AttributeList {
        fn drop(&mut self) {
            unsafe { sb::C_SkPDF_AttributeList_destruct(self) }
        }
    }
    impl Default for AttributeList {
        fn default() -> Self {
            AttributeList::from_native(unsafe { SkPDF_AttributeList::new() })
        }
    }
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

        pub fn append_string(
            &mut self,
            owner: impl AsRef<str>,
            name: impl AsRef<str>,
            value: impl AsRef<str>,
        ) -> &mut Self {
            let owner = CString::new(owner.as_ref()).unwrap();
            let name = CString::new(name.as_ref()).unwrap();
            let value = CString::new(value.as_ref()).unwrap();
            unsafe {
                self.native_mut()
                    .appendString(owner.as_ptr(), name.as_ptr(), value.as_ptr())
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

        pub fn append_string_array(
            &mut self,
            owner: impl AsRef<str>,
            name: impl AsRef<str>,
            value: &[impl AsRef<str>],
        ) -> &mut Self {
            let owner = CString::new(owner.as_ref()).unwrap();
            let name = CString::new(name.as_ref()).unwrap();
            let value: Vec<interop::String> = value.iter().map(interop::String::from_str).collect();

            unsafe {
                sb::C_SkPDF_AttributeList_appendStringArray(
                    self.native_mut(),
                    owner.as_ptr(),
                    name.as_ptr(),
                    value.native().as_ptr(),
                    value.len(),
                )
            }
            self
        }
    }

    // TODO: StructureElementNode

    #[derive(Clone, Debug, Default)]
    pub struct Metadata {
        pub title: String,
        pub author: String,
        pub subject: String,
        pub keywords: String,
        pub creator: String,
        pub producer: String,
        pub creation: Option<DateTime>,
        pub modified: Option<DateTime>,
        pub raster_dpi: Option<scalar>,
        pub pdfa: bool,
        pub encoding_quality: Option<i32>,
        // TODO: fStructureElementTreeRoot
    }

    // TODO: SetNodeId

    pub fn new_document(metadata: Option<&Metadata>) -> Document {
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
            if let Some(raster_dpi) = metadata.raster_dpi {
                internal.fRasterDPI = raster_dpi;
            }
            internal.fPDFA = metadata.pdfa;
            if let Some(encoding_quality) = metadata.encoding_quality {
                internal.fEncodingQuality = encoding_quality
            }
        }

        // we can't move the memory stream around anymore as soon it's referred by
        // the document.
        let mut memory_stream = Box::pin(DynamicMemoryWStream::new());
        let document = RCHandle::from_ptr(unsafe {
            sb::C_SkPDF_MakeDocument(memory_stream.native_mut().base_mut(), md.native())
        })
        .unwrap();

        Document::new(memory_stream, document)
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

#[test]
fn create_attribute_list() {
    use pdf::AttributeList;
    let mut _al = AttributeList::default();
    _al.append_float_array("Owner", "Name", &[1.0, 2.0, 3.0]);
    _al.append_string_array("Owner", "Name", &["A", "B", "C"]);
}
