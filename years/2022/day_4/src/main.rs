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
struct CleaningPair {
    elf1: Vec<u32>,
    elf2: Vec<u32>,
}

fn parse_input(input: &str) -> Result<Vec<CleaningPair>, anyhow::Error> {
    let mut cleaners: Vec<CleaningPair> = vec![];
    for line in input.split("\n") {
        let mut elves: Vec<Vec<u32>> = vec![];
        for pair in line.split(",") {
            let start_end: Vec<&str> = pair.split("-").collect();
            elves.push(
                (start_end[0].parse::<u32>()?..=start_end[1].parse::<u32>()?).collect::<Vec<u32>>(),
            );
        }

        cleaners.push(CleaningPair {
            elf1: elves[0].to_vec(),
            elf2: elves[1].to_vec(),
        });
    }

    Ok(cleaners)
}

fn part_1(input: Vec<CleaningPair>) -> Result<u32, anyhow::Error> {
    let mut overlaps = 0;
    for pair in input {
        if pair.elf1.len() >= pair.elf2.len() {
            if pair.elf2.iter().all(|i| pair.elf1.contains(i)) {
                overlaps += 1;
            }
        } else {
            if pair.elf1.iter().all(|i| pair.elf2.contains(i)) {
                overlaps += 1;
            }
        }
    }

    Ok(overlaps)
}

fn part_2(input: Vec<CleaningPair>) -> Result<u32, anyhow::Error> {
    let mut overlaps = 0;
    for pair in input {
        if pair.elf1.len() >= pair.elf2.len() {
            if pair.elf2.iter().any(|i| pair.elf1.contains(i)) {
                overlaps += 1;
            }
        } else {
            if pair.elf1.iter().any(|i| pair.elf2.contains(i)) {
                overlaps += 1;
            }
        }
    }

    Ok(overlaps)
}

fn main() -> Result<(), anyhow::Error> {
    let matches = Command::new("AoC 2022: day_4")
        .version("1.0")
        .author("Ricky Hosfelt <ricky@hosfe.lt>")
        .about("Solution to AoC day_4")
        .subcommand_required(true)
        .subcommand(
            Command::new("part1").about("day_4 part 1").arg(
                Arg::new("input")
                    .short('i')
                    .long("input")
                    .help("/path/to/puzzle.input")
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("part2").about("day_4 part 2").arg(
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

            println!("day_4 part 1: {:?}", part_1(parse_input(&total_inputs)?)?);
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

            println!("day_4 part 2: {:?}", part_2(parse_input(&total_inputs)?)?);
        }
    }

    Ok(())
}
