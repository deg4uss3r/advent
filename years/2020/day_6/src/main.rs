use std::{collections::HashMap, fs::File, io::prelude::*};

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

type GroupMembers = u64;
type Answers = Vec<char>;
type Groups = Vec<(Answers, GroupMembers)>;

fn parse_input(input: &str) -> Groups {
    let responses: Vec<&str> = input.split("\n").collect();
    let mut groups: Groups = Vec::new();
    let mut answers: Answers = Vec::new();
    let mut group_members: GroupMembers = 0;

    for response in responses.iter() {
        //if the line is blank it separates groups of people
        if response == &"" {
            groups.push((answers, group_members));
            answers = Vec::new();
            group_members = 0;
            continue;
        }

        for c in response.chars() {
            answers.push(c);
        }

        group_members += 1;
    }

    groups.push((answers, group_members));
    groups
}

fn part_1(mut groups: Groups) -> u64 {
    let mut yes_answers = 0;
    for (group, _group_members) in groups.iter_mut() {
        group.sort();
        group.dedup();
        yes_answers += group.len() as u64
    }

    yes_answers
}

fn part_2(groups: Groups) -> u64 {
    let mut yes_answers = 0_u64;

    for (group, group_members) in groups.iter() {
        let mut answer_set: HashMap<char, u64> = HashMap::new();

        for ans in group.iter() {
            let c = answer_set.entry(*ans).or_insert(0);
            *c += 1;
        }

        for (_questions, question_count) in answer_set.iter() {
            if question_count == group_members {
                yes_answers += 1;
            }
        }
    }

    yes_answers
}

fn main() -> Result<(), anyhow::Error> {
    let matches = App::new("AoC 2020: day_6")
        .version("1.0")
        .author("Ricky <Ricky@Hosfelt.io>")
        .about("Solution to AoC day_6")
        .subcommand(
            SubCommand::with_name("ex1")
                .about("day_6 part_1 example")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .help("puzzle input on the cmdline")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("part_1").about("day_6 part_1").arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .help("puzzle input as a text file")
                    .takes_value(true),
            ),
        )
        .subcommand(
            SubCommand::with_name("ex2")
                .about("day_6 part_2 example")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .help("puzzle input on the cmdline")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("part_2").about("day_6 part_2").arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .help("puzzle input as a text file")
                    .takes_value(true),
            ),
        )
        .get_matches();

    //day_6 part_1 example
    if let Some(ref matches) = matches.subcommand_matches("ex1") {
        if matches.is_present("input") {
            println!(
                "Total yes answers: {}",
                part_1(parse_input(
                    matches
                        .value_of("input")
                        .context("Error no value supplied for --input")?
                ))
            );
        }
    }

    // day_6 part_1
    if let Some(ref matches) = matches.subcommand_matches("part_1") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(
                matches
                    .value_of("input")
                    .context("Error no value supplied for --input")?,
            )?;
            let parsed_input = parse_input(&total_inputs);

            println!("Total yes answers: {}", part_1(parsed_input));
        }
    }

    // day_6 part_2 example
    if let Some(ref matches) = matches.subcommand_matches("ex2") {
        if matches.is_present("input") {
            println!(
                "Questions which all member answered yes: {}",
                part_2(parse_input(
                    matches
                        .value_of("input")
                        .context("Error no value supplied for --input")?
                ))
            );
        }
    }

    // day_6 part_2
    if let Some(ref matches) = matches.subcommand_matches("part_2") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(
                matches
                    .value_of("input")
                    .context("Error no value supplied for --input")?,
            )?;
            let parsed_input = parse_input(&total_inputs);

            println!(
                "Questions which all member answered yes: {}",
                part_2(parsed_input)
            );
        }
    }

    Ok(())
}
