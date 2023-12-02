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

fn parse_input(input: &str) -> Result<Vec<Vec<u32>>, anyhow::Error> {
    let mut forest = vec![];

    for line in input.split("\n") {
        let mut row = vec![];
        for num in line.trim().split("") {
            if num == "" {
                continue;
            } else {
                row.push(num.parse::<u32>()?)
            }
        }
        forest.push(row);
    }

    Ok(forest)
}

fn part_1(forest: Vec<Vec<u32>>) -> Result<u32, anyhow::Error> {
    let mut trees_seen: u32 = ((forest[0].len() - 1) * (forest.len() - 1)).try_into()?;

    let mut row_tree: Vec<&u32> = forest[0].iter().skip(1).take(forest[0].len() - 2).collect();

    for (forest_index, row) in forest.iter().skip(1).take(forest.len() - 2).enumerate() {
        let mut previous_tree = &forest[forest_index + 1][1];
        for (row_index, tree) in row.iter().skip(1).take(row.len() - 2).enumerate() {
            //for each tree go to the top, bottom, left, and right edges to check if it is visible
            // end early if the next tree is taller or of equal height
            //top
            let mut top = forest_index + 1;
            let mut top_see = 0;
            let mut top_previous_tree = previous_tree;
            while tree > previous_tree && top >= 0 {
                println!("top: comparing {} to {}", tree, top_previous_tree);
                top -= 1;
                top_previous_tree = &forest[top][row_index + 1];
                top_see += 1;
            }

            if top_see == forest_index {
                println!(
                    "saw tree [{}][{}]: {}",
                    forest_index + 1,
                    row_index + 1,
                    tree
                );
                trees_seen += 1;
            }

            //bottom
            let mut bottom = forest_index + 1;
            let mut bottom_see = 0;
            let mut bottom_previous_tree = previous_tree;
            while tree > bottom_previous_tree && bottom < forest.len() {
                println!("bottom: comparing {} to {}", tree, bottom_previous_tree);
                bottom += 1;
                bottom_previous_tree = &forest[bottom][row_index];
                bottom_see += 1;
            }

            if bottom_see == forest.len() - 1 {
                trees_seen += 1;
            }
        }
    }

    Ok(trees_seen)
}

fn part_2(input: Vec<Vec<u32>>) -> Result<(), anyhow::Error> {
    unimplemented!()
}

fn main() -> Result<(), anyhow::Error> {
    let matches = Command::new("AoC 2022: day_8")
        .version("1.0")
        .author("Ricky Hosfelt <ricky@hosfe.lt>")
        .about("Solution to AoC day_8")
        .subcommand_required(true)
        .subcommand(
            Command::new("part1").about("day_8 part 1").arg(
                Arg::new("input")
                    .short('i')
                    .long("input")
                    .help("/path/to/puzzle.input")
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("part2").about("day_8 part 2").arg(
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

            println!("day_8 part 1: {:?}", part_1(parse_input(&total_inputs)?)?);
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

            println!("day_8 part 2: {:?}", part_2(parse_input(&total_inputs)?)?);
        }
    }

    Ok(())
}
