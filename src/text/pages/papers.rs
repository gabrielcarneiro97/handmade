#[derive(Debug)]
pub struct PaperProps<'a> {
    pub name: &'a str,
    pub width: u32,
    pub height: u32,
}

pub static A4 : PaperProps = PaperProps { name: "A4", width: 595, height: 842 };
