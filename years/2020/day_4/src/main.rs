use clap::{App, Arg, SubCommand};
use regex::Regex;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn read_input_file(input_path: String) -> Result<String, std::io::Error> {
    let mut file = File::open(input_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_input(input: String) -> Vec<Passport> {
    let mut passports_parsed: Vec<Passport> = Vec::new();
    
    //passports are separated by blank lines
    let data_batch: Vec<&str> = input.split("\n").collect(); 

    let mut passport: Passport = Passport::new();
    
    for data in data_batch.iter() {
        if *data == "" {
            passports_parsed.push(passport);
            passport = Passport::new();
        } else {
            let fields: Vec<&str> = data.split(" ").collect(); 
            for field in fields.iter() {
                let field_value: Vec<&str> = field.split(":").collect(); 
                
                if field_value.len() != 2 {
                    panic!("Invalid field:value pair");
                } else {
                    passport.details.insert(PassportField::from(field_value[0]), field_value[1].to_string());
                }
            }
        }
    }
    passports_parsed.push(passport);
    
    passports_parsed
}

#[allow(non_camel_case_types)]
#[derive(Debug, Hash, Eq, PartialEq)]
enum PassportField {
    byr, // (Birth Year)
    iyr, // (Issue Year)
    eyr, // (Expiration Year)
    hgt, // (Height)
    hcl, // (Hair Color)
    ecl, // (Eye Color)
    pid, // (Passport ID)
    cid, // (Country ID)
    Unknown, // Unknown passport field 
}

impl From<&str> for PassportField {
    fn from(input: &str) -> Self {
        match input {
            "byr" => Self::byr, // (Birth Year)
            "iyr" => Self::iyr, // (Issue Year)
            "eyr" => Self::eyr, // (Expiration Year)
            "hgt" => Self::hgt, // (Height)
            "hcl" => Self::hcl, // (Hair Color)
            "ecl" => Self::ecl, // (Eye Color)
            "pid" => Self::pid, // (Passport ID)
            "cid" => Self::cid, // (Country ID)
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug)]
struct Passport {
    details: HashMap<PassportField, String>
}

impl Passport {
    fn new() -> Self {
        Passport{details: HashMap::new()}
    }
    
    fn is_valid(&self) -> bool {
        let required_fields: Vec<PassportField> = vec![PassportField::byr, PassportField::iyr, PassportField::eyr, PassportField::hgt, 
        PassportField::hcl, PassportField::ecl, PassportField::pid];
        
        for field in required_fields {
            if self.details.contains_key(&field) {
                continue;
            } else {
                return false;
            }
        }
        if self.details.contains_key(&PassportField::Unknown) {
            return false;
        } else {
            true
        }
    }

    fn part_2_is_valid(&self)-> bool {
        if self.is_valid() {
            for (field, value) in self.details.iter() {
                let out = match field {
                    PassportField::byr => {
                        let number = value.parse::<u16>(); 
                        match number {
                            Ok(v) => if v >= 1920 && v <= 2002 {
                                true
                            } else {
                                false
                            }, 
                            Err(_) => false
                        }
                    },
                    PassportField::iyr => {
                        let number = value.parse::<u16>();
                        match number {
                            Ok(v) => if v >= 2010 && v <= 2020 {
                                true
                            } else {
                                false
                            },
                            Err(_) => false,
                        }
                    }, 
                    PassportField::eyr => {
                        let number = value.parse::<u16>();
                        match number {
                            Ok(v) => if v >= 2020 && v <= 2030 {
                                true
                            } else {
                                false
                            },
                            Err(_) => false,
                        }
                    }, 
                    PassportField::hgt => {
                        if value.ends_with("in") {
                            let num: Vec<&str> = value.split("in").collect();
                            if num.len() == 2 {
                                let n = num[0].parse::<u8>();
                                match n {
                                    Ok(n) => {
                                        if n >= 59 && n <= 79 {
                                            true
                                        } else {
                                            false
                                        }
                                    },
                                    Err(_) => false,
                                }
                            } else {
                                false
                            }
                        } else if value.ends_with("cm") {
                            let num: Vec<&str> = value.split("cm").collect();
                            if num.len() == 2 {
                                let n = num[0].parse::<u8>();
                                match n {
                                    Ok(n) => {
                                        if n >= 150 && n <= 193 {
                                            true
                                        } else {
                                            false
                                        }
                                    },
                                    Err(_) => false,
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    }, 
                    PassportField::hcl => {
                        let re = Regex::new(r"\#[0-9a-f]{6}").unwrap();
                        re.is_match(&value)
                    }, 
                    PassportField::ecl => {
                        let valid_eyes = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                        valid_eyes.contains(&value.as_str())
                    }, 
                    PassportField::pid => {
                        let re = Regex::new(r"^[0-9]{9}$").unwrap();
                        re.is_match(&value)
                    }, 
                    PassportField::cid => true, 
                    PassportField::Unknown => false,
                }; 
                if out {
                    continue;
                } else {
                    return false;
                }
            }
        } else {
            return false;
        }

        true 
    }
}



fn part_1_valid_passports(passports: Vec<Passport>) -> u64 {
    let mut valid = 0_u64;
    
    for passport in passports.iter() {
        if passport.is_valid() {
            valid +=1;
        } 
    }
    valid
}

fn part_2_valid_passports(passports: Vec<Passport>) -> u64 {
    let mut valid = 0_u64;
    
    for passport in passports.iter() {
        if passport.part_2_is_valid() {
            valid +=1;
        } 
    }
    valid
}

fn main() {
    let matches = App::new("AoC 2020: day_4")
        .version("1.0")
        .author("degausser <Ricky@Hosfelt.io>")
        .about("Solution to AoC day_4")
        .subcommand(
            SubCommand::with_name("ex1")
                .about("day_4 part_1 example")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .help("puzzle input on the cmdline")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("part_1").about("day_4 part_1").arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .help("puzzle input as a text file")
                    .takes_value(true),
            ),
        )
        .subcommand(
            SubCommand::with_name("ex2")
                .about("day_4 part_2 example")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .help("puzzle input on the cmdline")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("part_2").about("day_4 part_2").arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .help("puzzle input as a text file")
                    .takes_value(true),
            ),
        )
        .get_matches();

    //day_4 part_1 example
    if let Some(ref matches) = matches.subcommand_matches("ex1") {
        if matches.is_present("input") {
            println!(
                "Valid passports: {}",
                part_1_valid_passports(parse_input(matches.value_of("input").unwrap().to_string()))
            );
        }
    }

    // day_4 part_1
    if let Some(ref matches) = matches.subcommand_matches("part_1") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(matches.value_of("input").unwrap().to_string());
            let parsed_input = parse_input(total_inputs.unwrap());

            println!("Valid passports: {}", part_1_valid_passports(parsed_input));
        }
    }

    // day_4 part_2 example
    if let Some(ref matches) = matches.subcommand_matches("ex2") {
        if matches.is_present("input") {
            println!(
                "Valid passports and fields: {}",
                part_2_valid_passports(parse_input(matches.value_of("input").unwrap().to_string()))
            );
        }
    }

    // day_4 part_2
    if let Some(ref matches) = matches.subcommand_matches("part_2") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(matches.value_of("input").unwrap().to_string());
            let parsed_input = parse_input(total_inputs.unwrap());

            println!("Valid passports and fields: {}", part_2_valid_passports(parsed_input));
        }
    }
}
