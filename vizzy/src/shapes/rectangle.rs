use crate::canvas::{Canvas, Color};

pub const HELP: &str = "rectangle x=<number> y=<number> width=<number> height=<number> color=<string> [fill=<boolean>] [stroke=<number>];
Draws a rectangle at top-left (x, y) with the given width, height, color, optional fill, and stroke width.
Example: rectangle x=500 y=400 width=100 height=80 color=\"black\" fill=false stroke=1;";

pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub color: Color,
    pub fill: bool,
    pub stroke: i32,
}

impl Rectangle {
    pub fn draw(&self, _canvas: &mut Canvas) {
        if self.fill {
            for y in self.y..self.y + self.height {
                for x in self.x..self.x + self.width {
                    _canvas.set_pixel(x as u32, y as u32, self.color.clone());
                }
            }
        } else {
            _canvas.draw_line(self.x, self.y, self.x + self.width, self.y, self.color.clone());
            _canvas.draw_line(self.x, self.y + self.height, self.x + self.width, self.y + self.height, self.color.clone());
            _canvas.draw_line(self.x, self.y, self.x, self.y + self.height, self.color.clone());
            _canvas.draw_line(self.x + self.width, self.y, self.x + self.width, self.y + self.height, self.color.clone());
        }
        // TODO: Implement stroke thickness
    }
}
