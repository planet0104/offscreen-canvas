use image::{RgbaImage, Rgba, Pixel};
use imageproc::{drawing::BresenhamLineIter, rect::Rect};

pub fn draw_line_segment_mut(canvas: &mut RgbaImage, start: (f32, f32), end: (f32, f32), color: Rgba<u8>) {
    let (width, height) = canvas.dimensions();
    let in_bounds = |x, y| x >= 0 && x < width as i32 && y >= 0 && y < height as i32;

    let line_iterator = BresenhamLineIter::new(start, end);

    for point in line_iterator {
        let x = point.0;
        let y = point.1;

        if in_bounds(x, y) {
            let mut bottom_pixel = canvas.get_pixel(x as u32, y as u32).to_rgba();
            bottom_pixel.blend(&color);
            canvas.put_pixel(x as u32, y as u32, bottom_pixel);
        }
    }
}

fn draw_if_in_bounds(canvas: &mut RgbaImage, x: i32, y: i32, color: Rgba<u8>){
    if x >= 0 && x < canvas.width() as i32 && y >= 0 && y < canvas.height() as i32 {
        let mut bottom_pixel = canvas.get_pixel(x as u32, y as u32).to_rgba();
        bottom_pixel.blend(&color);
        canvas.put_pixel(x as u32, y as u32, bottom_pixel);
    }
}

pub fn draw_hollow_circle_mut(canvas: &mut RgbaImage, center: (i32, i32), radius: i32, color: Rgba<u8>) {
    let mut x = 0i32;
    let mut y = radius;
    let mut p = 1 - radius;
    let x0 = center.0;
    let y0 = center.1;

    while x <= y {
        draw_if_in_bounds(canvas, x0 + x, y0 + y, color);
        draw_if_in_bounds(canvas, x0 + y, y0 + x, color);
        draw_if_in_bounds(canvas, x0 - y, y0 + x, color);
        draw_if_in_bounds(canvas, x0 - x, y0 + y, color);
        draw_if_in_bounds(canvas, x0 - x, y0 - y, color);
        draw_if_in_bounds(canvas, x0 - y, y0 - x, color);
        draw_if_in_bounds(canvas, x0 + y, y0 - x, color);
        draw_if_in_bounds(canvas, x0 + x, y0 - y, color);

        x += 1;
        if p < 0 {
            p += 2 * x + 1;
        } else {
            y -= 1;
            p += 2 * (x - y) + 1;
        }
    }
}

/// Draws a rectangle and its contents on an image in place.
///
/// Draws as much of the rectangle and its contents as lies inside the image bounds.
pub fn draw_filled_rect_mut(canvas: &mut RgbaImage, rect: Rect, color: Rgba<u8>){
    let canvas_bounds = Rect::at(0, 0).of_size(canvas.width(), canvas.height());
    if let Some(intersection) = canvas_bounds.intersect(rect) {
        for dy in 0..intersection.height() {
            for dx in 0..intersection.width() {
                let x = intersection.left() as u32 + dx;
                let y = intersection.top() as u32 + dy;
                // canvas.draw_pixel(x, y, color);
                let mut bottom_pixel = canvas.get_pixel(x as u32, y as u32).to_rgba();
                bottom_pixel.blend(&color);
                canvas.put_pixel(x as u32, y as u32, bottom_pixel);
            }
        }
    }
}

/// Draws the outline of a rectangle on an image in place.
///
/// Draws as much of the boundary of the rectangle as lies inside the image bounds.
pub fn draw_hollow_rect_mut(canvas: &mut RgbaImage, rect: Rect, color: Rgba<u8>){
    let left = rect.left() as f32;
    let right = rect.right() as f32;
    let top = rect.top() as f32;
    let bottom = rect.bottom() as f32;

    draw_line_segment_mut(canvas, (left, top), (right, top), color);
    draw_line_segment_mut(canvas, (left, bottom), (right, bottom), color);
    draw_line_segment_mut(canvas, (left, top), (left, bottom), color);
    draw_line_segment_mut(canvas, (right, top), (right, bottom), color);
}

pub fn draw_filled_circle_mut(canvas: &mut RgbaImage, center: (i32, i32), radius: i32, color: Rgba<u8>) {
    let mut x = 0i32;
    let mut y = radius;
    let mut p = 1 - radius;
    let x0 = center.0;
    let y0 = center.1;

    while x <= y {
        draw_line_segment_mut(
            canvas,
            ((x0 - x) as f32, (y0 + y) as f32),
            ((x0 + x) as f32, (y0 + y) as f32),
            color,
        );
        draw_line_segment_mut(
            canvas,
            ((x0 - y) as f32, (y0 + x) as f32),
            ((x0 + y) as f32, (y0 + x) as f32),
            color,
        );
        draw_line_segment_mut(
            canvas,
            ((x0 - x) as f32, (y0 - y) as f32),
            ((x0 + x) as f32, (y0 - y) as f32),
            color,
        );
        draw_line_segment_mut(
            canvas,
            ((x0 - y) as f32, (y0 - x) as f32),
            ((x0 + y) as f32, (y0 - x) as f32),
            color,
        );

        x += 1;
        if p < 0 {
            p += 2 * x + 1;
        } else {
            y -= 1;
            p += 2 * (x - y) + 1;
        }
    }
}