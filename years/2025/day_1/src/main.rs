use std::{fs::File, io::prelude::*};

use anyhow::Context;
use clap::{Arg, Command};
use thiserror::Error;

#[derive(Error, Debug)]
enum Day1Error {
    #[error("Invalid input: {0}")]
    ParseError(String),
}

#[derive(Debug, PartialEq)]
enum Rotate {
    Left,
    Right,
}

#[derive(Debug)]
struct Instruction {
    direction: Rotate,
    distance: usize,
}

fn read_input_file(input_path: &str) -> Result<String, anyhow::Error> {
    let mut file =
        File::open(input_path).context(format!("Error opening input file '{}'", input_path))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .context(format!("Error reading file '{}'", input_path))?;
    Ok(contents)
}

fn parse_input(input: &str) -> Result<Vec<Instruction>, anyhow::Error> {
    Ok(input
        .lines()
        .map(|d| {
            if d.contains("L") {
                let distance = d.replace("L", "").parse::<usize>().map_err(|_| {
                    Day1Error::ParseError(String::from("invalid distance to rotate"))
                })?;

                Ok(Instruction {
                    direction: Rotate::Left,
                    distance,
                })
            } else if d.contains("R") {
                let distance = d.replace("R", "").parse::<usize>().map_err(|_| {
                    Day1Error::ParseError(String::from("invalid distance to rotate"))
                })?;

                Ok(Instruction {
                    direction: Rotate::Right,
                    distance,
                })
            } else {
                Err(Day1Error::ParseError(String::from(
                    "malformed instruction {d}",
                )))
            }
        })
        .collect::<Result<Vec<Instruction>, Day1Error>>()?)
}

fn part_1(instructions: Vec<Instruction>) -> Result<usize, anyhow::Error> {
    let mut dial_point: i64 = 50;
    let mut zero_count = 0;

    instructions.iter().for_each(|i| {
        if i.direction == Rotate::Left {
            dial_point -= i.distance as i64;
        } else {
            dial_point += i.distance as i64;
        }

        if dial_point % 100 == 0 {
            zero_count += 1;
        }
    });

    Ok(zero_count)
}

fn part_2(instructions: Vec<Instruction>) -> Result<usize, anyhow::Error> {
    let mut dial_point: i64 = 50;
    let mut zero_count = 0;

    instructions.iter().for_each(|i| {
        if i.direction == Rotate::Left {
            let mut turns = 0;

            while turns < i.distance {
                dial_point -= 1_i64;
                if dial_point % 100 == 0 {
                    zero_count += 1;
                }

                turns += 1;
            }
        } else {
            let mut turns = 0;

            while turns < i.distance {
                dial_point += 1_i64;
                if dial_point % 100 == 0 {
                    zero_count += 1;
                }

                turns += 1;
            }
        }
    });

    Ok(zero_count)
}

fn main() -> Result<(), anyhow::Error> {
    let matches = Command::new("AoC 2022: day_1")
        .version("1.0")
        .author("Ricky Hosfelt <ricky@hosfe.lt>")
        .about("Solution to AoC day_1")
        .subcommand_required(true)
        .subcommand(
            Command::new("part1").about("day_1 part 1").arg(
                Arg::new("input")
                    .short('i')
                    .long("input")
                    .help("/path/to/puzzle.input")
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("part2").about("day_1 part 2").arg(
                Arg::new("input")
                    .short('i')
                    .long("input")
                    .help("/path/to/puzzle.input")
                    .required(true),
            ),
        )
        .get_matches();

    // day_1 part_1
    if let Some(ref commands) = matches.subcommand_matches("part1") {
        if commands.try_contains_id("input")? {
            let total_inputs = read_input_file(
                commands
                    .get_one::<String>("input")
                    .context("Error no value supplied for --input")?,
            )?;

            println!("day_1 part 1: {:?}", part_1(parse_input(&total_inputs)?)?);
        }
    }
    // day_1 part_2
    if let Some(ref commands) = matches.subcommand_matches("part2") {
        if commands.try_contains_id("input")? {
            let total_inputs = read_input_file(
                commands
                    .get_one::<String>("input")
                    .context("Error no value supplied for --input")?,
            )?;

            println!("day_1 part 2: {:?}", part_2(parse_input(&total_inputs)?)?);
        }
    }

    Ok(())
}
