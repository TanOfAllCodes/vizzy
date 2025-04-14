use crate::{Compiler, shapes::{Shape, ShapeKind, point::Point, line::Line, circle::Circle, rectangle::Rectangle, ellipse::Ellipse, arc::Arc, triangle::Triangle, quadrilateral::Quadrilateral, curve::Curve}};

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser.pest"]
pub struct VisuaLadParser;

pub fn parse(compiler: &mut Compiler, source: &str) -> Result<(), String> {
    let pairs = VisuaLadParser::parse(Rule::program, source)
        .map_err(|e| format!("Parse error: {}", e))?;
    for pair in pairs {
        for inner in pair.into_inner() {
            println!("Processing command: {:?}", inner.as_rule());
            match inner.as_rule() {
                Rule::canvas => {
                    let mut width = 800;
                    let mut height = 600;
                    let mut color = crate::canvas::Color::new(255, 255, 255);
                    for p in inner.into_inner() {
                        match p.as_rule() {
                            Rule::canvas_width => width = p.into_inner().next().unwrap().as_str().parse().unwrap_or(800),
                            Rule::canvas_height => height = p.into_inner().next().unwrap().as_str().parse().unwrap_or(600),
                            Rule::color => color = parse_color(p.into_inner().next().unwrap().as_str())?,
                            _ => {}
                        }
                    }
                    println!("Setting canvas: width={}, height={}, color={:?}", width, height, color);
                    compiler.set_canvas(width, height, color);
                }
                Rule::point => {
                    let mut x = 0;
                    let mut y = 0;
                    let mut color = "".to_string();
                    for p in inner.into_inner() {
                        match p.as_rule() {
                            Rule::x => x = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0),
                            Rule::y => y = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0),
                            Rule::color => color = p.into_inner().next().unwrap().as_str().to_string(),
                            _ => {}
                        }
                    }
                    let color = parse_color(&color)?;
                    compiler.add_shape(Shape {
                        kind: ShapeKind::Point(Point { x, y, color }),
                    });
                    println!("Added point: x={}, y={}", x, y);
                }
                Rule::line => {
                    let mut x1 = 0;
                    let mut y1 = 0;
                    let mut x2 = 0;
                    let mut y2 = 0;
                    let mut color = "".to_string();
                    let mut stroke = 1;
                    for p in inner.into_inner() {
                        match p.as_rule() {
                            Rule::x1 => x1 = p.into_inner().next().unwrap().as_str().parse().unwrap(),
                            Rule::y1 => y1 = p.into_inner().next().unwrap().as_str().parse().unwrap(),
                            Rule::x2 => x2 = p.into_inner().next().unwrap().as_str().parse().unwrap(),
                            Rule::y2 => y2 = p.into_inner().next().unwrap().as_str().parse().unwrap(),
                            Rule::color => color = p.into_inner().next().unwrap().as_str().to_string(),
                            Rule::stroke => stroke = p.into_inner().next().unwrap().as_str().parse().unwrap(),
                            _ => {}
                        }
                    }
                    let color = parse_color(&color)?;
                    compiler.add_shape(Shape {
                        kind: ShapeKind::Line(Line { x1, y1, x2, y2, color, stroke }),
                    });
                }
                Rule::circle => {
                    let mut x = 0;
                    let mut y = 0;
                    let mut radius = 0;
                    let mut color = "".to_string();
                    let mut fill = false;
                    let mut stroke = 1;
                    for p in inner.into_inner() {
                        match p.as_rule() {
                            Rule::x => x = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0),
                            Rule::y => y = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0),
                            Rule::radius => radius = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0),
                            Rule::color => color = p.into_inner().next().unwrap().as_str().to_string(),
                            Rule::fill => fill = p.into_inner().next().unwrap().as_str() == "true",
                            Rule::stroke => stroke = p.into_inner().next().unwrap().as_str().parse().unwrap_or(1),
                            _ => {}
                        }
                    }
                    let color = parse_color(&color)?;
                    compiler.add_shape(Shape {
                        kind: ShapeKind::Circle(Circle { x, y, radius, color, fill, stroke }),
                    });
                    println!("Added circle: x={}, y={}, radius={}", x, y, radius);
                }
                Rule::rectangle => {
                    let mut x = 0;
                    let mut y = 0;
                    let mut width = 0;
                    let mut height = 0;
                    let mut color = "".to_string();
                    let mut fill = false;
                    let mut stroke = 1;
                    for p in inner.into_inner() {
                        match p.as_rule() {
                            Rule::x => x = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0),
                            Rule::y => y = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0),
                            Rule::rect_width => width = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0),
                            Rule::rect_height => height = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0),
                            Rule::color => color = p.into_inner().next().unwrap().as_str().to_string(),
                            Rule::fill => fill = p.into_inner().next().unwrap().as_str() == "true",
                            Rule::stroke => stroke = p.into_inner().next().unwrap().as_str().parse().unwrap_or(1),
                            _ => {}
                        }
                    }
                    let color = parse_color(&color)?;
                    compiler.add_shape(Shape {
                        kind: ShapeKind::Rectangle(Rectangle { x, y, width, height, color, fill, stroke }),
                    });
                    println!("Added rectangle: x={}, y={}, width={}, height={}", x, y, width, height);
                }
                Rule::ellipse => {
                    let mut x = 0;
                    let mut y = 0;
                    let mut rx = 0;
                    let mut ry = 0;
                    let mut color = "".to_string();
                    let mut fill = false;
                    let mut stroke = 1;
                    for p in inner.into_inner() {
                        match p.as_rule() {
                            Rule::x => x = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0),
                            Rule::y => y = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0),
                            Rule::rx => rx = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0),
                            Rule::ry => ry = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0),
                            Rule::color => color = p.into_inner().next().unwrap().as_str().to_string(),
                            Rule::fill => fill = p.into_inner().next().unwrap().as_str() == "true",
                            Rule::stroke => stroke = p.into_inner().next().unwrap().as_str().parse().unwrap_or(1),
                            _ => {}
                        }
                    }
                    let color = parse_color(&color)?;
                    compiler.add_shape(Shape {
                        kind: ShapeKind::Ellipse(Ellipse { x, y, rx, ry, color, fill, stroke }),
                    });
                    println!("Added ellipse: x={}, y={}, rx={}, ry={}", x, y, rx, ry);
                }
                Rule::arc => {
                    let mut x = 0;
                    let mut y = 0;
                    let mut radius = 0;
                    let mut start = 0.0;
                    let mut end = 0.0;
                    let mut color = "".to_string();
                    let mut stroke = 1;
                    for p in inner.into_inner() {
                        match p.as_rule() {
                            Rule::x => x = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0),
                            Rule::y => y = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0),
                            Rule::radius => radius = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0),
                            Rule::start => start = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0.0),
                            Rule::end => end = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0.0),
                            Rule::color => color = p.into_inner().next().unwrap().as_str().to_string(),
                            Rule::stroke => stroke = p.into_inner().next().unwrap().as_str().parse().unwrap_or(1),
                            _ => {}
                        }
                    }
                    let color = parse_color(&color)?;
                    compiler.add_shape(Shape {
                        kind: ShapeKind::Arc(Arc { x, y, radius, start, end, color, stroke }),
                    });
                    println!("Added arc: x={}, y={}, radius={}, start={}, end={}", x, y, radius, start, end);
                }
                Rule::triangle => {
                    let mut x1 = 0;
                    let mut y1 = 0;
                    let mut x2 = 0;
                    let mut y2 = 0;
                    let mut x3 = 0;
                    let mut y3 = 0;
                    let mut color = "".to_string();
                    let mut fill = false;
                    let mut stroke = 1;
                    for p in inner.into_inner() {
                        match p.as_rule() {
                            Rule::x1 => x1 = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0),
                            Rule::y1 => y1 = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0),
                            Rule::x2 => x2 = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0),
                            Rule::y2 => y2 = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0),
                            Rule::x3 => x3 = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0),
                            Rule::y3 => y3 = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0),
                            Rule::color => color = p.into_inner().next().unwrap().as_str().to_string(),
                            Rule::fill => fill = p.into_inner().next().unwrap().as_str() == "true",
                            Rule::stroke => stroke = p.into_inner().next().unwrap().as_str().parse().unwrap_or(1),
                            _ => {}
                        }
                    }
                    let color = parse_color(&color)?;
                    compiler.add_shape(Shape {
                        kind: ShapeKind::Triangle(Triangle { x1, y1, x2, y2, x3, y3, color, fill, stroke }),
                    });
                    println!("Added triangle: x1={}, y1={}, x2={}, y2={}, x3={}, y3={}", x1, y1, x2, y2, x3, y3);
                }
                Rule::quadrilateral => {
                    let mut x1 = 0;
                    let mut y1 = 0;
                    let mut x2 = 0;
                    let mut y2 = 0;
                    let mut x3 = 0;
                    let mut y3 = 0;
                    let mut x4 = 0;
                    let mut y4 = 0;
                    let mut color = "".to_string();
                    let mut fill = false;
                    let mut stroke = 1;
                    for p in inner.into_inner() {
                        match p.as_rule() {
                            Rule::x1 => x1 = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0),
                            Rule::y1 => y1 = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0),
                            Rule::x2 => x2 = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0),
                            Rule::y2 => y2 = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0),
                            Rule::x3 => x3 = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0),
                            Rule::y3 => y3 = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0),
                            Rule::x4 => x4 = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0),
                            Rule::y4 => y4 = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0),
                            Rule::color => color = p.into_inner().next().unwrap().as_str().to_string(),
                            Rule::fill => fill = p.into_inner().next().unwrap().as_str() == "true",
                            Rule::stroke => stroke = p.into_inner().next().unwrap().as_str().parse().unwrap_or(1),
                            _ => {}
                        }
                    }
                    let color = parse_color(&color)?;
                    compiler.add_shape(Shape {
                        kind: ShapeKind::Quadrilateral(Quadrilateral { x1, y1, x2, y2, x3, y3, x4, y4, color, fill, stroke }),
                    });
                    println!("Added quadrilateral: x1={}, y1={}, x2={}, y2={}, x3={}, y3={}, x4={}, y4={}", x1, y1, x2, y2, x3, y3, x4, y4);
                }
                Rule::curve => {
                    let mut x1 = 0;
                    let mut y1 = 0;
                    let mut x2 = 0;
                    let mut y2 = 0;
                    let mut x3 = 0;
                    let mut y3 = 0;
                    let mut color = "".to_string();
                    let mut stroke = 1;
                    for p in inner.into_inner() {
                        match p.as_rule() {
                            Rule::x1 => x1 = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0),
                            Rule::y1 => y1 = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0),
                            Rule::x2 => x2 = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0),
                            Rule::y2 => y2 = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0),
                            Rule::x3 => x3 = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0),
                            Rule::y3 => y3 = p.into_inner().next().unwrap().as_str().parse().unwrap_or(0),
                            Rule::color => color = p.into_inner().next().unwrap().as_str().to_string(),
                            Rule::stroke => stroke = p.into_inner().next().unwrap().as_str().parse().unwrap_or(1),
                            _ => {}
                        }
                    }
                    let color = parse_color(&color)?;
                    compiler.add_shape(Shape {
                        kind: ShapeKind::Curve(Curve { x1, y1, x2, y2, x3, y3, color, stroke }),
                    });
                    println!("Added curve: x1={}, y1={}, x2={}, y2={}, x3={}, y3={}", x1, y1, x2, y2, x3, y3);
                }
                Rule::help => {
                    let ident = inner.into_inner().next().unwrap().as_str();
                    if let Some(help_text) = compiler.get_help(ident) {
                        println!("{}", help_text);
                    } else {
                        println!("No help available for '{}'", ident);
                    }
                }
                _ => println!("Ignored rule: {:?}", inner.as_rule()),
            }
        }
    }
    Ok(())
}

fn parse_color(color: &str) -> Result<crate::canvas::Color, String> {
    let color = color.trim_matches('"').to_lowercase();
    if color.starts_with('#') && color.len() == 7 {
        let r = u8::from_str_radix(&color[1..3], 16).map_err(|_| format!("Invalid hex: {}", color))?;
        let g = u8::from_str_radix(&color[3..5], 16).map_err(|_| format!("Invalid hex: {}", color))?;
        let b = u8::from_str_radix(&color[5..7], 16).map_err(|_| format!("Invalid hex: {}", color))?;
        return Ok(crate::canvas::Color::new(r, g, b));
    }
    match color.as_str() {
        "red" => Ok(crate::canvas::Color::new(255, 0, 0)),
        "green" => Ok(crate::canvas::Color::new(0, 255, 0)),
        "blue" => Ok(crate::canvas::Color::new(0, 0, 255)),
        "black" => Ok(crate::canvas::Color::new(0, 0, 0)),
        "white" => Ok(crate::canvas::Color::new(255, 255, 255)),
        "yellow" => Ok(crate::canvas::Color::new(255, 255, 0)),
        "cyan" => Ok(crate::canvas::Color::new(0, 255, 255)),
        "magenta" => Ok(crate::canvas::Color::new(255, 0, 255)),
        "gray" => Ok(crate::canvas::Color::new(128, 128, 128)),
        "orange" => Ok(crate::canvas::Color::new(255, 165, 0)),
        "purple" => Ok(crate::canvas::Color::new(128, 0, 128)),
        "brown" => Ok(crate::canvas::Color::new(165, 42, 42)),
        "pink" => Ok(crate::canvas::Color::new(255, 192, 203)),
        "lime" => Ok(crate::canvas::Color::new(0, 255, 0)),
        "navy" => Ok(crate::canvas::Color::new(0, 0, 128)),
        "teal" => Ok(crate::canvas::Color::new(0, 128, 128)),
        "olive" => Ok(crate::canvas::Color::new(128, 128, 0)),
        "silver" => Ok(crate::canvas::Color::new(192, 192, 192)),
        "maroon" => Ok(crate::canvas::Color::new(128, 0, 0)),
        "aqua" => Ok(crate::canvas::Color::new(0, 255, 255)),
        "coral" => Ok(crate::canvas::Color::new(255, 127, 80)),
        "khaki" => Ok(crate::canvas::Color::new(240, 230, 140)),
        "indigo" => Ok(crate::canvas::Color::new(75, 0, 130)),
        "gold" => Ok(crate::canvas::Color::new(255, 215, 0)),
        "violet" => Ok(crate::canvas::Color::new(238, 130, 238)),
        "lavender" => Ok(crate::canvas::Color::new(230, 230, 250)),
        "salmon" => Ok(crate::canvas::Color::new(250, 128, 114)),
        "tomato" => Ok(crate::canvas::Color::new(255, 99, 71)),
        "orchid" => Ok(crate::canvas::Color::new(218, 112, 214)),
        "plum" => Ok(crate::canvas::Color::new(221, 160, 221)),
        "peru" => Ok(crate::canvas::Color::new(205, 133, 63)),
        "chocolate" => Ok(crate::canvas::Color::new(210, 105, 30)),
        "sienna" => Ok(crate::canvas::Color::new(160, 82, 45)),
        "tan" => Ok(crate::canvas::Color::new(210, 180, 140)),
        "wheat" => Ok(crate::canvas::Color::new(222, 184, 135)),
        "bisque" => Ok(crate::canvas::Color::new(255, 228, 196)),
        "peach" => Ok(crate::canvas::Color::new(255, 218, 185)),
        "turquoise" => Ok(crate::canvas::Color::new(64, 224, 208)), 
        _ => Err(format!("Unknown color: {}", color)),
    }
}