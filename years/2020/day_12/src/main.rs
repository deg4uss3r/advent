use std::{convert::TryFrom, fmt, fs::File, io::prelude::*};

use anyhow::Context;
use clap::{App, Arg, SubCommand};

type Movements = Vec<Direction>;

fn read_input_file(input_path: &str) -> Result<String, anyhow::Error> {
    let mut file =
        File::open(input_path).context(format!("Error opening input file '{}'", input_path))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .context(format!("Error reading file '{}'", input_path))?;
    Ok(contents)
}

#[derive(Debug)]
enum Direction {
    North(u64),
    South(u64),
    East(u64),
    West(u64),
    Left(u64),
    Right(u64),
    Forward(u64),
}

impl TryFrom<&str> for Direction {
    type Error = ParseError;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let r = input.chars().nth(0).unwrap();
        match r {
            'N' => Ok(Self::North(input[1..].parse::<u64>().map_err(|_e| {
                Self::Error::CannotParseNumber(input[1..].to_string())
            })?)),
            'S' => Ok(Self::South(input[1..].parse::<u64>().map_err(|_e| {
                Self::Error::CannotParseNumber(input[1..].to_string())
            })?)),
            'E' => Ok(Self::East(input[1..].parse::<u64>().map_err(|_e| {
                Self::Error::CannotParseNumber(input[1..].to_string())
            })?)),
            'W' => Ok(Self::West(input[1..].parse::<u64>().map_err(|_e| {
                Self::Error::CannotParseNumber(input[1..].to_string())
            })?)),
            'L' => Ok(Self::Left(input[1..].parse::<u64>().map_err(|_e| {
                Self::Error::CannotParseNumber(input[1..].to_string())
            })?)),
            'R' => Ok(Self::Right(input[1..].parse::<u64>().map_err(|_e| {
                Self::Error::CannotParseNumber(input[1..].to_string())
            })?)),
            'F' => Ok(Self::Forward(input[1..].parse::<u64>().map_err(|_e| {
                Self::Error::CannotParseNumber(input[1..].to_string())
            })?)),
            _ => Err(Self::Error::UnknownInstruction(input.to_string())),
        }
    }
}

#[derive(Debug)]
enum ParseError {
    UnknownInstruction(String),
    CannotParseNumber(String),
}

impl std::error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownInstruction(s) => write!(f, "Unknown Instruction '{}'", s),
            Self::CannotParseNumber(s) => write!(f, "Unknown Number '{}'", s),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialOrd, PartialEq)]
enum Heading {
    North(u64),
    East(u64),
    South(u64),
    West(u64),
}

impl Heading {
    fn set_heading(&self, value: u64) -> Self {
        match self {
            Heading::North(_) => Heading::North(value),
            Heading::South(_) => Heading::South(value),
            Heading::East(_) => Heading::East(value),
            Heading::West(_) => Heading::West(value),
        }
    }

    fn get_value(&self) -> u64 {
        match self {
            Heading::North(value) => *value,
            Heading::South(value) => *value,
            Heading::East(value) => *value,
            Heading::West(value) => *value,
        }
    }
}

#[derive(Debug)]
struct Ship {
    north: u64,
    south: u64,
    east: u64,
    west: u64,
    heading: Heading,
}

#[derive(Debug)]
struct Waypoint {
    north_south: Heading,
    east_west: Heading,
}

trait New {
    fn new() -> Self;
}

impl New  for Waypoint {
    fn new() -> Self {
        Waypoint {
            north_south: Heading::North(1),
            east_west: Heading::East(10),
        }
    }
}

impl New for Ship {
    fn new() -> Self {
        Ship {
            north: 0,
            south: 0,
            east: 0,
            west: 0,
            heading: Heading::East(0),
        }
    }
}

trait Turn {
    fn turn(&mut self, direction: &Direction);
}

impl Turn for Waypoint {
    fn turn(&mut self, direction: &Direction) {
        let mut thevalue = 0;

        let mut compass = vec![
            (Heading::North(0), Heading::East(0)),
            (Heading::South(0), Heading::East(0)),
            (Heading::South(0), Heading::West(0)),
            (Heading::North(0), Heading::West(0)),
        ];

        match (self.north_south, self.east_west) {
            (Heading::North(_), Heading::East(_)) => compass.rotate_left(0),
            (Heading::South(_), Heading::East(_)) => compass.rotate_left(1),
            (Heading::South(_), Heading::West(_)) => compass.rotate_left(2),
            (Heading::North(_), Heading::West(_)) => compass.rotate_left(3),
            _ => (),
        }

        let old_ew = self.east_west.get_value();
        let old_ns = self.north_south.get_value();

        match direction {
            Direction::Left(value) => {
                thevalue = *value;
                compass.rotate_right((value / 90) as usize);
            }
            Direction::Right(value) => {
                thevalue = *value;
                compass.rotate_left((value / 90) as usize);
            }
            _ => (),
        }

        if (thevalue / 90) % 2 == 1 {
            self.north_south = compass[0].0.set_heading(old_ew);
            self.east_west = compass[0].1.set_heading(old_ns);
        } else {
            self.north_south = compass[0].0.set_heading(old_ns);
            self.east_west = compass[0].1.set_heading(old_ew);
        }
    }
}

impl Turn for Ship {
    fn turn(&mut self, direction: &Direction) {
        let mut compass = vec![
            Heading::North(0),
            Heading::East(0),
            Heading::South(0),
            Heading::West(0),
        ];

        //start the compass based off the ship's current direction
        match self.heading {
            Heading::North(_) => compass.rotate_left(0),
            Heading::East(_) => compass.rotate_left(1),
            Heading::South(_) => compass.rotate_left(2),
            Heading::West(_) => compass.rotate_left(3),
        }

        match direction {
            Direction::Left(value) => compass.rotate_right((value / 90) as usize),
            Direction::Right(value) => compass.rotate_left((value / 90) as usize),
            _ => (),
        }

        self.heading = compass[0];
    }
}

impl Waypoint {
    fn update(&mut self, direction: &Direction) {
        match direction {
            Direction::North(v) => match self.north_south {
                Heading::North(wv) => self.north_south = Heading::North(wv + v),
                Heading::South(wv) => {
                    if v > &wv {
                        self.north_south = Heading::North(v - wv);
                    } else {
                        self.north_south = Heading::South(wv - v);
                    }
                }
                _ => (),
            },
            Direction::South(v) => match self.north_south {
                Heading::South(wv) => self.north_south = Heading::South(wv + v),
                Heading::North(wv) => {
                    if v >= &wv {
                        self.north_south = Heading::South(v - wv);
                    } else {
                        self.north_south = Heading::North(wv - v);
                    }
                }
                _ => (),
            },
            Direction::East(v) => match self.east_west {
                Heading::East(wv) => self.east_west = Heading::East(wv + v),
                Heading::West(wv) => {
                    if v > &wv {
                        self.east_west = Heading::East(v - wv);
                    } else {
                        self.east_west = Heading::West(wv - v);
                    }
                }
                _ => (),
            },
            Direction::West(v) => match self.east_west {
                Heading::West(wv) => self.east_west = Heading::West(wv + v),
                Heading::East(wv) => {
                    if v > &wv {
                        self.east_west = Heading::West(v - wv);
                    } else {
                        self.east_west = Heading::East(wv - v);
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }
}

impl Ship {
    fn move_ship(&mut self, direction: Direction) {
        match direction {
            Direction::North(value) => self.north += value,
            Direction::South(value) => self.south += value,
            Direction::East(value) => self.east += value,
            Direction::West(value) => self.west += value,
            Direction::Forward(value) => match self.heading {
                Heading::North(_) => self.north += value,
                Heading::South(_) => self.south += value,
                Heading::East(_) => self.east += value,
                Heading::West(_) => self.west += value,
            },
            Direction::Left(_value) => self.turn(&direction),
            Direction::Right(_value) => self.turn(&direction),
        }
    }

    fn follow_waypoint(&mut self, waypoint: &mut Waypoint, direction: &Direction) {
        match direction {
            Direction::North(_value) => waypoint.update(&direction),
            Direction::South(_value) => waypoint.update(&direction),
            Direction::East(_value) => waypoint.update(&direction),
            Direction::West(_value) => waypoint.update(&direction),
            Direction::Forward(value) => {
                match waypoint.north_south {
                    Heading::North(wayvalue) => self.north += value * wayvalue,
                    Heading::South(wayvalue) => self.south += value * wayvalue,
                    _ => (),
                }

                match waypoint.east_west {
                    Heading::East(wayvalue) => self.east += value * wayvalue,
                    Heading::West(wayvalue) => self.west += value * wayvalue,
                    _ => (),
                }
            }
            Direction::Left(_value) => waypoint.turn(&direction),
            Direction::Right(_value) => waypoint.turn(&direction),
        }
    }

    fn manhattan_distance(&self) -> u64 {
        ((self.east as i64 - self.west as i64).abs()
            + (self.north as i64 - self.south as i64).abs()) as u64
    }
}


fn parse_input(input: &str) -> Result<Movements, ParseError> {
    let moves: Vec<&str> = input.split("\n").collect();
    let mut out: Movements = Vec::new();
    for m in moves {
        if m != "" {
            out.push(Direction::try_from(m)?);
        }
    }

    Ok(out)
}

fn part_1(moves: Movements) -> u64 {
    let mut ship = Ship::new();

    for m in moves {
        ship.move_ship(m);
    }

    ship.manhattan_distance()
}

fn part_2(moves: Movements) -> u64 {
    let mut ship = Ship::new();
    let mut waypoint = Waypoint::new();
    
    for m in moves {
        ship.follow_waypoint(&mut waypoint, &m);
    }

    ship.manhattan_distance()
}

fn main() -> Result<(), anyhow::Error> {
    let matches = App::new("AoC 2020: day_12")
        .version("1.0")
        .author("Ricky <Ricky@Hosfelt.io>")
        .about("Solution to AoC day_12")
        .subcommand(
            SubCommand::with_name("ex1")
                .about("day_12 part_1 example")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .help("puzzle input on the cmdline")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("part_1").about("day_12 part_1").arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .help("puzzle input as a text file")
                    .takes_value(true),
            ),
        )
        .subcommand(
            SubCommand::with_name("ex2")
                .about("day_12 part_2 example")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .help("puzzle input on the cmdline")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("part_2").about("day_12 part_2").arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .help("puzzle input as a text file")
                    .takes_value(true),
            ),
        )
        .get_matches();

    //day_12 part_1 example
    if let Some(ref matches) = matches.subcommand_matches("ex1") {
        if matches.is_present("input") {
            println!(
                "manhattan distance of ship: {}",
                part_1(parse_input(
                    matches
                        .value_of("input")
                        .context("Error no value supplied for --input")?
                )?)
            );
        }
    }

    // day_12 part_1
    if let Some(ref matches) = matches.subcommand_matches("part_1") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(
                matches
                    .value_of("input")
                    .context("Error no value supplied for --input")?,
            )?;
            let parsed_input = parse_input(&total_inputs)?;

            println!("manhattan distance of ship: {}", part_1(parsed_input));
        }
    }

    // day_12 part_2 example
    if let Some(ref matches) = matches.subcommand_matches("ex2") {
        if matches.is_present("input") {
            println!(
                "Manhattan distance while following waypoint: {}",
                part_2(parse_input(
                    matches
                        .value_of("input")
                        .context("Error no value supplied for --input")?
                )?)
            );
        }
    }

    // day_12 part_2
    if let Some(ref matches) = matches.subcommand_matches("part_2") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(
                matches
                    .value_of("input")
                    .context("Error no value supplied for --input")?,
            )?;
            let parsed_input = parse_input(&total_inputs)?;

            println!(
                "Manhattan distance while following waypoint: {}",
                part_2(parsed_input)
            );
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const PUZZLE: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn ex1() {
        assert_eq!(25, part_1(parse_input(PUZZLE).unwrap()));
    }

    #[test]
    fn ex2() {
        assert_eq!(286, part_2(parse_input(PUZZLE).unwrap()));
    }
}
