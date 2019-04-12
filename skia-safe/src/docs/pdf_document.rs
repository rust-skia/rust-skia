use crate::DateTime;

pub mod pdf {

    // TODO: DocumentStructureType
    // TODO: StructureElementNode

    #[derive(Clone, Debug)]
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
    // fn new_document()
}


