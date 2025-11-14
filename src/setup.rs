use ffmpeg_next::software::scaling::context::Context;
use ffmpeg_next::util::frame::video::Video;

pub fn setup_frames() {
    run_video("./data/video/bad-apple.mp4");
}

fn run_video<P: AsRef<std::path::Path> + ?Sized>(path: &P) {
    let mut input = ffmpeg_next::format::input(path).unwrap();
    let stream = input
        .streams()
        .best(ffmpeg_next::media::Type::Video)
        .unwrap();

    let context_decoder =
        ffmpeg_next::codec::context::Context::from_parameters(stream.parameters()).unwrap();
    let mut decoder = context_decoder.decoder().video().unwrap();

    let mut scaler = Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        ffmpeg_next::format::Pixel::RGB24,
        decoder.width(),
        decoder.height(),
        ffmpeg_next::software::scaling::flag::Flags::BILINEAR,
    )
    .unwrap();

    let index = stream.index();
    let mut frame_index = 0;
    for (stream, packet) in input.packets() {
        if stream.index() == index {
            decoder.send_packet(&packet).unwrap();
            receive_frames(&mut decoder, &mut scaler, frame_index).unwrap();
            frame_index += 1;
        }
    }

    decoder.send_eof().unwrap();
}

fn receive_frames(
    decoder: &mut ffmpeg_next::decoder::Video,
    scaler: &mut Context,
    frame_index: usize,
) -> Result<(), ffmpeg_next::Error> {
    let mut decoded = Video::empty();
    while decoder.receive_frame(&mut decoded).is_ok() {
        let mut rgb_frame = Video::empty();
        scaler.run(&decoded, &mut rgb_frame)?;
        save_frame(&rgb_frame, frame_index);
    }
    Ok(())
}

fn save_frame(frame: &Video, index: usize) {
    image::save_buffer(
        format!("./data/frames/frame{}.png", index),
        frame.data(0),
        frame.width(),
        frame.height(),
        image::ColorType::Rgb8,
    )
    .unwrap();
}
