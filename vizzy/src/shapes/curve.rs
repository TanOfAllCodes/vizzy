use crate::canvas::{Canvas, Color};

pub const HELP: &str = "curve x1=<number> y1=<number> x2=<number> y2=<number> x3=<number> y3=<number> color=<string> [stroke=<number>];
Draws a quadratic Bezier curve with control points (x1, y1), (x2, y2), (x3, y3), color, and optional stroke width.
Example: curve x1=100 y1=200 x2=150 y2=250 x3=200 y3=200 color=\"gray\" stroke=3;";

pub struct Curve {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
    pub x3: i32,
    pub y3: i32,
    pub color: Color,
    pub stroke: i32,
}

impl Curve {
    pub fn draw(&self, _canvas: &mut Canvas) {
        _canvas.draw_curve(self.x1, self.y1, self.x2, self.y2, self.x3, self.y3, self.color.clone());
    }
}
