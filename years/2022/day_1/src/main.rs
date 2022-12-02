use std::{fmt, fs::File, io::prelude::*};

use anyhow::Context;
use clap::{Arg, Command};

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
    EmptyElves,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::EmptyElves => write!(
                f,
                "There were no elves in the array! Check your input files :D"
            ),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Clone)]
struct Elf {
    snacks: Vec<u32>,
}

impl Elf {
    fn sum(&self) -> u32 {
        self.snacks.iter().sum()
    }

    fn new() -> Self {
        Elf { snacks: vec![] }
    }
}

fn parse_input(input: &str) -> Result<Vec<Elf>, anyhow::Error> {
    let mut elves = vec![];

    let mut elf = Elf::new();

    for line in input.split("\n") {
        if line.trim() == "" {
            elves.push(elf);
            elf = Elf::new();
            continue;
        }
        elf.snacks.push(line.parse::<u32>().unwrap())
    }

    elves.push(elf);

    Ok(elves)
}

fn part_1(elves: Vec<Elf>) -> Result<u32, anyhow::Error> {
    Ok(elves
        .into_iter()
        .max_by_key(|elf| elf.sum())
        .ok_or(Error::EmptyElves)?
        .sum())
}

fn part_2(elves: Vec<Elf>) -> Result<u32, anyhow::Error> {
    let mut elves = elves.clone();
    elves.sort_unstable_by_key(|elf| elf.sum());
    Ok(elves.iter().rev().take(3).map(|elf| elf.sum()).sum())
}

fn main() -> Result<(), anyhow::Error> {
    let matches = Command::new("AoC 2022: day_1")
        .version("1.0")
        .author("Ricky Hosfelt <ricky_github@hosfe.lt>")
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
