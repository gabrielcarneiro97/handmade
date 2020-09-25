#![allow(dead_code)]

mod colors;
mod text;
mod text_img;

use text::*;
use image;

use std::collections::BTreeMap;

use imageproc::{region_labelling::{connected_components, Connectivity}};

static _LOREM : &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent vel lobortis erat. Cras lacus lorem, lacinia sed ante sed, fringilla viverra elit. Morbi ornare enim at augue malesuada, non egestas lectus scelerisque. Sed vitae odio eget tortor feugiat convallis vitae in sapien. Fusce sagittis risus eget tortor viverra, a tempus orci accumsan. Praesent molestie ex turpis, non pretium ante scelerisque ut. Quisque consectetur lectus ut auctor tempus. Vivamus tincidunt porttitor felis tincidunt bibendum. Vestibulum ante est, eleifend convallis ex non, pretium scelerisque mi. Vivamus elit justo, cursus eu mauris tincidunt, scelerisque malesuada magna. Sed ante ex, tincidunt vitae dignissim vitae, aliquet nec velit. Nulla egestas lorem sit amet leo commodo interdum. Ut neque quam, scelerisque eu feugiat vitae, ornare eu velit. Nunc eget dui quis elit pharetra gravida. Aenean accumsan risus tincidunt dolor pretium dignissim. Fusce at leo eget risus tristique dapibus. Praesent fermentum dignissim ultrices. Curabitur tempus eget felis a ultrices. Curabitur nec vulputate velit. In hac habitasse platea dictumst. Nunc dignissim dui magna, a molestie metus tempus in. Quisque felis nibh, elementum sed mauris eget, viverra pretium augue. In sapien massa, laoreet ut lobortis a, lobortis ac velit. Proin sit amet luctus ante. Integer lacinia pharetra orci, sit amet tempor nibh aliquam id. Sed eget ex accumsan, lobortis neque aliquet, ullamcorper urna. Ut nec metus leo. Suspendisse luctus tortor ut ex dapibus tincidunt. Proin pretium at purus id feugiat. Ut ut nisi vitae turpis faucibus feugiat.";

fn most_common<T : std::cmp::Ord + Copy>(vec : &Vec<T>) -> T {
    let mut counts = BTreeMap::new();

    for el in vec {
        *counts.entry(el).or_insert(0) += 1;
    }

    let max = counts.into_iter().max_by_key(|&(_, count)| count);

    *max.unwrap().0
}

fn find_bkg(connected : &image::ImageBuffer<image::Luma<u32>, std::vec::Vec<u32>>) -> u32 {
    let vec = connected.to_vec();
    most_common(&vec)
}

fn cols_avg(connected : &image::ImageBuffer<image::Luma<u32>, std::vec::Vec<u32>>) -> Vec<f32> {
    let mut avgs : Vec<f32> = Vec::new();

    for x in 0..connected.width() {
        let mut avg : f32 = 0.0;
        for y in 0..connected.height() {
            let px = connected.get_pixel(x, y).0[0];
            avg += px as f32;
        }
        avg /= connected.height() as f32;
        avgs.push(avg);
    }

    avgs
}

fn rows_avg(connected : &image::ImageBuffer<image::Luma<u32>, std::vec::Vec<u32>>) -> Vec<f32> {
    let mut avgs : Vec<f32> = Vec::new();

    for y in 0..connected.height() {
        let mut avg : f32 = 0.0;
        for x in 0..connected.width() {
            let px = connected.get_pixel(x, y).0[0];
            avg += px as f32;
        }
        avg /= connected.width() as f32;
        avgs.push(avg);
    }

    avgs
}

fn main() {
    // text_img::update_images("./src/assets/00 abc 2.png");

    let mut img = match image::open("./src/assets/00 abc 2.png") {
        Ok(i) => i,
        Err(e) => panic!(e),
    };

    let img = img.to_rgba();

    let res = connected_components(&img, Connectivity::Eight, image::Rgba([255, 255, 255, 1]));
    // res.save("./src/assets/00 abc 2 connected.png");
    // println!("{:?}", res);

    let bkg = find_bkg(&res);

    let avgs = rows_avg(&res);

    println!("{:?}", avgs);

    // let mut text = Text::new(&pages::DEFAULT);
    // text.parse_str(_LOREM);
    // text.to_files();
}
