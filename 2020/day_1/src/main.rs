use clap::{Arg, App, SubCommand};
use std::fs::File;
use std::io::prelude::*;

fn read_input_file(input_path: String) -> Result<String, std::io::Error> {
    let mut file = File::open(input_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_input(input_values: String) -> Vec<u64> {
    let mut parsed_values: Vec<&str> = input_values.split("\n").collect();
    // Removing the blank after the last number
    parsed_values.pop();
    let converted_parsed = parsed_values.iter().map(|x| u64::from_str_radix(x, 10).unwrap()).collect();
    converted_parsed
}

fn parse_example_input(input_values: String) -> Vec<u64> {
    let mut parsed_values: Vec<&str> = input_values.split(",").collect();
    // Removing the blank after the last number
    parsed_values.pop();
    let converted_parsed = parsed_values.iter().map(|x| u64::from_str_radix(x, 10).unwrap()).collect();
    converted_parsed
}

fn part_1(input: Vec<u64>) -> u64 {
    for i in input.iter() {
        if input.contains(&(2020-i)) {
            return i*(2020-i);
        }
    }
    
    0_u64
}

fn part_2(input: Vec<u64>) -> u64 {
    for i in input.iter() {
        for n in input.iter() {
           if i+n < 2020 && input.contains(&(2020-(i+n))) {
                return i*n*(2020-(i+n));
            }
        }
    }
    
    0_u64
}

fn main() {
    let matches = App::new("AoC 2020: day_1")
        .version("1.0")
        .author("degausser <Ricky@Hosfelt.io>")
        .about("Solution to AoC day_1")
        .subcommand(SubCommand::with_name("ex1")
            .about("day_1 part_1 example")
            .arg(Arg::with_name("input")
                .short("i")
                .long("input")
                .help("puzzle input on the cmdline")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("part_1")
            .about("day_1 part_1")
            .arg(Arg::with_name("input")
                .short("i")
                .long("input")
                .help("puzzle input as a text file")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("ex2")
            .about("day_1 part_2 example")
            .arg(Arg::with_name("input")
                .short("i")
                .long("input")
                .help("puzzle input on the cmdline")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("part_2")
            .about("day_1 part_2")
            .arg(Arg::with_name("input")
                .short("i")
                .long("input")
                .help("puzzle input as a text file")
                .takes_value(true)))
        .get_matches();

    //day_1 part_1 example
    if let Some(ref matches) = matches.subcommand_matches("ex1") {
        if matches.is_present("input") {
            println!("Get that 2020: {}", part_1(parse_example_input(matches.value_of("input").unwrap().to_string())));
        }
    }
    
    // day_1 part_1   
    if let Some(ref matches) = matches.subcommand_matches("part_1") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(matches.value_of("input").unwrap().to_string());
            let parsed_input = parse_input(total_inputs.unwrap());

            println!("Get that 2020: {}", part_2(parsed_input));
        }
    }

    // day_1 part_2 example
    if let Some(ref matches) = matches.subcommand_matches("ex2") {
        if matches.is_present("input") {
            println!("Get that 2020: {}", part_2(parse_example_input(matches.value_of("input").unwrap().to_string())));
        }
    }
    
    // day_1 part_2
    if let Some(ref matches) = matches.subcommand_matches("part_2") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(matches.value_of("input").unwrap().to_string());
            let parsed_input = parse_input(total_inputs.unwrap());

            println!("Get that 2020: {}", part_2(parsed_input));
        }
    }
}   
