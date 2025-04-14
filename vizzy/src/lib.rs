
pub mod canvas;
pub mod parser;
pub mod shapes;

use canvas::Canvas;
use shapes::Shape;
use std::collections::HashMap;

pub struct Compiler {
    canvas: Canvas,
    shapes: Vec<Shape>,
    help: HashMap<String, &'static str>,
}

impl Compiler {
    pub fn new(width: u32, height: u32, color: canvas::Color) -> Self {
        let mut help = HashMap::new();
        help.insert("canvas".to_string(), canvas::HELP);
        help.insert("point".to_string(), shapes::point::HELP);
        help.insert("line".to_string(), shapes::line::HELP);
        help.insert("circle".to_string(), shapes::circle::HELP);
        help.insert("rectangle".to_string(), shapes::rectangle::HELP);
        help.insert("ellipse".to_string(), shapes::ellipse::HELP);
        help.insert("arc".to_string(), shapes::arc::HELP);
        help.insert("triangle".to_string(), shapes::triangle::HELP);
        help.insert("quadrilateral".to_string(), shapes::quadrilateral::HELP);
        help.insert("curve".to_string(), shapes::curve::HELP);
        Compiler {
            canvas: Canvas::new(width, height, color),
            shapes: Vec::new(),
            help,
        }
    }

    pub fn parse(&mut self, source: &str) -> Result<(), String> {
        println!("Parsing source: {}", source);
        parser::parse(self, source)
    }

    pub fn render(&mut self) {
        println!("Rendering {} shapes", self.shapes.len());
        for shape in &self.shapes {
            shape.draw(&mut self.canvas);
        }
    }

    pub fn save(&self, filename: &str) -> Result<(), String> {
        self.canvas.save(filename)
    }

    pub fn add_shape(&mut self, shape: Shape) {
        self.shapes.push(shape);
    }

    pub fn set_canvas(&mut self, width: u32, height: u32, color: canvas::Color) {
        self.canvas = Canvas::new(width, height, color);
    }

    pub fn get_help(&self, command: &str) -> Option<&'static str> {
        self.help.get(command).copied()
    }
}