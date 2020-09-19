#[derive(Debug)]
pub struct CanvasProps<'a> {
    pub name: &'a str,
    pub width: u32,
    pub height: u32,
}

pub static A4 : CanvasProps = CanvasProps { name: "A4 1:1", width: 595, height: 842 };
