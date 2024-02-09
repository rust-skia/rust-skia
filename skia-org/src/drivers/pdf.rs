use crate::{artifact, drivers::DrawingDriver, Driver};
use skia_safe::Canvas;
use std::path::Path;

pub struct Pdf;

impl DrawingDriver for Pdf {
    const DRIVER: Driver = Driver::Pdf;

    fn new() -> Self {
        Self
    }

    fn draw_image(&mut self, size: (i32, i32), path: &Path, name: &str, func: impl Fn(&Canvas)) {
        let mut memory = Vec::new();
        let mut document = skia_safe::pdf::new_document(&mut memory, None).begin_page(size, None);
        func(document.canvas());
        document.end_page().close();
        artifact::write_file(&memory, path, name, "pdf");
    }
}
