use clap::{Arg, App, SubCommand};
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn read_input_file(input_path: String) -> Result<String, std::io::Error> {
    let mut file = File::open(input_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_string())
}

fn parse_input(input_values: String) -> Vec<Vec<String>> {
    let lines: Vec<&str> = input_values.split("\n").collect();

    let mut parsed_lines: Vec<Vec<String>> = Vec::new();

    for line in lines.iter() {
        let parsed_values: Vec<String> = line.split(",").map(|x| x.to_string()).collect();
        parsed_lines.append(&mut vec![parsed_values]);
    }
   
    parsed_lines
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Point {
    x: i64, 
    y: i64,
}

trait Movement {
    fn move_right(starting_point: Point) -> Point;
    fn move_left(starting_point: Point) -> Point;
    fn move_up(starting_point: Point) -> Point;
    fn move_down(starting_point: Point) -> Point;
}

impl Movement for Point {
    fn move_right(starting_point: Point) -> Point {
        Point{x: starting_point.x+1, y: starting_point.y}
    } 

    fn move_left(starting_point: Point) -> Point {
        Point{x: starting_point.x-1, y: starting_point.y}
    }

    fn move_up(starting_point: Point) -> Point {
        Point{x: starting_point.x, y: starting_point.y+1}
    } 

    fn move_down(starting_point: Point) -> Point {
        Point{x: starting_point.x, y: starting_point.y-1}
    } 
}

trait Distance {
    fn manhattan(self) -> u64;
}

impl Distance for Point {
    // Calculated the Manhattan distance from the origin point (0,0)
    fn manhattan(self) -> u64 {
        self.x.abs() as u64 + self.y.abs() as u64
    }
}

fn trace_wire(commands: &Vec<String>) -> HashSet<Point> {
    // Get command direction
    let mut wire_trace: HashSet<Point> = HashSet::new();
    //Starting at origin (0,0)
    let mut wire = Point{x: 0, y: 0};

    for command in commands.iter() {
        let direction = &command.to_string()[0..1];
        let distance: u64 = u64::from_str_radix(&command[1..].to_string(), 10).expect("Error could not extract distance from command");
        let mut i_d = 0;

        if direction == "R" {
            while i_d < distance as usize {
                wire = Point::move_right(wire);
                wire_trace.insert(wire);
                i_d += 1;
            }
        } else if direction == "L" {
            while i_d < distance as usize {
                wire = Point::move_left(wire);
                wire_trace.insert(wire);
                i_d += 1;
            }
        } else if direction == "U" {
            while i_d < distance as usize {
                wire = Point::move_up(wire);
                wire_trace.insert(wire);
                i_d += 1;
            }

        } else if direction == "D" {
            while i_d < distance as usize {
                wire = Point::move_down(wire);
                wire_trace.insert(wire);
                i_d += 1;
            }
        }
    }

    wire_trace
}

fn trace_wire_ordered(commands: &Vec<String>) -> Vec<Point> {
    // Get command direction
    let mut wire_trace: Vec<Point> = Vec::new();
    //Starting at origin (0,0)
    let mut wire = Point{x: 0, y: 0};

    for command in commands.iter() {
        let direction = &command.to_string()[0..1];
        let distance: u64 = u64::from_str_radix(&command[1..].to_string(), 10).expect("Error could not extract distance from command");
        let mut i_d = 0;

        if direction == "R" {
            while i_d < distance as usize {
                wire = Point::move_right(wire);
                wire_trace.append(&mut vec!(wire));
                i_d += 1;
            }
        } else if direction == "L" {
            while i_d < distance as usize {
                wire = Point::move_left(wire);
                wire_trace.append(&mut vec!(wire));
                i_d += 1;
            }
        } else if direction == "U" {
            while i_d < distance as usize {
                wire = Point::move_up(wire);
                wire_trace.append(&mut vec!(wire));
                i_d += 1;
            }

        } else if direction == "D" {
            while i_d < distance as usize {
                wire = Point::move_down(wire);
                wire_trace.append(&mut vec!(wire));
                i_d += 1;
            }
        }
    }

    wire_trace
}

fn main() {
    let matches = App::new("AoC: day_3")
        .version("1.0")
        .author("degausser <Ricky@Hosfelt.io>")
        .about("Solution to AoC day_3")
        .subcommand(SubCommand::with_name("ex1")
            .about("day_3 part_1 example")
            .arg(Arg::with_name("input")
                .short("i")
                .long("input")
                .help("input of mass (one number to test)")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("part_1")
            .about("day_3 part_1")
            .arg(Arg::with_name("input")
                .short("i")
                .long("input")
                .help("input of masses (your program input) in a file with new line spaces")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("ex2")
            .about("day_3 part_2 example")
            .arg(Arg::with_name("input")
                .short("i")
                .long("input")
                .help("input of mass (one number to test)")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("part_2")
            .about("day_3 part_2")
            .arg(Arg::with_name("input")
                .short("i")
                .long("input")
                .help("input of masses (your program input) in a file with new line spaces")
                .takes_value(true)))
        .get_matches();

    //day_3 part_1 example
    if let Some(ref matches) = matches.subcommand_matches("ex1") {
        if matches.is_present("input") {
            let i_values = matches.value_of("input").unwrap().to_string();
            let values_parsed = parse_input(i_values);

            for wire in values_parsed.iter() {
                let t_w = trace_wire(&wire);
                println!("{:?}", t_w);
            }
        }
    }
    
    // day_3 part_1   
    if let Some(ref matches) = matches.subcommand_matches("part_1") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(matches.value_of("input").unwrap().to_string());
            let values_parsed = parse_input(total_inputs.unwrap());

            //in values_parsed there are two wires
            let first_wire = trace_wire(&values_parsed[0]);
            let second_wire = trace_wire(&values_parsed[1]);
            let wire_intersections = first_wire.intersection(&second_wire); 

            let mut manhattan_d = u64::max_value();

            for point in wire_intersections {
                if point.manhattan() < manhattan_d {
                    manhattan_d = point.manhattan();
                }
            }

            println!("Closest manhattan distance to origin: {}", manhattan_d);
        }
    }

    //day_3 part_2 example (for quick testing)
    if let Some(ref matches) = matches.subcommand_matches("ex2") {
        if matches.is_present("input") {
            let i_values = matches.value_of("input").unwrap().to_string();
            let values_parsed = parse_input(i_values);
            for value in values_parsed.iter() {
                let t_w = trace_wire_ordered(&value);
                println!("{:?}", t_w);
            }
        }
    }

    // day_3 part_2
    if let Some(ref matches) = matches.subcommand_matches("part_2") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(matches.value_of("input").unwrap().to_string());
            let values_parsed = parse_input(total_inputs.unwrap());

            //in values_parsed there are two wires
            let first_wire = trace_wire(&values_parsed[0]);
            let second_wire = trace_wire(&values_parsed[1]);
            let wire_intersections = first_wire.intersection(&second_wire);
            let first_wire_ordered = trace_wire_ordered(&values_parsed[0]);
            let second_wire_ordered = trace_wire_ordered(&values_parsed[1]);

            let mut delay: u64 = u64::max_value();

            for inter in wire_intersections {
                let inter_d = first_wire_ordered.iter().position(|&r| &r == inter).unwrap() as u64 + second_wire_ordered.iter().position(|&s| &s == inter).unwrap() as u64;
                if inter_d < delay {
                    delay = inter_d;
                }
            }

            // Adding +2 since I don't count the first step from origin to point 1
            // it's 2, for both wires 
            println!("Shortest intersected delay: {}", delay+2);
        }
    }
}  
