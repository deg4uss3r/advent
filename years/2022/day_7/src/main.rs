use std::{fmt, fs::File, io::prelude::*};

use anyhow::Context;
use clap::{Arg, Command};
use std::collections::HashMap;

fn read_input_file(input_path: &str) -> Result<String, anyhow::Error> {
    let mut file =
        File::open(input_path).context(format!("Error opening input file '{}'", input_path))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .context(format!("Error reading file '{}'", input_path))?;
    Ok(contents)
}

fn parse_input(input: &str) -> Result<HashMap<String, Vec<u32>>, anyhow::Error> {
    let mut pwd = Vec::new();
    let mut file_system: HashMap<String, Vec<u32>> = HashMap::new();

    for line in input.split("\n") {
        if line.starts_with("$") {
            // a command, cd or ls
            let commands = line.split(" ").collect::<Vec<&str>>();
            if commands[1] == "cd" {
                if commands[2] == ".." {
                    pwd.pop();
                } else {
                    pwd.push(commands[2].to_string());
                    file_system.insert(pwd.join("\\"), vec![]);
                }
            }
        } else {
            //it's a directory or file
            let objects = line.split(" ").collect::<Vec<&str>>();

            if let Ok(size) = objects[0].parse::<u32>() {
                let sizes = file_system.entry(pwd.join("\\")).or_insert_with(Vec::new);

                sizes.push(size);
            }
        }
    }

    Ok(file_system)
}

fn part_1(input: HashMap<String, Vec<u32>>) -> Result<u32, anyhow::Error> {
    let mut sizes: Vec<u32> = vec![];

    for dir in input.keys() {
        let mut size = 0;

        for subdir in input.keys().filter(|subdir| subdir.starts_with(dir)) {
            let subdir_contents = input.get(subdir).unwrap();
            size += subdir_contents.iter().sum::<u32>()
        }

        sizes.push(size);
    }

    // sum all of the directories that are less than 100_000
    Ok(sizes.iter().filter(|x| **x < 100_000_u32).sum())
}

fn part_2(input: HashMap<String, Vec<u32>>) -> Result<u32, anyhow::Error> {
    let mut sizes: Vec<u32> = vec![];

    for dir in input.keys() {
        let mut size = 0;

        for subdir in input.keys().filter(|subdir| subdir.starts_with(dir)) {
            let subdir_contents = input.get(subdir).unwrap();
            size += subdir_contents.iter().sum::<u32>()
        }

        sizes.push(size);
    }

    // minimum space needed is 30_000_000 we probably have _some_ free space under the 70_000_000 limit
    // so find out how much remaining we need to have at least 30_000_000
    let free_space_needed = 30_000_000 - (70_000_000_u32 - sizes.iter().max().unwrap());

    // find the smallest directory that would can remove to free up 30_000_000 space
    let mut min_dirs = sizes
        .into_iter()
        .filter(|x| *x >= free_space_needed)
        .collect::<Vec<u32>>();

    min_dirs.sort();

    Ok(min_dirs[0])
}

fn main() -> Result<(), anyhow::Error> {
    let matches = Command::new("AoC 2022: day_7")
        .version("1.0")
        .author("Ricky Hosfelt <ricky@hosfe.lt>")
        .about("Solution to AoC day_7")
        .subcommand_required(true)
        .subcommand(
            Command::new("part1").about("day_7 part 1").arg(
                Arg::new("input")
                    .short('i')
                    .long("input")
                    .help("/path/to/puzzle.input")
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("part2").about("day_7 part 2").arg(
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

            println!("day_7 part 1: {:?}", part_1(parse_input(&total_inputs)?)?);
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

            println!("day_7 part 2: {:?}", part_2(parse_input(&total_inputs)?)?);
        }
    }

    Ok(())
}
