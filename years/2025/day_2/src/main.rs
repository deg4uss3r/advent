use std::{collections::HashMap, fs::File, io::prelude::*};

use anyhow::Context;
use clap::{Arg, Command};

#[derive(Debug)]
struct ProductID {
    start: usize,
    end: usize,
}

fn read_input_file(input_path: &str) -> Result<String, anyhow::Error> {
    let mut file =
        File::open(input_path).context(format!("Error opening input file '{}'", input_path))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .context(format!("Error reading file '{}'", input_path))?;
    Ok(contents)
}

fn parse_input(input: &str) -> Result<Vec<ProductID>, anyhow::Error> {
    let id_ranges = input.split(",").collect::<Vec<&str>>();

    Ok(id_ranges
        .iter()
        .map(|id| {
            let first_last: Vec<&str> = id.split("-").collect();

            Ok(ProductID {
                start: first_last[0].parse::<usize>()?,
                end: first_last[1].parse::<usize>()?,
            })
        })
        .collect::<Result<Vec<ProductID>, anyhow::Error>>()?)
}

fn part_1(ids: Vec<ProductID>) -> Result<usize, anyhow::Error> {
    let mut invaid_id_count = 0;

    for id in ids.iter() {
        for n in id.start..id.end + 1 {
            let mut t = format!("{n}");
            let t_length = t.len();
            if t_length % 2 == 0 {
                let last = t.split_off(t_length / 2);

                if t == last {
                    invaid_id_count += n;
                }
            }
        }
    }

    Ok(invaid_id_count)
}

fn part_2(ids: Vec<ProductID>) -> Result<usize, anyhow::Error> {
    let mut invaid_id_count = 0;

    for id in ids.iter() {
        for n in id.start..id.end + 1 {
            let t = format!("{n}");
            let t_length = t.len();
            let windows = t.chars().map(|s| s.to_string()).collect::<Vec<String>>();

            for range in 1..t_length {
                let ranges: Vec<Vec<String>> = windows.chunks(range).map(|r| r.to_vec()).collect();
                let mut map: HashMap<String, usize> = HashMap::new();
                for v in ranges.iter() {
                    let key = v.join("");
                    let value = map.entry(key).or_insert(0);
                    *value += 1;
                }

                if map.keys().len() == 1 {
                    invaid_id_count += n;
                    break;
                }
            }
        }
    }

    Ok(invaid_id_count)
}

fn main() -> Result<(), anyhow::Error> {
    let matches = Command::new("AoC 2022: day_2")
        .version("1.0")
        .author("Ricky Hosfelt <ricky@hosfe.lt>")
        .about("Solution to AoC day_2")
        .subcommand_required(true)
        .subcommand(
            Command::new("part1").about("day_2 part 1").arg(
                Arg::new("input")
                    .short('i')
                    .long("input")
                    .help("/path/to/puzzle.input")
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("part2").about("day_2 part 2").arg(
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

            println!("day_2 part 1: {:?}", part_1(parse_input(&total_inputs)?)?);
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

            println!("day_2 part 2: {:?}", part_2(parse_input(&total_inputs)?)?);
        }
    }

    Ok(())
}
