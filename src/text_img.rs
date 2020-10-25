use super::text::*;

use image;

pub static LETTERS_FOLDER : &str = "./src/assets/";
pub static LETTERS_EXT : &str = ".png";


struct ImgCropOverlayInfos {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

fn cols_gray_avg(img : &image::ImageBuffer<image::Luma<u8>, std::vec::Vec<u8>>) -> Vec<f32> {
    let mut avgs : Vec<f32> = Vec::new();

    for x in 0..img.width() {
        let mut avg : f32 = 0.0;
        for y in 0..img.height() {
            let px = img.get_pixel(x, y).0[0];
            avg += px as f32;
        }
        avg /= img.height() as f32;
        avgs.push(avg);
    }

    avgs
}

fn rows_gray_avg(img : &image::ImageBuffer<image::Luma<u8>, std::vec::Vec<u8>>) -> Vec<f32> {
    let mut avgs : Vec<f32> = Vec::new();

    for y in 0..img.height() {
        let mut avg : f32 = 0.0;
        for x in 0..img.width() {
            let px = img.get_pixel(x, y).0[0];
            avg += px as f32;
        }
        avg /= img.width() as f32;
        avgs.push(avg);
    }

    avgs
}

pub fn update_images(dic_name : Option<&str>) -> Result<(), String> {
    let dic_path = paths::dic_path(&dic_name);

    let abc_path = dic_path.join("00 abc.png");

    let img = match image::open(abc_path) {
        Ok(i) => i,
        Err(e) => panic!(e),
    };


    let mut img_colorful = img.clone().to_rgba();

    let mut img_gray = img.to_luma();
    let c_avg = cols_gray_avg(&img_gray);

    let mut crops = Vec::new();

    let mut x : u32 = 0;
    let mut width : u32;

    let mut max_height = 0;

    for (i, col_avg) in c_avg.iter().enumerate() {
        if *col_avg < 254.0 {
            if x == 0 {
                x = i as u32;
            }
        } else if x != 0 {
            width = i as u32 - x;
            if width > 5 {
                let img_h = img_gray.height();
                let letter = image::imageops::crop(&mut img_gray, x, 0, width, img_h).to_image();

                let mut r_avg = rows_gray_avg(&letter);

                let mut y : u32 = 0;
                let mut height : u32 = 0;

                for (i2, row_avg) in r_avg.iter().enumerate() {
                    if *row_avg < 254.0 && y == 0 {
                        y = i2 as u32;
                        break;
                    }
                }

                r_avg.reverse();

                for (i2, row_avg) in r_avg.iter().enumerate() {
                    if *row_avg < 254.0 {
                        height = r_avg.len() as u32 - i2 as u32 - y;
                        break;
                    }
                }

                let crop = ImgCropOverlayInfos { x, y, width, height };

                if height > max_height {
                    max_height = height;
                }
                crops.push(crop);
            }

            x = 0;
        }
    }

    let over_line_height = max_height as f32;
    let under_line_height = over_line_height / 5.0;
    let line_height = over_line_height + under_line_height;

    let qnt = crops.len();

    println!("{}", qnt);

    if qnt < 58 {
        return Err(format!("Expected 58 characters, found {}, please sanitize the input file.", qnt));
    }

    for (i, c) in CHARS.iter().enumerate() {
        let crop = &crops[i];

        let letter = image::imageops::crop(&mut img_colorful, crop.x, crop.y, crop.width, crop.height).to_image();

        let l_width = letter.width() as f32;

        let width = (l_width + l_width * 0.2) as u32;

        let width = if width == letter.width() {
            width + 1
        } else {
            width
        };

        let mut bkg = image::RgbaImage::from_pixel(width, line_height as u32, image::Rgba([255, 255, 255, 1]));

        let x = 0;
        let y = if CHARS_UNDER.contains(c) {
            line_height as u32 - letter.height()
        } else {
            over_line_height as u32 - letter.height()
        };

        image::imageops::overlay(&mut bkg, &letter, x, y);

        let mut l_path = dic_path.join(Letter::char_name(*c));
        l_path.set_extension("png");

        bkg.save(l_path).unwrap();
    }

    Ok(())
}
