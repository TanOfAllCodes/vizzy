# Vizzy

Vizzy is a simple graphics language for drawing shapes on a canvas.

#### Author & License

I, [@TanOfAllCodes](https://github.com/TanOfAllCodes/) am the original author.

## Features

The Vizzy compiler is built on Rust. It implements the following concepts to create a graphics programming language:

- **Lexical Analysis**: The compiler reads the .vizzy file and breaks it into tokens, like "canvas", "point", or "color". This is done using the Pest parser with parser.pest.
- **Parsing**: The compiler checks the tokens against grammar rules in parser.pest to build a structure of commands, such as canvas settings or shapes.
- **Rules**: Grammar rules define valid syntax, like "canvas width=<number> height=<number> color=<string>;". These ensure commands are correct.
- **Intermediate Representation (IR)**: Parsed commands are stored as a list of shapes (e.g., Point, Circle) in memory, forming the IR used for rendering.

**Chain of Events**:
1. The compiler reads the .vizzy file (e.g., main.vizzy).
2. Lexical analysis turns the text into tokens.
3. Parsing matches tokens to grammar rules.
4. Valid commands are converted into an IR (shape objects).
5. The IR is rendered onto a canvas to create output.png.

## Usage

### Building Vizzy

Clone the repository and build Vizzy:

```bash
git clone https://github.com/TanOfAllCodes/vizzy
cd vizzy
cargo build --release
```

### Example 

CD into the `example-project` directory after building Vizzy. 

```bash 
chmod +x run.sh 
./run.sh
```

### Create a Vizzy program (e.g., example.vizzy):

```vizzy
canvas width=800 height=600 color="#FFFFFF";
point x=100 y=100 color="blue";
circle x=200 y=200 radius=50 color="red" fill=true;
rectangle x=300 y=300 width=100 height=80 color="green" fill=false;
```

Run the program:

```bash
./target/release/vizzy example.vizzy output.png
```

View output.png to see your drawing.

### Adding a Dev Dependency

To add a development dependency (e.g., toml-cli for scripts):
Edit `Cargo.toml` to include it under [dev-dependencies]:

```toml
[dev-dependencies]
toml-cli = "0.2"
```

Rebuild the project to install the dependency:

```bash
cargo build --release
```

###  Changing Compiler Path 

To change the path of the compiler, change the `va_config.toml` file:

```toml
compiler_path = "/home/$USER/Desktop/visualad-workspace/visualad/target/release/vizzy"
```

## Repo 

### Project Structure

The Vizzy repository is organized to support the compiler and user projects:

```bash
├── example-project              # Directory for a sample Vizzy project
│   ├── main.vizzy               # Vizzy program with drawing commands
│   ├── output.png               # Generated image from running main.vizzy
│   ├── run.sh                   # Script to execute the Vizzy compiler
│   └── va_config.toml           # Config file specifying the compiler path
├── LICENSE                      # MIT license file for the project
├── README.md                    # Documentation file (this file)
└── vizzy                        # Directory containing the Vizzy compiler
    ├── Cargo.lock               # Locks dependency versions for reproducibility
    ├── Cargo.toml               # Configures the vizzy binary and dependencies
    └── src                      # Source code for the Vizzy compiler
        ├── canvas.rs            # Handles canvas rendering and colors
        ├── lib.rs               # Core compiler logic and module definitions
        ├── main.rs              # Command-line interface for the compiler
        ├── parser.pest          # Grammar file for parsing .vizzy syntax
        ├── parser.rs            # Logic to parse .vizzy files
        └── shapes               # Directory for shape definitions
            ├── arc.rs           # Defines arc shape properties and rendering
            ├── circle.rs        # Defines circle shape properties and rendering
            ├── curve.rs         # Defines curve shape properties and rendering
            ├── ellipse.rs       # Defines ellipse shape properties and rendering
            ├── line.rs          # Defines line shape properties and rendering
            ├── mod.rs           # Module declarations for shapes
            ├── point.rs         # Defines point shape properties and rendering
            ├── quadrilateral.rs # Defines quadrilateral shape properties and rendering
            ├── rectangle.rs     # Defines rectangle shape properties and rendering
            └── triangle.rs      # Defines triangle shape properties and rendering
```

## License

Vizzy is licensed under the MIT License. See LICENSE for details.
