pub mod point;
pub mod line;
pub mod circle;
pub mod rectangle;
pub mod ellipse;
pub mod arc;
pub mod triangle;
pub mod quadrilateral;
pub mod curve;

use crate::canvas::Canvas;

pub enum ShapeKind {
    Point(point::Point),
    Line(line::Line),
    Circle(circle::Circle),
    Rectangle(rectangle::Rectangle),
    Ellipse(ellipse::Ellipse),
    Arc(arc::Arc),
    Triangle(triangle::Triangle),
    Quadrilateral(quadrilateral::Quadrilateral),
    Curve(curve::Curve),
}

pub struct Shape {
    pub kind: ShapeKind,
}

impl Shape {
    pub fn draw(&self, _canvas: &mut Canvas) {
        match &self.kind {
            ShapeKind::Point(p) => p.draw(_canvas),
            ShapeKind::Line(l) => l.draw(_canvas),
            ShapeKind::Circle(c) => c.draw(_canvas),
            ShapeKind::Rectangle(r) => r.draw(_canvas),
            ShapeKind::Ellipse(e) => e.draw(_canvas),
            ShapeKind::Arc(a) => a.draw(_canvas),
            ShapeKind::Triangle(t) => t.draw(_canvas),
            ShapeKind::Quadrilateral(q) => q.draw(_canvas),
            ShapeKind::Curve(c) => c.draw(_canvas),
        }
    }
}