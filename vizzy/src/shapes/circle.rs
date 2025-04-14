use crate::canvas::{Canvas, Color};

pub const HELP: &str = "circle x=<number> y=<number> radius=<number> color=<string> [fill=<boolean>] [stroke=<number>];
Draws a circle centered at (x, y) with the given radius, color, optional fill, and stroke width.
Example: circle x=400 y=300 radius=50 color=\"green\" fill=true;";

pub struct Circle {
    pub x: i32,
    pub y: i32,
    pub radius: i32,
    pub color: Color,
    pub fill: bool,
    pub stroke: i32,
}

impl Circle {
    pub fn draw(&self, _canvas: &mut Canvas) {
        _canvas.draw_circle(self.x, self.y, self.radius, self.color.clone(), self.fill);
        // TODO: Implement stroke thickness
    }
}
