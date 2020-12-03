use clap::{Arg, App, SubCommand};

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

type Password = String;
    
#[derive(Debug)]
struct Rules {
    character: char,
    min: u64,
    max: u64,
}

fn read_input_file(input_path: String) -> Result<String, std::io::Error> {
    let mut file = File::open(input_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_input(input: String) -> Vec<(Rules, Password)> {
    let mut password_set: Vec<(Rules, String)> = Vec::new();

    let examples: Vec<&str> = input.split("\n").collect();
    
    for ex in examples.iter() {
        if ex == &"" {
            continue;
        }
        let rule_password: Vec<&str> = ex.split(": ").collect();
        let rule_part: Vec<&str> = rule_password[0].split(" ").collect();
        let min_max: Vec<&str> = rule_part[0].split("-").collect();
        let rule: Rules = Rules{character: rule_part[1].parse().unwrap(), min: min_max[0].parse().unwrap(), max: min_max[1].parse().unwrap()}; 
        let password = rule_password[1].to_string();
        
        password_set.push((rule, password));
    }
    
    password_set
}

fn parse_example_input(input: String) -> Vec<(Rules, Password)> {
    let mut password_set: Vec<(Rules, String)> = Vec::new(); 
    let examples: Vec<&str> = input.split(",").collect();
    for ex in examples.iter() { 
        let rule_password: Vec<&str> = ex.split(": ").collect();
        let rule_part: Vec<&str> = rule_password[0].split(" ").collect();
        let min_max: Vec<&str> = rule_part[0].split("-").collect();
        let rule: Rules = Rules{character: rule_part[1].parse().unwrap(), min: min_max[0].parse().unwrap(), max: min_max[1].parse().unwrap()}; 
        let password = rule_password[1].to_string();
        
        password_set.push((rule, password));
    }
    
    password_set
}

fn part_1(input: Vec<(Rules, Password)>) -> u64 {
    let mut valid_passwords = 0_u64; 
    
    for set in input.iter() {
        let parts: Vec<char> = set.1.chars().collect();
        
        let mut part_count: HashMap<char, u64> = HashMap::new();
        
        for c in parts.iter() {
            let c_count = part_count.entry(*c).or_insert(0); 
            *c_count+=1;
        }
        
        if part_count.get(&set.0.character).unwrap_or(&0) >= &set.0.min && part_count.get(&set.0.character).unwrap_or(&u64::MAX) <= &set.0.max {
            valid_passwords+=1;
        }

    }
    
    valid_passwords
}

fn part_2(input: Vec<(Rules, Password)>) -> u64 {
    let mut valid_passwords = 0_u64; 
    
    for set in input.iter() {
        let parts: Vec<char> = set.1.chars().collect();
        
        if (parts[(set.0.min-1_u64) as usize] == set.0.character) ^ (parts[(set.0.max-1_u64) as usize] == set.0.character) {
            valid_passwords+=1;
        }

    }
    
    valid_passwords
}

fn main() {
    let matches = App::new("AoC 2020: day_2")
        .version("1.0")
        .author("degausser <Ricky@Hosfelt.io>")
        .about("Solution to AoC day_2")
        .subcommand(SubCommand::with_name("ex1")
            .about("day_2 part_1 example")
            .arg(Arg::with_name("input")
                .short("i")
                .long("input")
                .help("puzzle input on the cmdline")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("part_1")
            .about("day_2 part_1")
            .arg(Arg::with_name("input")
                .short("i")
                .long("input")
                .help("puzzle input as a text file")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("ex2")
            .about("day_2 part_2 example")
            .arg(Arg::with_name("input")
                .short("i")
                .long("input")
                .help("puzzle input on the cmdline")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("part_2")
            .about("day_2 part_2")
            .arg(Arg::with_name("input")
                .short("i")
                .long("input")
                .help("puzzle input as a text file")
                .takes_value(true)))
        .get_matches();

    //day_2 part_1 example
    if let Some(ref matches) = matches.subcommand_matches("ex1") {
        if matches.is_present("input") {
            println!("Passwords that match the character ranges: {}", part_1(parse_example_input(matches.value_of("input").unwrap().to_string())));
        }
    }
    
    // day_2 part_1   
    if let Some(ref matches) = matches.subcommand_matches("part_1") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(matches.value_of("input").unwrap().to_string());
            let parsed_input = parse_input(total_inputs.unwrap());

            println!("Passwords that match the character ranges: {}", part_1(parsed_input));
        }
    }

    // day_2 part_2 example
    if let Some(ref matches) = matches.subcommand_matches("ex2") {
        if matches.is_present("input") {
            println!("Passwords that match the character indexes: {}", part_2(parse_example_input(matches.value_of("input").unwrap().to_string())));
        }
    }
    
    // day_2 part_2
    if let Some(ref matches) = matches.subcommand_matches("part_2") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(matches.value_of("input").unwrap().to_string());
            let parsed_input = parse_input(total_inputs.unwrap());

            println!("Passwords that match the character indexes: {}", part_2(parsed_input));
        }
    }
}   
