use std::{fs::File, io::prelude::*};
use std::convert::TryFrom;

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

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
enum Instruction {
    acc(i64),
    jmp(i64),
    nop(i64),
}

#[derive(Debug)]
enum ParseError {
    InstructionNotRecognized,
    MalformedLine,
    MalformedNumber,
}

impl std::error::Error for ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let e = match self {
            Self::InstructionNotRecognized => "Instruction not recognized",
            Self::MalformedLine => "Malformed instruction line",
            Self::MalformedNumber => "Malformed number",
        };

        write!(f, "{}", e)
    }
}

impl TryFrom<&str> for Instruction {
    type Error = ParseError;
    
    fn try_from(input: &str) -> Result<Instruction, ParseError> {
        let ins_value: Vec<&str> = input.split(" ").collect(); 
        
        if ins_value.len() != 2 {
            return Err(ParseError::MalformedLine);
        }
        
        match ins_value[0] {
            "acc" => Ok(Instruction::acc(ins_value[1].parse::<i64>().map_err(|_| ParseError::MalformedNumber)?)),
            "jmp" => Ok(Instruction::jmp(ins_value[1].parse::<i64>().map_err(|_| ParseError::MalformedNumber)?)),
            "nop" => Ok(Instruction::nop(ins_value[1].parse::<i64>().map_err(|_| ParseError::MalformedNumber)?)),
            _ => Err(ParseError::InstructionNotRecognized),
        }
    }
}

fn parse_example_input(input: &str) -> Result<Vec<Instruction>, ParseError> {
    let mut instructions = Vec::new();
    let inputs: Vec<&str> = input.split("\n").collect();
    
    for i in inputs.iter() {
        if i != &"" {
            instructions.push(Instruction::try_from(*i)?);
        }
    }
    
    Ok(instructions)
}

fn parse_input(input: &str) -> Result<Vec<Instruction>, ParseError> {
    parse_example_input(input)
}

fn part_1(instructions: Vec<Instruction>) -> i64 {
    let mut visited_instruction = Vec::new();
    let mut accumulator = 0;
    let mut index = 0; 
    let mut next_index = 0;
    
    while !visited_instruction.contains(&next_index) {    
        visited_instruction.push(index);
        
       let ins = &instructions[index as usize];

        match ins {
            Instruction::acc(value) => {
                accumulator += value;
                next_index = index + 1;
            },
            Instruction::jmp(value) => {
                next_index = index + value;
            }
            Instruction::nop(_value) => {
                next_index = index + 1;
            }
        }
        
        index = next_index;
    }
    
    accumulator
}

fn part_2(instructions: Vec<Instruction>) -> i64 {
    let mut accumulator = 0;

    for instruction_index in 0..instructions.len()-1 {
        let mut instructions_copy = instructions.to_vec();
        let mut visited_instruction = Vec::new();
        let mut index = 0; 
        accumulator = 0;
        let mut next_index = 0;

        // Replace this instruction if it is a jmp or nop
        instructions_copy[instruction_index] = match instructions_copy[instruction_index] {
            Instruction::jmp(value) => Instruction::nop(value), 
            Instruction::nop(value) => Instruction::jmp(value),
            _ => instructions[instruction_index].clone(),
        };
        
        while !visited_instruction.contains(&next_index) {
            visited_instruction.push(index);
            
        let ins = &instructions_copy[index as usize];

            match ins {
                Instruction::acc(value) => {
                    accumulator += value;
                    next_index = index + 1;
                },
                Instruction::jmp(value) => {
                    next_index = index + value;
                }
                Instruction::nop(_value) => {
                    next_index = index + 1;
                }
            }
            
            if next_index as usize == instructions.len() {
                return accumulator;
            }

            index = next_index;
        }
    }
    
    accumulator
}

fn main() -> Result<(), anyhow::Error> {
    let matches = App::new("AoC 2020: day_8")
        .version("1.0")
        .author("Ricky <Ricky@Hosfelt.io>")
        .about("Solution to AoC day_8")
        .subcommand(
            SubCommand::with_name("ex1")
                .about("day_8 part_1 example")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .help("puzzle input on the cmdline")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("part_1").about("day_8 part_1").arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .help("puzzle input as a text file")
                    .takes_value(true),
            ),
        )
        .subcommand(
            SubCommand::with_name("ex2")
                .about("day_8 part_2 example")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .help("puzzle input on the cmdline")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("part_2").about("day_8 part_2").arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .help("puzzle input as a text file")
                    .takes_value(true),
            ),
        )
        .get_matches();

    //day_8 part_1 example
    if let Some(ref matches) = matches.subcommand_matches("ex1") {
        if matches.is_present("input") {
            println!(
                "Accumulator value at loop: {}",
                part_1(parse_example_input(
                    matches
                        .value_of("input")
                        .context("Error no value supplied for --input")?
                )?)
            );
        }
    }

    // day_8 part_1
    if let Some(ref matches) = matches.subcommand_matches("part_1") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(
                matches
                    .value_of("input")
                    .context("Error no value supplied for --input")?,
            )?;
            let parsed_input = parse_input(&total_inputs)?;
            println!("Accumulator value at loop: {}", part_1(parsed_input));
        }
    }

    // day_8 part_2 example
    if let Some(ref matches) = matches.subcommand_matches("ex2") {
        if matches.is_present("input") {
            println!(
                "Accumulator value with successful run: {}",
                part_2(parse_input(
                    matches
                        .value_of("input")
                        .context("Error no value supplied for --input")?
                )?)
            );
        }
    }

    // day_8 part_2
    if let Some(ref matches) = matches.subcommand_matches("part_2") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(
                matches
                    .value_of("input")
                    .context("Error no value supplied for --input")?,
            )?;
            let parsed_input = parse_input(&total_inputs)?;

            println!("Accumulator value with successful run: {}", part_2(parsed_input));
        }
    }

    Ok(())
}
