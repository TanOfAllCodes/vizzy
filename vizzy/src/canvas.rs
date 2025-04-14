use image::{ImageBuffer, Rgb};

pub const HELP: &str = "canvas width=<number> height=<number>;
Creates a canvas with the specified width and height in pixels.
Example: canvas width=800 height=600;";

#[derive(Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }
}

pub struct Canvas {
    buffer: ImageBuffer<Rgb<u8>, Vec<u8>>,
}

impl Canvas {
    pub fn new(width: u32, height: u32, color: Color) -> Self {
        let buffer = ImageBuffer::from_fn(width, height, |_, _| Rgb([color.r, color.g, color.b]));
        Canvas { buffer }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        if x < self.buffer.width() && y < self.buffer.height() {
            self.buffer.put_pixel(x, y, Rgb([color.r, color.g, color.b]));
        }
    }

    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: Color) {
        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx - dy;

        let mut x = x1;
        let mut y = y1;

        loop {
            self.set_pixel(x as u32, y as u32, color.clone());
            if x == x2 && y == y2 {
                break;
            }
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }

    pub fn draw_circle(&mut self, cx: i32, cy: i32, r: i32, color: Color, fill: bool) {
        let mut x = r;
        let mut y = 0;
        let mut err = 0;

        while x >= y {
            if fill {
                for i in cx - x..=cx + x {
                    self.set_pixel(i as u32, (cy + y) as u32, color.clone());
                    self.set_pixel(i as u32, (cy - y) as u32, color.clone());
                }
                for i in cx - y..=cx + y {
                    self.set_pixel(i as u32, (cy + x) as u32, color.clone());
                    self.set_pixel(i as u32, (cy - x) as u32, color.clone());
                }
            } else {
                self.set_pixel((cx + x) as u32, (cy + y) as u32, color.clone());
                self.set_pixel((cx - x) as u32, (cy + y) as u32, color.clone());
                self.set_pixel((cx + x) as u32, (cy - y) as u32, color.clone());
                self.set_pixel((cx - x) as u32, (cy - y) as u32, color.clone());
                self.set_pixel((cx + y) as u32, (cy + x) as u32, color.clone());
                self.set_pixel((cx - y) as u32, (cy + x) as u32, color.clone());
                self.set_pixel((cx + y) as u32, (cy - x) as u32, color.clone());
                self.set_pixel((cx - y) as u32, (cy - x) as u32, color.clone());
            }
            if err <= 0 {
                y += 1;
                err += 2 * y + 1;
            }
            if err > 0 {
                x -= 1;
                err -= 2 * x + 1;
            }
        }
    }

    pub fn save(&self, filename: &str) -> Result<(), String> {
        self.buffer
            .save(filename)
            .map_err(|e| format!("Failed to save image: {}", e))?;
        Ok(())
    }

    pub fn draw_rectangle(&mut self, x: i32, y: i32, width: i32, height: i32, color: Color, fill: bool) {
        if fill {
            for i in x..x + width {
                for j in y..y + height {
                    self.set_pixel(i as u32, j as u32, color.clone());
                }
            }
        } else {
            self.draw_line(x, y, x + width, y, color.clone());
            self.draw_line(x + width, y, x + width, y + height, color.clone());
            self.draw_line(x + width, y + height, x, y + height, color.clone());
            self.draw_line(x, y + height, x, y, color.clone());
        }
    }
    pub fn draw_ellipse(&mut self, cx: i32, cy: i32, rx: i32, ry: i32, color: Color, fill: bool) {
        let mut x = 0;
        let mut y = ry;
        let mut d1 = ry * ry - rx * rx * ry + rx * rx / 4;
        let mut dx = 2 * ry * ry * x;
        let mut dy = 2 * rx * rx * y;
        while dx < dy {
            if fill {
                for i in cx - x..=cx + x {
                    self.set_pixel(i as u32, (cy + y) as u32, color.clone());
                    self.set_pixel(i as u32, (cy - y) as u32, color.clone());
                }
            } else {
                self.set_pixel((cx + x) as u32, (cy + y) as u32, color.clone());
                self.set_pixel((cx - x) as u32, (cy + y) as u32, color.clone());
                self.set_pixel((cx + x) as u32, (cy - y) as u32, color.clone());
                self.set_pixel((cx - x) as u32, (cy - y) as u32, color.clone());
            }
            if d1 < 0 {
                x += 1;
                dx += 2 * ry * ry;
                d1 += dx + ry * ry;
            } else {
                x += 1;
                y -= 1;
                dx += 2 * ry * ry;
                dy -= 2 * rx * rx;
                d1 += dx - dy + ry * ry;
            }
        }
        let mut d2 = ((ry * ry) as f32 * (x as f32 + 0.5).powi(2) + (rx * rx) as f32 * (y - 1) as f32 * (y - 1) as f32 - (rx * rx * ry * ry) as f32) as i32;        while y >= 0 {
            if fill {
                for i in cx - x..=cx + x {
                    self.set_pixel(i as u32, (cy + y) as u32, color.clone());
                    self.set_pixel(i as u32, (cy - y) as u32, color.clone());
                }
            } else {
                self.set_pixel((cx + x) as u32, (cy + y) as u32, color.clone());
                self.set_pixel((cx - x) as u32, (cy + y) as u32, color.clone());
                self.set_pixel((cx + x) as u32, (cy - y) as u32, color.clone());
                self.set_pixel((cx - x) as u32, (cy - y) as u32, color.clone());
            }
            if d2 > 0 {
                y -= 1;
                dy -= 2 * rx * rx;
                d2 += rx * rx - dy;
            } else {
                y -= 1;
                x += 1;
                dx += 2 * ry * ry;
                dy -= 2 * rx * rx;
                d2 += dx - dy + rx * rx;
            }
        }
    }
    pub fn draw_arc(&mut self, cx: i32, cy: i32, r: i32, start: f32, end: f32, color: Color) {
        let start_rad = start.to_radians();
        let end_rad = end.to_radians();
        let steps = 100;
        let step = (end_rad - start_rad) / steps as f32;
        for i in 0..=steps {
            let theta = start_rad + i as f32 * step;
            let x = cx + (r as f32 * theta.cos()) as i32;
            let y = cy + (r as f32 * theta.sin()) as i32;
            self.set_pixel(x as u32, y as u32, color.clone());
        }
    }
    pub fn draw_triangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, color: Color, fill: bool) {
        self.draw_line(x1, y1, x2, y2, color.clone());
        self.draw_line(x2, y2, x3, y3, color.clone());
        self.draw_line(x3, y3, x1, y1, color.clone());
        if fill {
            let min_x = x1.min(x2).min(x3);
            let max_x = x1.max(x2).max(x3);
            let min_y = y1.min(y2).min(y3);
            let max_y = y1.max(y2).max(y3);
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    self.set_pixel(x as u32, y as u32, color.clone());
                }
            }
        }
    }
    pub fn draw_quadrilateral(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, x4: i32, y4: i32, color: Color, fill: bool) {
        self.draw_line(x1, y1, x2, y2, color.clone());
        self.draw_line(x2, y2, x3, y3, color.clone());
        self.draw_line(x3, y3, x4, y4, color.clone());
        self.draw_line(x4, y4, x1, y1, color.clone());
        if fill {
            let min_x = x1.min(x2).min(x3).min(x4);
            let max_x = x1.max(x2).max(x3).max(x4);
            let min_y = y1.min(y2).min(y3).min(y4);
            let max_y = y1.max(y2).max(y3).max(y4);
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    self.set_pixel(x as u32, y as u32, color.clone());
                }
            }
        }
    }
    pub fn draw_curve(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, color: Color) {
        let steps = 100;
        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            let u = 1.0 - t;
            let x = (u * u * x1 as f32 + 2.0 * u * t * x2 as f32 + t * t * x3 as f32) as i32;
            let y = (u * u * y1 as f32 + 2.0 * u * t * y2 as f32 + t * t * y3 as f32) as i32;
            self.set_pixel(x as u32, y as u32, color.clone());
        }
    }
}
