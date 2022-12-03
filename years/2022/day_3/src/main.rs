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

type Items = char;

#[derive(Debug)]
struct Backpack {
    compartment1: Vec<Items>,
    compartment2: Vec<Items>,
}

#[derive(Debug)]
struct ElfGroup {
    elf1: Vec<Items>,
    elf2: Vec<Items>,
    elf3: Vec<Items>,
}

fn parse_input(input: &str) -> Result<Vec<Backpack>, anyhow::Error> {
    let mut packs = vec![];

    for line in input.split("\n") {
        let items: Vec<char> = line.chars().collect();

        packs.push(Backpack {
            compartment1: items[0..items.len() / 2].to_vec(),
            compartment2: items[items.len() / 2..].to_vec(),
        });
    }

    Ok(packs)
}

fn parse_input2(input: &str) -> Result<Vec<ElfGroup>, anyhow::Error> {
    let mut queue = vec![];

    for lines in input.split("\n").collect::<Vec<&str>>().chunks(3) {
        let mut elves: Vec<Vec<char>> = vec![];

        for line in lines {
            elves.push(line.chars().collect());
        }

        queue.push(ElfGroup {
            elf1: elves[0].to_vec(),
            elf2: elves[1].to_vec(),
            elf3: elves[2].to_vec(),
        });
    }

    Ok(queue)
}

fn score(input: Vec<char>) -> u32 {
    input
        .iter()
        .map(|c| {
            // uppercase is offset by 38 when converting from hex
            if c.is_uppercase() {
                *c as u32 - 38
            } else {
                //lowercase is offset by 96 when converting from hex
                *c as u32 - 96
            }
        })
        .sum()
}

fn part_1(input: Vec<Backpack>) -> Result<u32, anyhow::Error> {
    let mut scores = vec![];
    for pack in input {
        for item in pack.compartment1 {
            if pack.compartment2.contains(&item) {
                scores.push(item);
                break;
            } else {
                continue;
            }
        }
    }

    Ok(score(scores))
}

fn part_2(input: Vec<ElfGroup>) -> Result<u32, anyhow::Error> {
    let mut scores = vec![];
    for elves in input {
        for item in elves.elf1 {
            if elves.elf2.contains(&item) && elves.elf3.contains(&item) {
                scores.push(item);
                break;
            } else {
                continue;
            }
        }
    }

    Ok(score(scores))
}

fn main() -> Result<(), anyhow::Error> {
    let matches = Command::new("AoC 2022: day_3")
        .version("1.0")
        .author("Ricky Hosfelt <ricky@hosfe.lt>")
        .about("Solution to AoC day_3")
        .subcommand_required(true)
        .subcommand(
            Command::new("part1").about("day_3 part 1").arg(
                Arg::new("input")
                    .short('i')
                    .long("input")
                    .help("/path/to/puzzle.input")
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("part2").about("day_3 part 2").arg(
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

            println!("day_3 part 1: {:?}", part_1(parse_input(&total_inputs)?)?);
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

            println!("day_3 part 2: {:?}", part_2(parse_input2(&total_inputs)?)?);
        }
    }

    Ok(())
}
