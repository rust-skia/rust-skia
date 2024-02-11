use std::path::Path;

use skia_safe::{pdf, Canvas};

use crate::{artifact, drivers::DrawingDriver, Driver};

pub struct Pdf;

impl DrawingDriver for Pdf {
    const DRIVER: Driver = Driver::Pdf;

    fn new() -> Self {
        Self
    }

    fn draw_image(&mut self, size: (i32, i32), path: &Path, name: &str, func: impl Fn(&Canvas)) {
        let mut memory = Vec::new();
        let mut document = pdf::new_document(&mut memory, None).begin_page(size, None);
        func(document.canvas());
        document.end_page().close();
        artifact::write_file(&memory, path, name, "pdf");
    }
}
