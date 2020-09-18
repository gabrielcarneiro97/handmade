mod papers;

#[derive(Debug)]
pub struct PageProps<'a> {
    paper: &'a papers::PaperProps<'a>,
    margins: f32,
}

impl PageProps<'_> {
    pub fn line_max_width(&self) -> f32 {
        self.paper.width - (self.margins * 2.0)
    }
}

pub static DEFAULT : PageProps = PageProps { margins: 10.0, paper: &papers::A4 };
