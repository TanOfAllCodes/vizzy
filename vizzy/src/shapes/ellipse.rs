use crate::canvas::{Canvas, Color};

pub const HELP: &str = "ellipse x=<number> y=<number> rx=<number> ry=<number> color=<string> [fill=<boolean>] [stroke=<number>];
Draws an ellipse centered at (x, y) with horizontal radius rx, vertical radius ry, color, optional fill, and stroke width.
Example: ellipse x=200 y=500 rx=60 ry=40 color=\"purple\";";

pub struct Ellipse {
    pub x: i32,
    pub y: i32,
    pub rx: i32,
    pub ry: i32,
    pub color: Color,
    pub fill: bool,
    pub stroke: i32,
}

impl Ellipse {
    pub fn draw(&self, _canvas: &mut Canvas) {
        _canvas.draw_ellipse(self.x, self.y, self.rx, self.ry, self.color.clone(), self.fill);
    }
}