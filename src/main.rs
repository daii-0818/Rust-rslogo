use clap::Parser;
use unsvg::Image;
use unsvg::Color;
use unsvg::get_end_coordinates;
use std::collections::HashMap;
use std::fs;


/// A simple program to parse four arguments using clap.
#[derive(Parser)]
struct Args {
    /// Path to a file
    file_path: std::path::PathBuf,

    /// Path to an svg or png image
    image_path: std::path::PathBuf,

    /// Height
    height: u32,

    /// Width
    width: u32,
}

struct Turtle {
    x: f32,
    y: f32,
    angle: f32,
    pen_down: bool,
    pen_color: Color,
    variables: HashMap<String, String>,
}

fn main() -> Result<(), ()> {
    let args: Args = Args::parse();

    // Access the parsed arguments
    let file_path = args.file_path;
    let image_path = args.image_path;
    let height = args.height;
    let width = args.width;

    let mut image = Image::new(width, height);
    let mut turtle = Turtle {
        x: args.width as f32 / 2.0,
        y: args.height as f32 / 2.0,
        angle: 0.0,
        pen_down: false,
        pen_color: Color::black(), // Assume black as default
        variables: HashMap::new(),
    };

    let commands = fs::read_to_string(file_path).expect("Unable to read the file");
    for command in commands.lines() {
        execute_command(command, &mut turtle, &mut image);
    }




    
    
    
    
    
    //save img
    match image_path.extension().map(|s| s.to_str()).flatten() {
        Some("svg") => {
            let res = image.save_svg(&image_path);
            if let Err(e) = res {
                eprintln!("Error saving svg: {e}");
                return Err(());
            }
        }
        Some("png") => {
            let res = image.save_png(&image_path);
            if let Err(e) = res {
                eprintln!("Error saving png: {e}");
                return Err(());
            }
        }
        _ => {
            eprintln!("File extension not supported");
            return Err(());
        }
    }

    Ok(())
}


fn execute_command(command: &str, turtle: &mut Turtle, image: &mut Image) -> Result<(), String>  {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        return Err("Command is empty".to_string());
    }
    match parts[0] {
        "PENUP" => turtle.pen_down = false,
        "PENDOWN" => turtle.pen_down = true,
        "FORWARD" => {
            let distance = parts[1].parse::<f32>().expect("Expected a number for distance");
            move_turtle(distance, turtle, image);
        },
        "BACK" => {
            let distance = parts[1].parse::<f32>().expect("Expected a number for distance");
            move_turtle(-distance, turtle, image);
        },
        "LEFT" => {
            let angle = parts[1].parse::<f32>().expect("Expected a number for angle");
            turtle.angle -= angle;
        },
        "RIGHT" => {
            let angle = parts[1].parse::<f32>().expect("Expected a number for angle");
            turtle.angle += angle;
        },
        "SETPENCOLOR" => {
            let color = parts[1].parse::<usize>().expect("Expected a color index");
            turtle.pen_color = index_to_color(color_index);
        },
        "TURN" => {
            let degrees = parts[1].parse::<f32>().expect("Expected a number for degrees");
            turtle.angle += degrees;
        },
        "SETHEADING" => {
            let degrees = parts[1].parse::<f32>().expect("Expected a number for degrees");
            turtle.angle = degrees;
        },
        "SETX" => {
            let x = parts[1].parse::<f32>().expect("Expected a number for X coordinate");
            turtle.x = x;
        },
        "SETY" => {
            let y = parts[1].parse::<f32>().expect("Expected a number for Y coordinate");
            turtle.y = y;
        },
        "MAKE" => {
            if parts.len() < 3 {
                return Err("MAKE command requires 2 arguments".into());
            }
            let var_name = parts[1].to_string();
            let value = parts[2].to_string();
            turtle.variables.insert(var_name, value);
        },
        "ADDASSIGN" => {
            if parts.len() < 3 {
                return Err("ADDASSIGN command requires 2 arguments".into());
            }
            let var_name = parts[1];
            let add_value = parts[2].parse::<f32>().map_err(|_| "Invalid number for ADDASSIGN")?;
            if let Some(value) = turtle.variables.get(var_name) {
                let current_value = value.parse::<f32>().map_err(|_| "Existing value is not a number")?;
                turtle.variables.insert(var_name.into(), (current_value + add_value).to_string());
            } else {
                return Err("Variable does not exist for ADDASSIGN".into());
            }
        },
        _ => println!("Unknown command: {}", command),
    }
}

fn move_turtle(distance: f32, turtle: &mut Turtle, image: &mut Image) {
    let (new_x, new_y) = unsvg::get_end_coordinates(turtle.x, turtle.y, turtle.angle, distance);


    if turtle.pen_down {
        //let color = unsvg::COLORS[turtle.pen_color];
        let color = index_to_color(turtle.pen_color);
        image.draw_simple_line(turtle.x as i32 as f32, turtle.y as i32 as f32, new_x as i32, new_y as i32 as f32, color);
    }

    turtle.x = new_x;
    turtle.y = new_y;
}


fn index_to_color(index: usize) -> Color {
    match index {
        1 => Color::Red,
        2 => Color::Green,
        3 => Color::Blue,
        _ => Color::Black,
    }
}

