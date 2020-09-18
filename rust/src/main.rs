
mod colors;
mod text;

use text::*;

static LOREM : &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque felis nibh, elementum sed mauris eget, viverra pretium augue. In sapien massa, laoreet ut lobortis a, lobortis ac velit. Proin sit amet luctus ante. Integer lacinia pharetra orci, sit amet tempor nibh aliquam id. Sed eget ex accumsan, lobortis neque aliquet, ullamcorper urna. Ut nec metus leo. Suspendisse luctus tortor ut ex dapibus tincidunt. Proin pretium at purus id feugiat. Ut ut nisi vitae turpis faucibus feugiat.";

fn main() {
    let mut text = Text::new(&pages::DEFAULT);

    text.parse_str(LOREM);
    text.print();
}
