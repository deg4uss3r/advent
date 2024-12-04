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
struct ListHolder {
    left: Vec<i32>,
    right: Vec<i32>,
}

fn parse_input(input: &str) -> Result<ListHolder, anyhow::Error> {
    let mut lists = ListHolder {
        left: vec![],
        right: vec![],
    };

    let lines = input.split("\n").collect::<Vec<&str>>();

    for line in lines.iter() {
        let mut left = false;
        let nums = line.split(" ").collect::<Vec<&str>>();
        for num in nums {
            if num != "" {
                if !left {
                    lists.left.push(num.parse::<i32>()?);
                    left = true;
                } else {
                    lists.right.push(num.parse::<i32>()?);
                }
            } else {
                continue;
            }
        }
    }

    //pre sort
    lists.left.sort();
    lists.right.sort();

    Ok(lists)
}

fn part_1(input: ListHolder) -> Result<i32, anyhow::Error> {
    // calculate distances
    let mut sum = 0;

    for left_right in input.left.iter().zip(input.right.iter()) {
        sum += (left_right.0 - left_right.1).abs()
    }

    Ok(sum)
}

fn part_2(input: ListHolder) -> Result<i32, anyhow::Error> {
    let mut sum = 0;

    for left in input.left.iter() {
        let right_count: i32 = input
            .right
            .iter()
            .filter(|n| *n == left)
            .count()
            .try_into()?;
        sum += left * right_count;
    }

    Ok(sum)
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
