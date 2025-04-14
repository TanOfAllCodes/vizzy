use crate::canvas::{Canvas, Color};

pub const HELP: &str = "point x=<number> y=<number> color=<string>;
Draws a single pixel at the specified coordinates with the given color.
Example: point x=100 y=100 color=\"red\";";

pub struct Point {
    pub x: i32,
    pub y: i32,
    pub color: Color,
}

impl Point {
    pub fn draw(&self, _canvas: &mut Canvas) {
        _canvas.set_pixel(self.x as u32, self.y as u32, self.color.clone());
    }
}
