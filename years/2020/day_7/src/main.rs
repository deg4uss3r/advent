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

use std::collections::HashMap;

type Bag = String;
type Rules = HashMap<Bag, HashMap<Bag, u64>>;

fn parse_bag_line(input: &str) -> (String, u64) {
    let input_minus_bags = input.replace("bags", "").replace("bag", "").replace(".", "");
    //The first bit is the count, the rest is the bag name
    let mut split_on_space: Vec<&str> = input_minus_bags.split(" ").collect();
    if split_on_space.len() >= 2 {
        let count = split_on_space.remove(0).parse::<u64>().expect("Error parsing bag count");
        let bag_name: String = split_on_space.iter().map(|s| format!("{} ", s.to_string())).collect::<String>().trim().to_string();
        return (bag_name, count); 
    } else {
        panic!("Error malformed bag line '{}'", input);
    }
}

fn parse_input(input: &str) -> Rules {
    parse_example_input(input)
}

fn parse_example_input(input: &str) -> Rules {
    let raw_rules: Vec<&str> = input.split("\n").collect();
    
    let mut rules: Rules = HashMap::new();
    
    for rule in raw_rules.iter() {
        if rule == &"" {
            continue;
        }

        let bag_interior: Vec<&str> = rule.split(" bags contain ").collect();
        let bag = bag_interior[0];
        let can_contain = bag_interior[1];
        
        //multiple rules
        if can_contain.ends_with("no other bags.") {
            rules.insert(bag.to_string(), HashMap::new());
        } else {
            let multiple_bags: Vec<&str> = can_contain.split(", ").collect();
            let mut interior_rules = HashMap::new();
            for x in multiple_bags.iter() {
                let (interior_bag, interior_count) = parse_bag_line(x);
                interior_rules.insert(interior_bag, interior_count);
            }
            rules.insert(bag.to_string(), interior_rules);
        }
    }
    
    rules
}

fn search_bags(bag: Bag, rules: Rules) -> Vec<String> {
    let mut found: Vec<String> = Vec::new();
    
    for (this_bag, interiors) in rules.iter() {
        if interiors.contains_key(&bag) {
            found.push(this_bag.to_string());
            found.append(&mut search_bags(this_bag.to_string(), rules.clone()));
        }
    }
    
    found
}

fn part_1(mut bags: Vec<String>) -> u64 {
    bags.sort();
    bags.dedup();
    bags.len() as u64
}

fn part_2(bag: Bag, rules: Rules) -> u64 {
    let mut found = 0_u64;
    
    for (this_bag, interiors) in rules.iter() {
        if this_bag == &bag {
            found += interiors.values().sum::<u64>();
            for next_bag in interiors.keys() {
                found += interiors[next_bag]*part_2(next_bag.to_string(), rules.clone());
            }
        }
    }
    
    found
}

fn main() -> Result<(), anyhow::Error> {
    let matches = App::new("AoC 2020: day_7")
        .version("1.0")
        .author("Ricky <Ricky@Hosfelt.io>")
        .about("Solution to AoC day_7")
        .subcommand(
            SubCommand::with_name("ex1")
                .about("day_7 part_1 example")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .help("puzzle input on the cmdline")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("part_1").about("day_7 part_1").arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .help("puzzle input as a text file")
                    .takes_value(true),
            ),
        )
        .subcommand(
            SubCommand::with_name("ex2")
                .about("day_7 part_2 example")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .help("puzzle input on the cmdline")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("part_2").about("day_7 part_2").arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .help("puzzle input as a text file")
                    .takes_value(true),
            ),
        )
        .get_matches();

    //day_7 part_1 example
    if let Some(ref matches) = matches.subcommand_matches("ex1") {
        if matches.is_present("input") {
            println!(
                "Bags that can contain 'shiny gold': {}",
                part_1(search_bags("shiny gold".to_string(), parse_example_input(
                    matches
                        .value_of("input")
                        .context("Error no value supplied for --input")?
                )))
            );
        }
    }

    // day_7 part_1
    if let Some(ref matches) = matches.subcommand_matches("part_1") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(
                matches
                    .value_of("input")
                    .context("Error no value supplied for --input")?,
            )?;
            let parsed_input = parse_input(&total_inputs);

            println!("Bags that can contain 'shiny gold': {}", part_1(search_bags("shiny gold".to_string(), parsed_input)));
        }
    }
    // day_7 part_2 example
    if let Some(ref matches) = matches.subcommand_matches("ex2") {
        if matches.is_present("input") {
            println!(
                "Bags to buy for 'shiny gold': {}",
                part_2("shiny gold".to_string(), parse_input(
                    matches
                        .value_of("input")
                        .context("Error no value supplied for --input")?
                ))
            );
        }
    }

    // day_7 part_2
    if let Some(ref matches) = matches.subcommand_matches("part_2") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(
                matches
                    .value_of("input")
                    .context("Error no value supplied for --input")?,
            )?;
            let parsed_input = parse_input(&total_inputs);

            println!("Bags to buy for 'shiny gold': {}", part_2("shiny gold".to_string(), parsed_input));
        }
    }
    Ok(())
}
