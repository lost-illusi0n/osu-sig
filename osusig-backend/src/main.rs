mod sig_image_generator;
mod sig_models;

use warp::Filter;
use warp::http::Response;
use crate::sig_models::{SigQuery, OsuManager};
use rosu::Osu;
use std::env;
use std::sync::Arc;
use std::convert::Infallible;
use crate::sig_image_generator::SigImageGenerator;
use image::codecs::png::PngEncoder;
use image::ColorType;
use warp::hyper::header::CONTENT_TYPE;
use warp::hyper::http::HeaderValue;

#[tokio::main]
async fn main() {
    let manager = Arc::new(OsuManager { osu: Osu::new(env::var("osu-token").unwrap()) });

    let sig = warp::path!("sig")
        .and(warp::query::<SigQuery>())
        .and(warp::any().map(move || manager.clone()))
        .and_then(handle_sig);

    warp::serve(sig)
        .run(([0, 0, 0, 0], 3030))
        .await;
}

async fn handle_sig(query: SigQuery, manager: Arc<OsuManager>) -> Result<Response<Vec<u8>>, Infallible> {
    let data = &manager.request_user_data(&query.name).await;
    let data = match data {
        Some(data) => data,
        None => return default_image()
    };
    let image = SigImageGenerator { color: query.color, data: &data }.generate().await;
    match image {
        Some(image) => {
            let mut output = Vec::new();
            PngEncoder::new(&mut output).encode(image.as_raw(), 338, 94, ColorType::Rgb8).unwrap();
            let mut resp = Response::new(output);
            resp.headers_mut().insert(CONTENT_TYPE, HeaderValue::from_static("image/png"));
            Ok(resp)
        },
        None => return default_image()
    }
}

fn default_image() -> Result<Response<Vec<u8>>, Infallible> {
    let image = image::load_from_memory(include_bytes!("not_found.png")).unwrap();
    let mut output = Vec::new();
    PngEncoder::new(&mut output).encode(image.to_rgb().as_raw(), 216, 62, ColorType::Rgb8).unwrap();
    let mut resp = Response::new(output);
    resp.headers_mut().insert(CONTENT_TYPE, HeaderValue::from_static("image/png"));
    Ok(resp)
}