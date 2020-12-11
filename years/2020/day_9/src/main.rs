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

fn parse_input(input: &str) -> Vec<u64> {
    parse_example_input(input)
}
fn parse_example_input(input: &str) -> Vec<u64> {
    let numbers: Vec<&str> = input.split("\n").collect();
    
    numbers.iter().map(|s| s.parse()).filter(|s| s.is_ok()).map(|s| s.unwrap()).collect::<Vec<u64>>()
}

fn get_preamble(indx: usize, v: Vec<u64>, length: usize) -> Vec<u64> {
    v[indx..indx+length].to_vec()
}

fn is_number_summed_in_preamble(preamble: Vec<u64>, num: u64) -> bool {
    for x in preamble.iter() {
        if &num > x && preamble.contains(&(num-x)) {
            return true;
        }
    }
    
    false
}

fn part_1(numbers: Vec<u64>, length: usize) -> u64 {
    for idx in 0..numbers.len()-length {
        let preamble = get_preamble(idx, numbers.to_vec(), length);
        if is_number_summed_in_preamble(preamble, numbers[idx+length]) {
            continue;
        } else {
            return numbers[idx+length];
        }
    }
    
    0_u64
}

fn part_2(numbers: Vec<u64>) -> u64 {
    for sindx in 0..numbers.len() - 2 {
        for eindx in sindx+1..numbers.len() - 1 {
            if numbers[sindx..eindx].iter().sum::<u64>() == 133015568 {
                let mut v = numbers[sindx..eindx].to_vec();
                v.sort();
                return v[0] + v[v.len()-1];
            }
        }
    }
    
    0_u64
}

fn main() -> Result<(), anyhow::Error> {
    let matches = App::new("AoC 2020: day_9")
        .version("1.0")
        .author("Ricky <Ricky@Hosfelt.io>")
        .about("Solution to AoC day_9")
        .subcommand(
            SubCommand::with_name("ex1")
                .about("day_9 part_1 example")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .help("puzzle input on the cmdline")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("part_1").about("day_9 part_1").arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .help("puzzle input as a text file")
                    .takes_value(true),
            ),
        )
        .subcommand(
            SubCommand::with_name("ex2")
                .about("day_9 part_2 example")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .help("puzzle input on the cmdline")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("part_2").about("day_9 part_2").arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .help("puzzle input as a text file")
                    .takes_value(true),
            ),
        )
        .get_matches();

    //day_9 part_1 example
    if let Some(ref matches) = matches.subcommand_matches("ex1") {
        if matches.is_present("input") {
            println!(
                "Number that can't be summed with preamble: {}",
                part_1(parse_example_input(
                    matches
                        .value_of("input")
                        .context("Error no value supplied for --input")?
                ), 5)
            );
        }
    }

    // day_9 part_1
    if let Some(ref matches) = matches.subcommand_matches("part_1") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(
                matches
                    .value_of("input")
                    .context("Error no value supplied for --input")?,
            )?;
            let parsed_input = parse_input(&total_inputs);

            println!("Number that can't be summed with preamble: {}", part_1(parsed_input, 25));
        }
    }

    // day_9 part_2 example
    if let Some(ref matches) = matches.subcommand_matches("ex2") {
        if matches.is_present("input") {
            println!(
                "series that match bad number: {}",
                part_2(parse_input(
                    matches
                        .value_of("input")
                        .context("Error no value supplied for --input")?
                ))
            );
        }
    }

    // day_9 part_2
    if let Some(ref matches) = matches.subcommand_matches("part_2") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(
                matches
                    .value_of("input")
                    .context("Error no value supplied for --input")?,
            )?;
            let parsed_input = parse_input(&total_inputs);

            println!("series that match bad number: {}", part_2(parsed_input));
        }
    }

    Ok(())
}
