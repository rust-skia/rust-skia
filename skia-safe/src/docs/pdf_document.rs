pub mod pdf {
    use crate::interop::DynamicMemoryWStream;
    use crate::prelude::*;
    use crate::{scalar, DateTime, Document};
    use skia_bindings as sb;
    use skia_bindings::{SkPDF_Metadata, SkString};

    // TODO: DocumentStructureType
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
            internal.fTitle.set_s(&metadata.title);
            internal.fAuthor.set_s(&metadata.author);
            internal.fSubject.set_s(&metadata.subject);
            internal.fKeywords.set_s(&metadata.keywords);
            internal.fCreator.set_s(&metadata.creator);
            internal.fProducer.set_s(&metadata.producer);
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
unsafe impl Send for InternalMetadata {}
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

    trait Set {
        fn set_s(&mut self, str: impl AsRef<str>);
    }

    impl Set for SkString {
        fn set_s(&mut self, str: impl AsRef<str>) {
            let bytes = str.as_ref().as_bytes();
            unsafe { self.set1(bytes.as_ptr() as _, bytes.len()) }
        }
    }
}
