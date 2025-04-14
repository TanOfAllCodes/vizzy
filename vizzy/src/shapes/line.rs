use crate::canvas::{Canvas, Color};

pub const HELP: &str = "line x1=<number> y1=<number> x2=<number> y2=<number> color=<string> [stroke=<number>];
Draws a line from (x1, y1) to (x2, y2) with the given color and optional stroke width.
Example: line x1=150 y1=150 x2=300 y2=300 color=\"blue\" stroke=2;";

pub struct Line {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
    pub color: Color,
    pub stroke: i32,
}

impl Line {
    pub fn draw(&self, _canvas: &mut Canvas) {
        _canvas.draw_line(self.x1, self.y1, self.x2, self.y2, self.color.clone());
        // TODO: Implement stroke thickness
    }
}
