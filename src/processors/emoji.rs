use image::{self, GenericImageView};

pub fn process(frame_id: u32, width: u32, height: u32) -> String {
    let frame = image::ImageReader::open(format!("./data/frames/frame{}.png", frame_id))
        .unwrap()
        .decode()
        .unwrap()
        .resize(width, height, image::imageops::Gaussian);
    let mut art = String::new();
    for i in 1..frame.height() - 1 {
        let pixel = frame.get_pixel(0, i);
        art += &super::Color::from_pixel(pixel).to_emoji_art().to_string();
        for j in 1..frame.width() {
            let left_pixel = super::Color::from_pixel(frame.get_pixel(j - 1, i));
            let right_pixel = super::Color::from_pixel(frame.get_pixel(j, i));
            let pixel = match left_pixel {
                super::Color::Black => match right_pixel {
                    super::Color::Black => "🌑",
                    super::Color::White => "🌓",
                },
                super::Color::White => match right_pixel {
                    super::Color::Black => "🌗",
                    super::Color::White => "🌕",
                },
            };

            art += pixel;
        }
        let pixel = frame.get_pixel(frame.width() - 1, i);
        art += &super::Color::from_pixel(pixel).to_emoji_art().to_string();
        art += "\n";
    }
    art
}
