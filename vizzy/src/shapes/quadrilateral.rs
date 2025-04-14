use crate::canvas::{Canvas, Color};

pub const HELP: &str = "quadrilateral x1=<number> y1=<number> x2=<number> y2=<number> x3=<number> y3=<number> x4=<number> y4=<number> color=<string> [fill=<boolean>] [stroke=<number>];
Draws a quadrilateral with vertices (x1, y1), (x2, y2), (x3, y3), (x4, y4), color, optional fill, and stroke width.
Example: quadrilateral x1=400 y1=400 x2=450 y2=450 x3=400 y3=500 x4=350 y4=450 color=\"brown\";";

pub struct Quadrilateral {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
    pub x3: i32,
    pub y3: i32,
    pub x4: i32,
    pub y4: i32,
    pub color: Color,
    pub fill: bool,
    pub stroke: i32,
}

impl Quadrilateral {
    pub fn draw(&self, _canvas: &mut Canvas) {
        _canvas.draw_quadrilateral(self.x1, self.y1, self.x2, self.y2, self.x3, self.y3, self.x4, self.y4, self.color.clone(), self.fill);
    }
}
