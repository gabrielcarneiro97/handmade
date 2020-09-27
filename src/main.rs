#![allow(dead_code)]

mod colors;
mod text;
mod text_img;

use text::*;

static _LOREM : &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent vel lobortis erat. Cras lacus lorem, lacinia sed ante sed, fringilla viverra elit. Morbi ornare enim at augue malesuada, non egestas lectus scelerisque. Sed vitae odio eget tortor feugiat convallis vitae in sapien. Fusce sagittis risus eget tortor viverra, a tempus orci accumsan. Praesent molestie ex turpis, non pretium ante scelerisque ut. Quisque consectetur lectus ut auctor tempus. Vivamus tincidunt porttitor felis tincidunt bibendum. Vestibulum ante est, eleifend convallis ex non, pretium scelerisque mi. Vivamus elit justo, cursus eu mauris tincidunt, scelerisque malesuada magna. Sed ante ex, tincidunt vitae dignissim vitae, aliquet nec velit. Nulla egestas lorem sit amet leo commodo interdum. Ut neque quam, scelerisque eu feugiat vitae, ornare eu velit. Nunc eget dui quis elit pharetra gravida. Aenean accumsan risus tincidunt dolor pretium dignissim. Fusce at leo eget risus tristique dapibus. Praesent fermentum dignissim ultrices. Curabitur tempus eget felis a ultrices. Curabitur nec vulputate velit. In hac habitasse platea dictumst. Nunc dignissim dui magna, a molestie metus tempus in. Quisque felis nibh, elementum sed mauris eget, viverra pretium augue. In sapien massa, laoreet ut lobortis a, lobortis ac velit. Proin sit amet luctus ante. Integer lacinia pharetra orci, sit amet tempor nibh aliquam id. Sed eget ex accumsan, lobortis neque aliquet, ullamcorper urna. Ut nec metus leo. Suspendisse luctus tortor ut ex dapibus tincidunt. Proin pretium at purus id feugiat. Ut ut nisi vitae turpis faucibus feugiat.";

fn main() {
    let dic_name = "augusto";

    // text_img::update_images(Some(dic_name.clone()));

    let mut text = Text::new_with_dic_name(&pages::DEFAULT, dic_name);
    text.parse_to_png(_LOREM, Some(dic_name));
}
