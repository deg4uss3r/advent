use std::{fs::File, io::prelude::*};

use anyhow::Context;
use clap::{App, Arg, SubCommand};

fn read_input_file(input_path: String) -> Result<String, anyhow::Error> {
    let mut file = File::open(input_path).context("Error opening input file '{}'", input_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).context("Error reading file '{}'", input_path)?;
    Ok(contents)
}

fn parse_input(input: &str) -> Result<(), anyhow::Error> {
   Ok(())
}

fn part_1() {
    unimplemented!()
}

//fn part_2() {
//
//}

fn main() -> Result<(), anyhow::Error> {
    let matches = App::new("AoC 2020: {{crate_name}}")
        .version("1.0")
        .author("{{authors}}")
        .about("Solution to AoC {{crate_name}}")
        .subcommand(
            SubCommand::with_name("ex1")
                .about("{{crate_name}} part_1 example")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .help("puzzle input on the cmdline")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("part_1").about("{{crate_name}} part_1").arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .help("puzzle input as a text file")
                    .takes_value(true),
            ),
        )
        .subcommand(
            SubCommand::with_name("ex2")
                .about("{{crate_name}} part_2 example")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .help("puzzle input on the cmdline")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("part_2").about("{{crate_name}} part_2").arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .help("puzzle input as a text file")
                    .takes_value(true),
            ),
        )
        .get_matches();

    //{{crate_name}} part_1 example
    if let Some(ref matches) = matches.subcommand_matches("ex1") {
        if matches.is_present("input") {
            println!(
                ": {}",
                part_1(parse_input(matches.value_of("input").context("Error no value supplied for --input")?)?)
            );
        }
    }

    // {{crate_name}} part_1
    if let Some(ref matches) = matches.subcommand_matches("part_1") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(matches.value_of("input").context("Error no value supplied for --input")?)?;
            let parsed_input = parse_input(total_inputs);

            println!(": {}", part_1(parsed_input));
        }
    }
/*
    // {{crate_name}} part_2 example
    if let Some(ref matches) = matches.subcommand_matches("ex2") {
        if matches.is_present("input") {
            println!(
                ": {}",
                part_2(parse_input(matches.value_of("input").context("Error no value supplied for --input")?)?)
            );
        }
    }

    // {{crate_name}} part_2
    if let Some(ref matches) = matches.subcommand_matches("part_2") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(matches.value_of("input").context("Error no value supplied for --input")?)?;
            let parsed_input = parse_input(total_inputs);

            println!(": {}", part_2(parsed_input));
        }
    }
*/
    Ok(())
}
