use std::fs::File;
use std::path::Path;
use std::time::SystemTime;
use image::{Bgr, DynamicImage, GenericImage, ImageFormat, Rgba};
use crate::screenshot_lib::{get_screenshot, Pixel};

pub fn save_screenshot(filename: &String) {
    let s = get_screenshot(0).unwrap();
    let mut buffer = DynamicImage::new_bgr8(s.width() as u32, s.height() as u32).to_bgr8();

    let data = s.raw_data();
    let mut idx: isize = 0;
    let row_len = s.row_len() as isize;
    let pixel_width = s.pixel_width() as u32;
    for i in 0..(s.width() - 1) as u32 {
        idx = (i * pixel_width) as isize;
        for j in 0..(s.height() - 1) as u32 {
            unsafe {
                buffer.put_pixel(i, j, Bgr::from([
                    *data.offset((idx)),
                    *data.offset((idx + 1)),
                    *data.offset((idx + 2))
                ]
                ));
            }
            idx += row_len;
        }
    }
    let mut file = File::create(filename).unwrap();
    let writer = DynamicImage::ImageBgr8(buffer);
    writer.write_to(&mut file, ImageFormat::Jpeg);
}
