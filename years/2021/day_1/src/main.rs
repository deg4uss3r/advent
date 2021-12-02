use std::{fs::File, io::prelude::*};

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

fn parse_input(input: &str) -> Result<Vec<u64>, anyhow::Error> {
    // each new line represents a depth reading
    let str_depths: Vec<&str> = input.split("\n").collect();

    Ok(str_depths
        .iter()
        .map(|s| s.parse::<u64>())
        .collect::<Result<Vec<u64>, _>>()?)
}

fn part_1(puzz_input: Vec<u64>) -> Result<usize, anyhow::Error> {
    use itertools::Itertools;

    Ok(puzz_input
        .iter()
        .tuple_windows()
        .filter(|(t, n)| n > t)
        .count())
}

fn part_2(puzz_input: Vec<u64>) -> Result<usize, anyhow::Error> {
    use itertools::Itertools;

    // sliding windows of 3, turned into sums
    let depth_windows: Vec<u64> = puzz_input
        .iter()
        .tuple_windows::<(_, _, _)>()
        .map(|(p, n, q)| p + n + q)
        .collect();

    // count if the sums increase
    Ok(depth_windows
        .iter()
        .tuple_windows()
        .filter(|(t, n)| n > t)
        .count())
}

fn main() -> Result<(), anyhow::Error> {
    let matches = App::new("AoC 2021: day_1")
        .version("1.0")
        .author("rth")
        .about("Solution to AoC day_1")
        .subcommand(
            SubCommand::with_name("part1").about("day_1 part1").arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .help("/path/to/puzzle.input")
                    .required(true)
                    .takes_value(true),
            ),
        )
        .subcommand(
            SubCommand::with_name("part2").about("day_1 part2").arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .help("/path/to/puzzle.input")
                    .required(true)
                    .takes_value(true),
            ),
        )
        .get_matches();

    // day_1 part_1
    if let Some(ref matches) = matches.subcommand_matches("part1") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(
                matches
                    .value_of("input")
                    .context("Error no value supplied for --input")?,
            )?;

            println!("day_1 part 1: {}", part_1(parse_input(&total_inputs)?)?);
        }
    }

    // day_1 part_2
    if let Some(ref matches) = matches.subcommand_matches("part2") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(
                matches
                    .value_of("input")
                    .context("Error no value supplied for --input")?,
            )?;

            println!("day_1 part 2: {}", part_2(parse_input(&total_inputs)?)?);
        }
    }

    Ok(())
}
