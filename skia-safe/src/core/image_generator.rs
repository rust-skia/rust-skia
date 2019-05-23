use crate::{Data, ImageInfo, gpu};

// TOOD: Handle<>
pub enum ImageGenerator {}

// TODO: NativeDrop

impl ImageGenerator {
    pub fn unique_id(&self) -> u32 {
        unimplemented!()
    }

    pub fn encoded_data(&self) -> Data {
        unimplemented!()
    }

    pub fn info(&self) -> &ImageInfo {
        unimplemented!()
    }

    pub fn is_valid(&self, context: Option<&mut gpu::Context>) -> bool {
        unimplemented!()
    }

    #[must_use]
    pub fn get_pixels(&self, info: &ImageInfo, pixels: &mut [u8], row_bytes: usize) -> bool {
        // TODO: check if other functions similar to get_pixels adhere to the same asserts:
        assert!(info.height() > 0);
        assert!(pixels.len() >= ((info.height()-1) as usize) * row_bytes + ((info.width() as usize) * info.bytes_per_pixel()));
        unimplemented!()
    }

    // TODO: queryYUVA8()
    // TODO: getYUVA8Planes()
    // TODO: generateTexture()
    // TODO: MakeFromEncoded()
    // TODO: MakeFromPicture()
}
