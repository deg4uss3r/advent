use std::{fmt, fs::File, io::prelude::*};

use anyhow::Context;
use clap::{Arg, Command};

#[derive(Debug)]
enum Error {
    PacketErr,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::PacketErr => write!(
                f,
                "packet window does not have the correct amount of elements"
            ),
        }
    }
}

impl std::error::Error for Error {}

fn read_input_file(input_path: &str) -> Result<String, anyhow::Error> {
    let mut file =
        File::open(input_path).context(format!("Error opening input file '{}'", input_path))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .context(format!("Error reading file '{}'", input_path))?;
    Ok(contents)
}

fn parse_input(input: &str) -> Result<Vec<char>, anyhow::Error> {
    Ok(input.chars().collect())
}

fn is_unique(input: &[char]) -> bool {
    use std::collections::HashSet;

    let mut uniq = HashSet::new();
    input.iter().all(move |x| uniq.insert(x))
}

fn part_1(packet: Vec<char>) -> Result<u32, anyhow::Error> {
    let mut character_counter = 0;

    for window in packet.windows(4) {
        if is_unique(window) {
            return Ok(character_counter + 4);
        } else {
            character_counter += 1;
            continue;
        }
    }
    Err(Error::PacketErr)?
}

fn part_2(packet: Vec<char>) -> Result<u32, anyhow::Error> {
    let mut character_counter = 0;

    for window in packet.windows(14) {
        if is_unique(window) {
            return Ok(character_counter + 14);
        } else {
            character_counter += 1;
            continue;
        }
    }
    Err(Error::PacketErr)?
}

fn main() -> Result<(), anyhow::Error> {
    let matches = Command::new("AoC 2022: day_6")
        .version("1.0")
        .author("Ricky Hosfelt <ricky@hosfe.lt>")
        .about("Solution to AoC day_6")
        .subcommand_required(true)
        .subcommand(
            Command::new("part1").about("day_6 part 1").arg(
                Arg::new("input")
                    .short('i')
                    .long("input")
                    .help("/path/to/puzzle.input")
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("part2").about("day_6 part 2").arg(
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

            println!("day_6 part 1: {:?}", part_1(parse_input(&total_inputs)?)?);
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

            println!("day_6 part 2: {:?}", part_2(parse_input(&total_inputs)?)?);
        }
    }

    Ok(())
}
