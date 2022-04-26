use std::path::Path;
use crate::screenshot_lib::get_screenshot;

pub fn save_screenshot(filename: &String) {
    let s = get_screenshot(0).unwrap();
    image::save_buffer(&Path::new(&filename), s.as_ref(),
                       s.width() as u32, s.height() as u32, image::ColorType::Rgba8);
}
