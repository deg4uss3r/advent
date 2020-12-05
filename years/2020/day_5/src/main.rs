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

#[derive(Debug)]
struct Pass {
    front: String,
    back: String,
}

impl Pass {
    fn get_seat(&self) -> Result<Seat, anyhow::Error> {
        let mut row_str = String::new();
        let mut col_str = String::new();

        for c in self.back.chars() {
            if c == 'R' {
                col_str = format!("{}{}", col_str, 1);
            } else if c == 'L' {
                col_str = format!("{}{}", col_str, 0);
            } else {
                panic!(format!("Invalid column seat direction '{}'", c))
            }
        }

        for r in self.front.chars() {
            if r == 'F' {
                row_str = format!("{}{}", row_str, 0);
            } else if r == 'B' {
                row_str = format!("{}{}", row_str, 1);
            } else {
                panic!(format!("Invalid row seat direction '{}'", r))
            }
        }

        let seat: Seat = Seat {
            row: u64::from_str_radix(&row_str, 2)
                .context(format!("Unable to get row from binary '{}'", row_str))?,
            col: u64::from_str_radix(&col_str, 2)
                .context(format!("Unable to get column from binary '{}'", col_str))?,
        };

        Ok(seat)
    }
}

#[derive(Debug)]
struct Seat {
    row: u64,
    col: u64,
}

impl Seat {
    fn get_seat_id(&self) -> u64 {
        self.row * 8 + self.col
    }
}

fn parse_input(input: &str) -> Vec<Pass> {
    let seats: Vec<&str> = input.split("\n").collect();
    let mut passes: Vec<Pass> = Vec::new();

    for seat in seats.iter() {
        if seat != &"" {
            if seat.len() == 10 {
                let front = seat[..7].to_string();
                let back = seat[7..].to_string();

                passes.push(Pass {
                    front: front,
                    back: back,
                });
            } else {
                panic!("Seat was not 10 characters!");
            }
        }
    }

    passes
}

fn parse_example_input(input: &str) -> Pass {
    if input.len() == 10 {
        let front = input[..7].to_string();
        let back = input[7..].to_string();

        Pass {
            front: front,
            back: back,
        }
    } else {
        panic!("Seat was not 10 characters");
    }
}

fn ex1(pass: Pass) -> Result<u64, anyhow::Error> {
    Ok(pass.get_seat()?.get_seat_id())
}

fn part_1(passes: Vec<Pass>) -> Result<u64, anyhow::Error> {
    let mut highest_id = u64::MIN;

    for pass in passes {
        let this_seat_id = pass.get_seat()?.get_seat_id();

        if this_seat_id > highest_id {
            highest_id = this_seat_id;
        }
    }

    Ok(highest_id)
}

fn part_2(passes: Vec<Pass>) -> Result<u64, anyhow::Error> {
    let mut highest_id = u64::MIN;
    let mut lowest_id = u64::MAX;
    let mut seat_ids: Vec<u64> = Vec::new();

    for pass in passes {
        let this_seat_id = pass.get_seat()?.get_seat_id();

        if this_seat_id > highest_id {
            highest_id = this_seat_id;
        }

        if this_seat_id < lowest_id {
            lowest_id = this_seat_id;
        }

        seat_ids.push(this_seat_id);
    }

    Ok((lowest_id..=highest_id).sum::<u64>() - seat_ids.iter().sum::<u64>())
}

fn main() -> Result<(), anyhow::Error> {
    let matches = App::new("AoC 2020: day_5")
        .version("1.0")
        .author("Ricky <Ricky@Hosfelt.io>")
        .about("Solution to AoC day_5")
        .subcommand(
            SubCommand::with_name("ex1")
                .about("day_5 part_1 example")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .help("puzzle input on the cmdline")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("part_1").about("day_5 part_1").arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .help("puzzle input as a text file")
                    .takes_value(true),
            ),
        )
        .subcommand(
            SubCommand::with_name("ex2")
                .about("day_5 part_2 example")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .help("puzzle input on the cmdline")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("part_2").about("day_5 part_2").arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .help("puzzle input as a text file")
                    .takes_value(true),
            ),
        )
        .get_matches();

    //day_5 part_1 example
    if let Some(ref matches) = matches.subcommand_matches("ex1") {
        if matches.is_present("input") {
            println!(
                "Seat ID: {}",
                ex1(parse_example_input(
                    matches
                        .value_of("input")
                        .context("Error no value supplied for --input")?
                ))?
            );
        }
    }

    // day_5 part_1
    if let Some(ref matches) = matches.subcommand_matches("part_1") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(
                matches
                    .value_of("input")
                    .context("Error no value supplied for --input")?,
            )?;
            let parsed_input = parse_input(&total_inputs);

            println!("Seat ID: {}", part_1(parsed_input)?);
        }
    }

    // day_5 part_2 example
    if let Some(ref matches) = matches.subcommand_matches("ex2") {
        if matches.is_present("input") {
            println!(
                "Your seat: {}",
                part_2(parse_input(
                    matches
                        .value_of("input")
                        .context("Error no value supplied for --input")?
                ))?
            );
        }
    }

    // day_5 part_2
    if let Some(ref matches) = matches.subcommand_matches("part_2") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(
                matches
                    .value_of("input")
                    .context("Error no value supplied for --input")?,
            )?;
            let parsed_input = parse_input(&total_inputs);

            println!("Your seat: {}", part_2(parsed_input)?);
        }
    }
    Ok(())
}
