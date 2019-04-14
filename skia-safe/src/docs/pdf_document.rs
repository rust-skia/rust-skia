pub mod pdf {
    use crate::interop::DynamicMemoryWStream;
    use crate::prelude::*;
    use crate::{scalar, DateTime, Document};
    use skia_bindings::{
        C_SkPDF_MakeDocument, C_SkPDF_Metadata_Construct, C_SkPDF_Metadata_destruct,
        SkPDF_Metadata, SkString,
    };
    use std::mem;

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
            metadata
                .creation
                .map(|creation| internal.fCreation = creation.into_native());
            metadata
                .modified
                .map(|modified| internal.fModified = modified.into_native());
            metadata
                .raster_dpi
                .map(|raster_dpi| internal.fRasterDPI = raster_dpi);
            internal.fPDFA = metadata.pdfa;
            metadata
                .encoding_quality
                .map(|encoding_quality| internal.fEncodingQuality = encoding_quality);
        }

        let mut memory_stream = DynamicMemoryWStream::new();
        let document = RCHandle::from_ptr(unsafe {
            C_SkPDF_MakeDocument(&mut memory_stream.native_mut()._base, md.native())
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
            unsafe { C_SkPDF_Metadata_destruct(self) }
        }
    }

    impl Default for Handle<SkPDF_Metadata> {
        fn default() -> Self {
            let mut metadata = unsafe { mem::uninitialized() };
            unsafe { C_SkPDF_Metadata_Construct(&mut metadata) }
            Self::from_native(metadata)
        }
    }

    trait Set {
        fn set_s(&mut self, str: &String);
    }

    impl Set for SkString {
        fn set_s(&mut self, str: &String) {
            let bytes = str.as_str().as_bytes();
            unsafe { self.set2(bytes.as_ptr() as _, bytes.len()) }
        }
    }
}
