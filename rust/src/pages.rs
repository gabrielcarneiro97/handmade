#[derive(Debug)]
pub struct PaperProps {
    name: &'static str,
    width: i32,
    height: i32,
}

pub static A4 : PaperProps = PaperProps { name: "A4", width: 595, height: 842 };
