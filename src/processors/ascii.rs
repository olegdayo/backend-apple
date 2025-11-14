use image::{self, GenericImageView};

pub fn process(frame_id: u32, width: u32, height: u32) -> String {
    let frame = image::ImageReader::open(format!("./data/frames/frame{}.png", frame_id))
        .unwrap()
        .decode()
        .unwrap()
        .resize(width, height, image::imageops::Gaussian);
    let mut art = String::new();
    for i in 0..frame.height() {
        for j in 0..frame.width() {
            art += &super::Color::from_pixel(frame.get_pixel(j, i))
                .to_ascii_art()
                .to_string();
        }
        art += "\n"
    }
    art
}
