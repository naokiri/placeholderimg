use lambda_http::{handler, lambda::{self, Context}, Body, Request, Response, RequestExt};
use image::{RgbImage, Rgb};
use imageproc::rect::Rect;
use imageproc::drawing::{draw_hollow_rect_mut, draw_filled_rect_mut, draw_text_mut};
use rusttype::{Scale, Font, point};
use image::DynamicImage;
use lambda_http::http::HeaderValue;

// TODO: Define own error and return better response on error case.
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler(generate_response);
    lambda::run(func).await?;
    Ok(())
}

const MIN_H_PX: u32 = 1;
const MAX_H_PX: u32 = 10000;
const MIN_W_PX: u32 = 1;
const MAX_W_PX: u32 = 10000;

const WHITE_COLOR: Rgb<u8> = Rgb([255, 255, 255]);
const BLACK_COLOR: Rgb<u8> = Rgb([0, 0, 0]);

const TEXT_MARGIN: u32 = 5;

async fn generate_response(request: Request, _context: Context) -> Result<Response<Body>, Error> {
    let param_map = request.query_string_parameters();
    let h = crop(param_map.get("h").unwrap_or_default().parse::<u32>().unwrap_or_default(), MIN_H_PX, MAX_H_PX);
    let w = crop(param_map.get("w").unwrap_or_default().parse::<u32>().unwrap_or_default(), MIN_W_PX, MAX_W_PX);

    let img = DynamicImage::ImageRgb8(generate_img(w, h));
    let mut buf = Vec::new();
    img.write_to(&mut buf, image::ImageOutputFormat::Png).expect("Couldn't write the png img.");
    let buflen = buf.len();
    let response = Response::new(Body::Binary(buf));
    let (mut parts, body) = response.into_parts();
    parts.headers.append("content-type", HeaderValue::from_static("image/png"));
    parts.headers.append("content-length", HeaderValue::from_str(&format!("{}", buflen)).unwrap());
    parts.headers.append("content-disposition", HeaderValue::from_str(&format!("attachment; filename=\"{}x{}.png\"", w, h)).unwrap());
    Ok(Response::from_parts(parts, body))
}

fn generate_img(w: u32, h: u32) -> RgbImage {
    let mut img = RgbImage::new(w, h);

    draw_filled_rect_mut(&mut img, Rect::at(0, 0).of_size(w, h), WHITE_COLOR);
    draw_hollow_rect_mut(&mut img, Rect::at(0, 0).of_size(w, h), BLACK_COLOR);
    if w > 2 && h > 2 {
        draw_hollow_rect_mut(&mut img, Rect::at(1, 1).of_size(w - 2, h - 2), BLACK_COLOR);
    }
    if w > 4 && h > 4 {
        draw_hollow_rect_mut(&mut img, Rect::at(2, 2).of_size(w - 4, h - 4), BLACK_COLOR);
    }

    draw_wh_text_center(&mut img, BLACK_COLOR, h, w);
    img
}

fn draw_wh_text_center(canvas: &mut RgbImage,
                       color: Rgb<u8>,
                       h: u32,
                       w: u32)
{
    let scale = Scale::uniform(50.0);
    let font: Font = Font::try_from_bytes(include_bytes!("../fonts/source-sans-pro/TTF/SourceSans3-Regular.ttf")).unwrap();
    let text = format!("{}x{}", w, h);
    let mut glyphs = font.layout(&text, scale, point(0.0, font.v_metrics(scale).ascent));
    let first = glyphs.next().unwrap().pixel_bounding_box().unwrap();
    let last = glyphs.last().unwrap().pixel_bounding_box().unwrap();
    // We know that first letter is a number and the middle 'x' is lower and has same baseline, so whole glyphs' height is same as the first letter's height.
    let height = (first.max.y - first.min.y) as u32;
    let width = (last.max.x - first.min.x) as u32;

    if height < (h - TEXT_MARGIN * 2) && width < (w - TEXT_MARGIN * 2) {
        let center_x = (w / 2) - (width / 2) - first.min.x as u32;
        let center_y = (h / 2) - (height / 2) - first.min.y as u32;
        draw_text_mut(canvas, color, center_x, center_y, scale, &font, &text);
    }
}

fn crop(x: u32, min: u32, max: u32) -> u32 {
    if x < min { min } else if x > max { max } else { x }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generate_example() {
        let buffer = generate_img(400, 250);
        buffer.save("example.png");
    }
}