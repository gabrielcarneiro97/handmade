mod papers;

#[derive(Debug)]
pub struct PageProps<'a> {
    pub paper: &'a papers::PaperProps<'a>,
    pub margins: f32,
    pub line_height: f32,
}

impl PageProps<'_> {
    pub fn line_max_width(&self) -> f32 {
        self.paper.width as f32 - (self.margins * 2.0)
    }
}

pub static DEFAULT : PageProps = PageProps { line_height: 35.0, margins: 10.0, paper: &papers::A4 };
