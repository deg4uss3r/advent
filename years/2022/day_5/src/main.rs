use std::{fmt, fs::File, io::prelude::*, usize};

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

#[derive(Debug, Clone)]
enum Crates {
    EmptySpace,
    Crate(String),
}

impl Crates {
    fn inner(&self) -> String {
        match self {
            Crates::Crate(s) => s.to_owned(),
            _ => String::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct Warehouse {
    stacks: Vec<Vec<Crates>>,
}

#[derive(Debug)]
struct Instruction {
    crates: usize,
    source: usize,
    destination: usize,
}

fn parse_input(input: &str) -> Result<(Warehouse, Vec<Instruction>), anyhow::Error> {
    let mut rows = vec![];
    let mut instructions = vec![];

    let mut instruction_time = false;

    for line in input.split("\n") {
        if line == "" {
            //move onto the instructions this is the blank line between the stack and instructions
            instruction_time = true;
        } else if instruction_time {
            let ins = line.split(" ").collect::<Vec<&str>>();
            {
                instructions.push(Instruction {
                    crates: ins[1].parse::<usize>()?,
                    source: ins[3].parse::<usize>()? - 1,
                    destination: ins[5].parse::<usize>()? - 1,
                })
            }
        } else {
            // this will be the row
            let mut stack = vec![];
            let mut space_counter = 0;
            for c in line.split(" ") {
                if c == "" {
                    if space_counter == 3 {
                        stack.push(Crates::EmptySpace);
                        space_counter = 0;
                    } else {
                        space_counter += 1;
                    }
                } else if c.parse::<u32>().is_ok() {
                    continue;
                } else {
                    stack.push(Crates::Crate(
                        c.replace("[", "").replace("]", "").to_string(),
                    ));
                }
            }
            rows.push(stack);
        }
    }

    // removes the empty line after the stack numbers
    rows.remove(rows.len() - 1);

    let columns = rows
        .iter()
        .map(|c| {
            let mut i = 0;
            for cr in c.iter() {
                match cr {
                    Crates::EmptySpace => continue,
                    _ => i += 1,
                }
            }
            i
        })
        .max()
        .unwrap();

    let mut warehouse = Warehouse {
        stacks: vec![vec![]; columns],
    };

    for row in rows.iter().rev() {
        for (i, c) in row.iter().enumerate() {
            match c {
                Crates::EmptySpace => continue,
                _ => warehouse.stacks[i].push(c.clone()),
            }
        }
    }

    Ok((warehouse, instructions))
}

fn part_1(input: (Warehouse, Vec<Instruction>)) -> Result<String, anyhow::Error> {
    let (warehousef, instructions) = input;
    let mut warehouse = warehousef.clone();

    for ins in instructions {
        for _ in 0..ins.crates {
            let column_len = warehouse.stacks[ins.source].len();
            if column_len == 0 {
                continue;
            } else {
                let moving_crate = warehouse.stacks[ins.source].remove(column_len - 1);
                warehouse.stacks[ins.destination].push(moving_crate);
            }
        }
    }

    Ok(String::from(
        warehouse
            .stacks
            .iter()
            .map(|s| s[s.len() - 1].inner())
            .collect::<String>(),
    ))
}

fn part_2(input: (Warehouse, Vec<Instruction>)) -> Result<String, anyhow::Error> {
    let (warehousef, instructions) = input;
    let mut warehouse = warehousef.clone();

    for ins in instructions {
        let stack_size = warehouse.stacks[ins.source].len() - 1;
        let crates: Vec<Crates> = warehouse.stacks[ins.source]
            .drain(stack_size - (ins.crates - 1)..)
            .collect();

        warehouse.stacks[ins.destination].extend(crates);
    }

    Ok(String::from(
        warehouse
            .stacks
            .iter()
            .map(|s| s[s.len() - 1].inner())
            .collect::<String>(),
    ))
}

fn main() -> Result<(), anyhow::Error> {
    let matches = Command::new("AoC 2022: day_5")
        .version("1.0")
        .author("Ricky Hosfelt <ricky@hosfe.lt>")
        .about("Solution to AoC day_5")
        .subcommand_required(true)
        .subcommand(
            Command::new("part1").about("day_5 part 1").arg(
                Arg::new("input")
                    .short('i')
                    .long("input")
                    .help("/path/to/puzzle.input")
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("part2").about("day_5 part 2").arg(
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

            println!("day_5 part 1: {:?}", part_1(parse_input(&total_inputs)?)?);
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

            println!("day_5 part 2: {:?}", part_2(parse_input(&total_inputs)?)?);
        }
    }

    Ok(())
}
