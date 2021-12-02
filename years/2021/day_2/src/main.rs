use std::{convert::TryFrom, fmt, fs::File, io::prelude::*};

use anyhow::Context;
use clap::{App, Arg, SubCommand};

fn read_input_file(input_path: &str) -> Result<String, anyhow::Error> {
    let mut file =
        File::open(input_path).context(format!("Error opening input file '{}'", input_path))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .context(format!("Error reading file '{}'", input_path))?;
    Ok(contents)
}

#[derive(Debug)]
enum Error {
    FailedToParseLine,
    FailedToParseCommand,
    FailedToParseValue,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::FailedToParseLine => write!(f, "Failed to parse the line"),
            Error::FailedToParseCommand => write!(f, "Failed to parse Command from input"),
            Error::FailedToParseValue => write!(f, "Failed to pase Value from input"),
        }
    }
}

impl std::error::Error for Error {}

enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl TryFrom<&str> for Command {
    type Error = Error;

    fn try_from(input: &str) -> Result<Self, Error> {
        let command_value: Vec<&str> = input.split(" ").collect();

        if command_value.len() != 2 {
            println!("{:?}", command_value);
            return Err(Error::FailedToParseLine);
        }

        let command = command_value[0].to_owned();
        let value = command_value[1].to_owned();

        let parsed_value = value
            .parse::<u32>()
            .map_err(|_| Error::FailedToParseValue)?;

        Ok(match command.to_lowercase().as_str() {
            "forward" => Ok(Command::Forward(parsed_value)),
            "down" => Ok(Command::Down(parsed_value)),
            "up" => Ok(Command::Up(parsed_value)),
            _ => Err(Error::FailedToParseCommand),
        }?)
    }
}

fn parse_input(input: &str) -> Result<Vec<Command>, anyhow::Error> {
    //splitting each newline which will have a command+space+value
    let commands: Vec<&str> = input.trim_end().split("\n").collect();

    Ok(commands
        .iter()
        .map(|s| Command::try_from(*s))
        .collect::<Result<Vec<Command>, _>>()?)
}

fn part_1(commands: Vec<Command>) -> u32 {
    let mut vertical_position: u32 = 0;
    let mut horizontal_position: u32 = 0;

    for command in commands.iter() {
        match command {
            Command::Forward(value) => horizontal_position += value,
            Command::Down(value) => vertical_position += value,
            Command::Up(value) => vertical_position -= value,
        };
    }

    horizontal_position * vertical_position
}

fn part_2(commands: Vec<Command>) -> u32 {
    let mut vertical_position: u32 = 0;
    let mut horizontal_position: u32 = 0;
    let mut aim: u32 = 0;

    for command in commands.iter() {
        match command {
            Command::Forward(value) => {
                horizontal_position += value;
                if aim > 0 {
                    vertical_position += value * aim;
                }
            }
            Command::Down(value) => aim += value,
            Command::Up(value) => aim -= value,
        };
    }

    horizontal_position * vertical_position
}

fn main() -> Result<(), anyhow::Error> {
    let matches = App::new("AoC 2021: day_2")
        .version("1.0")
        .author("Ricky Hosfelt <ricky_github@hosfe.lt>")
        .about("Solution to AoC day_2")
        .subcommand(
            SubCommand::with_name("part1").about("day_2 part 1").arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .help("/path/to/puzzle.input")
                    .required(true)
                    .takes_value(true),
            ),
        )
        .subcommand(
            SubCommand::with_name("part2").about("day_2 part 2").arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .help("/path/to/puzzle.input")
                    .required(true)
                    .takes_value(true),
            ),
        )
        .get_matches();

    // day_2 part_1
    if let Some(ref matches) = matches.subcommand_matches("part1") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(
                matches
                    .value_of("input")
                    .context("Error no value supplied for --input")?,
            )?;

            println!("day_2 part 1: {}", part_1(parse_input(&total_inputs)?));
        }
    }

    // day_2 part_2
    if let Some(ref matches) = matches.subcommand_matches("part2") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(
                matches
                    .value_of("input")
                    .context("Error no value supplied for --input")?,
            )?;

            println!("day_2 part 2: {}", part_2(parse_input(&total_inputs)?));
        }
    }

    Ok(())
}
