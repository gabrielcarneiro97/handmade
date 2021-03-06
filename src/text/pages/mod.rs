pub mod canvas;

use image::*;

#[derive(Debug)]
pub struct PageProps<'a> {
    pub canvas: &'a canvas::CanvasProps<'a>,
    pub margins: f32,
    pub line_height: f32,
    pub space_width: f32,
}

impl PageProps<'_> {
    pub fn line_max_width(&self) -> f32 {
        self.canvas.width as f32 - (self.margins * 2.0)
    }

    pub fn white_page(&self) -> RgbaImage {
        ImageBuffer::from_pixel(self.canvas.width, self.canvas.height, image::Rgba([255, 255, 255, 1]))
    }
}

pub static DEFAULT : PageProps = PageProps { canvas: &canvas::A4, line_height: 122.0, margins: 100.0, space_width: 20.0 };
