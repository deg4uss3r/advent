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

fn calculate_mass(mass: &u64) -> u64 {
    (mass/3)-2
}

fn calculate_mass_2(mass: &u64) -> u64 {
    let mut r_mass: u64 = mass.clone(); 
    let mut total_fuel: u64 = 0;

    // No more fuel needed, if <=6 because 6/3-2 = 0  
    while r_mass > 6 {
        r_mass = calculate_mass(&r_mass);
        total_fuel += r_mass;
    }

    total_fuel
}

fn main() {
    let matches = App::new("AoC: day_1")
        .version("1.0")
        .author("degausser <Ricky@Hosfelt.io>")
        .about("Solution to AoC day_1")
        .subcommand(SubCommand::with_name("ex1")
            .about("day_1 part_1 example")
            .arg(Arg::with_name("input")
                .short("i")
                .long("input")
                .help("input of mass (one number to test)")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("part_1")
            .about("day_1 part_1")
            .arg(Arg::with_name("input")
                .short("i")
                .long("input")
                .help("input of masses (your program input) in a file with new line spaces")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("ex2")
            .about("day_1 part_2 example")
            .arg(Arg::with_name("input")
                .short("i")
                .long("input")
                .help("input of mass (one number to test)")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("part_2")
            .about("day_1 part_2")
            .arg(Arg::with_name("input")
                .short("i")
                .long("input")
                .help("input of masses (your program input) in a file with new line spaces")
                .takes_value(true)))
        .get_matches();

    //day_1 part_1 example
    if let Some(ref matches) = matches.subcommand_matches("ex1") {
        if matches.is_present("input") {
            println!("sum of mass calculations: {}", calculate_mass(&u64::from_str_radix(matches.value_of("input").unwrap(), 10).unwrap()));
        }
    }
    
    // day_1 part_1   
    if let Some(ref matches) = matches.subcommand_matches("part_1") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(matches.value_of("input").unwrap().to_string());
            let parsed_input = parse_input(total_inputs.unwrap());
            let mut sum = 0;

            for val in parsed_input.iter() {
                sum += calculate_mass(val)
            }

            println!("sum of mass calculations: {}", sum);
        }
    }

    // day_1 part_2 example
    if let Some(ref matches) = matches.subcommand_matches("ex2") {
        if matches.is_present("input") {
            println!("sum of mass calculations: {}", calculate_mass_2(&u64::from_str_radix(matches.value_of("input").unwrap(), 10).unwrap()));
        }
    }
    
    // day_1 part_2
    if let Some(ref matches) = matches.subcommand_matches("part_2") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(matches.value_of("input").unwrap().to_string());
            let parsed_input = parse_input(total_inputs.unwrap());
            let mut sum = 0;

            for val in parsed_input.iter() {
                sum += calculate_mass_2(val)
            }

            println!("sum of mass calculations: {}", sum);
        }
    }
}   
