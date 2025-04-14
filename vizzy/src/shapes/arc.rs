use crate::canvas::{Canvas, Color};

pub const HELP: &str = "arc x=<number> y=<number> radius=<number> start=<number> end=<number> color=<string> [stroke=<number>];
Draws an arc centered at (x, y) with the given radius, from start angle to end angle (degrees), color, and optional stroke width.
Example: arc x=600 y=200 radius=50 start=45 end=135 color=\"orange\";";

pub struct Arc {
    pub x: i32,
    pub y: i32,
    pub radius: i32,
    pub start: f32,
    pub end: f32,
    pub color: Color,
    pub stroke: i32,
}

impl Arc {
    pub fn draw(&self, _canvas: &mut Canvas) {
        _canvas.draw_arc(self.x, self.y, self.radius, self.start, self.end, self.color.clone());
    }
}
