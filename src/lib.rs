use fontdue::layout::{CoordinateSystem, Layout, TextStyle};
use image::{imageops::{resize, crop_imm}, Pixel, ImageError};
use imageproc::geometric_transformations::rotate;
pub use fontdue::{Font, FontSettings};
pub use imageproc::geometric_transformations::Interpolation;
pub use image::{ Rgba, RgbaImage, imageops::FilterType };
mod imageproc_ex;
pub const WHITE:Rgba<u8> = Rgba([255, 255, 255, 255]);
pub const BLACK:Rgba<u8> = Rgba([0, 0, 0, 255]);
pub const RED:Rgba<u8> = Rgba([255, 0, 0, 255]);
pub const GREEN:Rgba<u8> = Rgba([0, 255, 0, 255]);
pub const BLUE:Rgba<u8> = Rgba([0, 0, 255, 255]);
pub const TRANSPARENT:Rgba<u8> = Rgba([0, 0, 0, 0]);

#[derive(Debug, Clone, Default, Copy)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Rect {
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Rect {
        Rect {
            left,
            top,
            right,
            bottom,
        }
    }

    pub fn from(x: i32, y: i32, width: i32, height: i32) -> Rect {
        Rect{
            left: x,
            top: y,
            right: x+width,
            bottom: y+height,
        }
    }

    pub fn width(&self) -> i32{
        self.right - self.left
    }

    pub fn height(&self) -> i32{
        self.bottom - self.top
    }

    /** 修改rect大小 */
    pub fn inflate(&mut self, dx:i32, dy:i32) {
        self.left -= dx;
        self.right += dx;
        self.top -= dy;
        self.bottom += dy;
    }

    pub fn offset(&mut self, dx:i32, dy:i32) {
        self.left += dx;
        self.right += dx;
        self.top += dy;
        self.bottom += dy;
    }

    pub fn contain(&self, x:i32, y:i32) -> bool {
        x >= self.left && x <= self.right && y >= self.top && y <= self.bottom
    }
}

pub struct RotateOption{
    pub center: (f32, f32),
    pub theta: f32,
    pub interpolation: Interpolation,
    pub default: Rgba<u8>
}

impl RotateOption{
    pub fn from(center: (f32, f32), theta: f32) -> Self{
        Self { center, theta, interpolation: Interpolation::Nearest, default: Rgba([0, 0, 0, 0]) }
    }
}

pub struct ResizeOption{
    pub nwidth: u32,
    pub nheight: u32,
    pub filter: FilterType
}

pub fn open_png(path: &str) -> Result<RgbaImage, ImageError>{
    Ok(image::open(path)?.to_rgba8())
}

pub fn load_png(data:&[u8]) -> Result<RgbaImage, ImageError>{
    Ok(image::load_from_memory_with_format(data, image::ImageFormat::Png)?.to_rgba8())
}

pub fn measure_text(text:&str, px: f32, font:&Font) -> Rect{
    // 创建文本布局
    let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
    // 设置文本和样式
    layout.append(&[font], &TextStyle::new(text, px, 0));
    // 获取渲染的字形信息
    let glyphs_pos = layout.glyphs();

    // 遍历每个字形位置
    let mut width = 0;
    let mut height = 0;
    for glyph in glyphs_pos {
        width += glyph.width;
        height = glyph.height;
    }
    Rect::from(0, 0, width as i32, height as i32)
}

pub struct OffscreenCanvas {
    canvas: RgbaImage,
    font: Font,
}

impl OffscreenCanvas {
    pub fn new(width: u32, height: u32, font: Font) -> Self{
        Self {
            font,
            canvas: RgbaImage::new(width, height)
        }
    }

    pub fn clear(&mut self, color: Rgba<u8>) {
        let width = self.width();
        let height = self.height();
        imageproc_ex::draw_filled_rect_mut(&mut self.canvas, imageproc::rect::Rect::at(0, 0).of_size(width, height), color);
    }

    pub fn fill_rect(&mut self, rect: Rect, color: Rgba<u8>) {
        imageproc_ex::draw_filled_rect_mut(&mut self.canvas, imageproc::rect::Rect::at(rect.left, rect.top).of_size(rect.width() as u32, rect.height() as u32), color);
    }

    pub fn stroke_rect(&mut self, rect: Rect, color: Rgba<u8>) {
        imageproc_ex::draw_hollow_rect_mut(&mut self.canvas, imageproc::rect::Rect::at(rect.left, rect.top).of_size(rect.width() as u32, rect.height() as u32), color);
    }

    pub fn fill_circle(&mut self, center: (i32, i32), radius: i32, color: Rgba<u8>) {
        imageproc_ex::draw_filled_circle_mut(&mut self.canvas, center, radius, color)
    }

    pub fn stroke_circle(&mut self, center: (i32, i32), radius: i32, color: Rgba<u8>) {
        imageproc_ex::draw_hollow_circle_mut(&mut self.canvas, center, radius, color)
    }

    pub fn stroke_line(&mut self, start: (i32, i32), end: (i32, i32), color: Rgba<u8>){
        imageproc_ex::draw_line_segment_mut(&mut self.canvas, (start.0 as f32, start.1 as f32), (end.0 as f32, end.1 as f32), color)
    }

    pub fn draw_image_with_rotation_at(&mut self, i: &RgbaImage, x: i32, y: i32, r: RotateOption){
        let r = rotate(i, r.center, r.theta, r.interpolation, r.default);
        image::imageops::overlay(&mut self.canvas, &r, x as i64, y as i64);
    }

    pub fn draw_image_at(&mut self, i: &RgbaImage, x: i32, y: i32, size:Option<ResizeOption>, rotate_option:Option<RotateOption>){
        if let Some(resize_option) = size{
            let i = resize(i, resize_option.nwidth, resize_option.nheight, resize_option.filter);
            match rotate_option {
                Some(option) => {
                    let r = rotate(&i, option.center, option.theta, option.interpolation, option.default);
                    image::imageops::overlay(&mut self.canvas, &r, x as i64, y as i64);
                }
                None => image::imageops::overlay(&mut self.canvas, &i, x as i64, y as i64)
            }
        }else{
            image::imageops::overlay(&mut self.canvas, i, x as i64, y as i64);
        }
    }

    pub fn draw_image_with_size_at(&mut self, i: &RgbaImage, x: i32, y: i32, nwidth: u32, nheight: u32, filter: FilterType){
        self.draw_image_at(i, x, y, Some(ResizeOption { nwidth, nheight, filter }), None)
    }

    pub fn draw_image_with_src_and_dst(&mut self, i: &RgbaImage, src: &Rect, dest: &Rect, filter: FilterType){
        let sub_image = crop_imm(i, src.left as u32, src.top as u32, src.width() as u32, src.height() as u32).to_image();
        self.draw_image_with_size_at(&sub_image, dest.left, dest.top, dest.width() as u32, dest.height() as u32, filter);
    }

    pub fn draw_image_with_src_and_dst_and_rotation(&mut self, i: &RgbaImage, src: &Rect, dest: &Rect, rotate_option:RotateOption){
        let sub_image = image::imageops::crop_imm(i, src.left as u32, src.top as u32, src.width() as u32, src.height() as u32).to_image();
        let filter = match rotate_option.interpolation{
            Interpolation::Nearest => FilterType::Nearest,
            Interpolation::Bilinear => FilterType::Triangle,
            Interpolation::Bicubic => FilterType::Lanczos3,
        };
        self.draw_image_at(&sub_image, dest.left, dest.top, Some(ResizeOption { nwidth: dest.width() as u32, nheight:dest.height() as u32, filter }), Some(rotate_option))
    }

    pub fn draw_text(&mut self, text: &str, color: Rgba<u8>, px: f32, x: u32, y: u32){
        // 创建文本布局
        let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
        // 设置文本和样式
        layout.append(&[&self.font], &TextStyle::new(text, px, 0));
        // 获取渲染的字形信息
        let glyphs_pos = layout.glyphs();
        // 遍历每个字形并渲染位图
        let glyphs: Vec<_> = text.chars().map(|c| self.font.rasterize(c, px)).collect();
        // 遍历每个字形的位图叠加到画布上
        for (glyph, (m, bitmap)) in glyphs_pos.iter().zip(glyphs) {
            let left = glyph.x;
            let top = glyph.y;
            //遍历字形的每一个像素
            for (i, value) in bitmap.iter().enumerate() {
                let dx = (i % m.width) as u32;
                let dy = (i / m.width) as u32;
                let sx = x + left as u32 + dx;
                let sy = y + top as u32 + dy;
                let mut p = color.clone();
                p[3] = (p[3] as f32 * (*value as f32 / 255.)) as u8;
                let mut bottom_pixel = *self.canvas.get_pixel(sx, sy);
                bottom_pixel.blend(&p);
                self.canvas.put_pixel(sx, sy, bottom_pixel);
            }
        }
    }

    pub fn get_pixel_rgb565(&self, x:u32, y:u32) -> u16{
        let pixel = self.canvas.get_pixel(x, y);
        let scale_color_to_565 = |color: u8, bits: u32| -> u16 {
            let scaled = (color as u16) >> (8 - bits);
            scaled & ((1 << bits) - 1)
        };
        let r = scale_color_to_565(pixel[0], 5);
        let g = scale_color_to_565(pixel[1], 6);
        let b = scale_color_to_565(pixel[2], 5);
        (r << 11) | (g << 5) | b
    }

    pub fn get_pixel(&self, x:u32, y:u32) -> &Rgba<u8>{
        self.canvas.get_pixel(x, y)
    }

    pub fn font(&self) -> &Font{
        &self.font
    }

    pub fn image_data(&self) -> &RgbaImage{
        &self.canvas
    }

    pub fn image_data_mut(&mut self) -> &mut RgbaImage{
        &mut self.canvas
    }

    pub fn width(&self) -> u32 {
        self.canvas.width()
    }

    pub fn height(&self) -> u32 {
        self.canvas.height()
    }
}

#[cfg(test)]
mod tests {
    use fontdue::{FontSettings, Font, layout::{TextStyle, Layout, CoordinateSystem}};
    use image::{Rgba, RgbaImage, Pixel};

    #[test]
    fn test1(){

        let font_bytes:&[u8] = include_bytes!("../examples/hello-slint/VonwaonBitmap-16px.ttf");
        let font = Font::from_bytes(font_bytes, FontSettings::default()).unwrap();
        // 设置文本和样式
        let text = "你好吗";
        let size = 16.0;
        let red = Rgba([255,0,0,255]);
        let text_pos_x = 150;
        let text_pos_y = 150;
        let mut source = RgbaImage::new(300, 300);

        // 创建文本布局
        let mut layout = Layout::new(CoordinateSystem::PositiveYDown);

        // 设置文本和样式
        layout.append(&[&font], &TextStyle::new(text, size, 0));

        // 获取渲染的字形信息
        let glyphs_pos = layout.glyphs();
        let glyphs: Vec<_> = text.chars().map(|c| font.rasterize(c, size)).collect();

        // 遍历每个字形并渲染位图
        for (glyph, (m, bitmap)) in glyphs_pos.iter().zip(glyphs) {
            let left = glyph.x;
            let top = glyph.y;
            //遍历字形的每一个像素
            for (i, value) in bitmap.iter().enumerate() {
                let dx = (i % m.width) as u32;
                let dy = (i / m.width) as u32;
                let sx = text_pos_x + left as u32 + dx;
                let sy = text_pos_y + top as u32 + dy;
                let mut p = red.clone();
                p[3] = (p[3] as f32 * (*value as f32 / 255.)) as u8;
                let mut bottom_pixel = *source.get_pixel(sx, sy);
                bottom_pixel.blend(&p);
                source.put_pixel(sx, sy, bottom_pixel);
            }
        }

        source.save("test.png").unwrap();
    }
}
