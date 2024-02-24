use anyhow::{anyhow, Ok, Result};
use offscreen_canvas::{FontSettings, FilterType, WHITE, measure_text, Rect, Font, OffscreenCanvas, load_png, RED};
use slint::SharedPixelBuffer;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

slint::slint! {
    export component HelloWorld inherits Window {
        width: 300px;
        height: 300px;

        in-out property <image> source;

        image := Image {
            width: 100%;
            height: 100%;
            source: source;
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn run() {
    if let Err(err) = main() {
        eprintln!("{:?}", err);
    }
}

fn main() -> Result<()> {
    let app = HelloWorld::new()?;

    let flower = load_png(include_bytes!("../flower.png"))?;
    let mario = load_png(include_bytes!("../mario.png"))?;
    let font_bytes:&[u8] = include_bytes!("../VonwaonBitmap-16px.ttf");
    let font = Font::from_bytes(font_bytes, FontSettings::default()).map_err(|err| anyhow!("{err}"))?;

    let mut canvas = OffscreenCanvas::new(300, 300, font);

    canvas.draw_image_with_size_at(&flower, 0, 0, canvas.width(), canvas.height(), FilterType::Triangle);
    
    canvas.draw_image_with_src_and_dst(&mario, &Rect::from(268, 277, 37, 37), &Rect::from(0, 0, 50, 50), FilterType::Triangle);

    canvas.draw_image_with_src_and_dst(&mario, &Rect::from(361, 388, 37, 37), &Rect::from(50, 0, 50, 50), FilterType::Triangle);

    let text = "花.png";
    let text_width = measure_text(text, 25., canvas.font()).width() as u32;
    canvas.draw_text(text, WHITE, 25., 150-text_width as i32/2, 270);
    
    canvas.stroke_line((0, 0), (canvas.width() as i32, canvas.height() as i32), RED);

    let image_data = canvas.image_data();
    let buf = SharedPixelBuffer::clone_from_slice(&image_data, canvas.width(), canvas.height());
    app.set_source(slint::Image::from_rgba8(buf));

    app.run()?;

    Ok(())
}