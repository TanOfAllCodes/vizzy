use crate::canvas::{Canvas, Color};

pub const HELP: &str = "triangle x1=<number> y1=<number> x2=<number> y2=<number> x3=<number> y3=<number> color=<string> [fill=<boolean>] [stroke=<number>];
Draws a triangle with vertices (x1, y1), (x2, y2), (x3, y3), color, optional fill, and stroke width.
Example: triangle x1=300 y1=100 x2=350 y2=200 x3=250 y3=200 color=\"cyan\" fill=true;";

pub struct Triangle {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
    pub x3: i32,
    pub y3: i32,
    pub color: Color,
    pub fill: bool,
    pub stroke: i32,
}

impl Triangle {
    pub fn draw(&self, _canvas: &mut Canvas) {
        _canvas.draw_triangle(self.x1, self.y1, self.x2, self.y2, self.x3, self.y3, self.color.clone(), self.fill);
    }
}