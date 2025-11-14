use axum::extract::Path;
use axum::extract::Query;
use serde::Deserialize;

const DEFAULT_WIDTH: u32 = 960;
const DEFAULT_HEIGHT: u32 = 720;

#[derive(Debug, Deserialize)]
pub struct Params {
    pub format: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

pub async fn frame(Path(frame_id): Path<u32>, Query(params): Query<Params>) -> String {
    match params.format {
        Some(format) => match &format as &str {
            "ascii" => crate::processors::ascii::process(
                frame_id,
                params.width.unwrap_or(DEFAULT_WIDTH),
                params.height.unwrap_or(DEFAULT_HEIGHT),
            ),
            "emoji" => crate::processors::emoji::process(
                frame_id,
                params.width.unwrap_or(DEFAULT_WIDTH),
                params.height.unwrap_or(DEFAULT_HEIGHT),
            ),
            _ => crate::processors::ascii::process(
                frame_id,
                params.width.unwrap_or(DEFAULT_WIDTH),
                params.height.unwrap_or(DEFAULT_HEIGHT),
            ),
        },
        None => crate::processors::ascii::process(
            frame_id,
            params.width.unwrap_or(DEFAULT_WIDTH),
            params.height.unwrap_or(DEFAULT_HEIGHT),
        ),
    }
}
