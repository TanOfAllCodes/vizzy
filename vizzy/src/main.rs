use visualad::{Compiler, canvas::Color};
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input.vizzy> <output.png>", args[0]);
        let source = fs::read_to_string(&args[1]).expect("Failed to read input file");
    }

    let source = fs::read_to_string(&args[1]).expect("Failed to read input file");
    let mut compiler = Compiler::new(800, 600, Color::new(212, 120, 120));
    compiler.parse(&source).expect("Parse error");
    compiler.render();
    compiler.save(&args[2]).expect("Failed to save output");
}