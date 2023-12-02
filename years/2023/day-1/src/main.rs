use std::{fmt, fs::File, io::prelude::*};

use anyhow::Context;
use clap::{Arg, Command};

fn read_input_file(input_path: &str) -> Result<String, anyhow::Error> {
    let mut file =
        File::open(input_path).context(format!("Error opening input file '{}'", input_path))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .context(format!("Error reading file '{}'", input_path))?;
    Ok(contents)
}

fn parse_input(input: &str) -> Result<Vec<&str>, anyhow::Error> {
    Ok(input.trim().split("\n").collect())
}

fn part_1(input: Vec<&str>) -> Result<u32, anyhow::Error> {
    //combine the first and last number of each line into a 2 digit number
    let numbers: Vec<u32> = input
        .iter()
        .map(|line| {
            let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
            (digits.first().unwrap().to_string() + &digits.last().unwrap().to_string())
                .parse::<u32>()
                .unwrap()
        })
        .collect();

    //finally take the sum of the numbers
    Ok(numbers.iter().sum::<u32>())
}

fn part_2(input: Vec<&str>) -> Result<u32, anyhow::Error> {
    // use strum to iterate over all possible combinations of the Number enum
    use std::collections::HashMap;
    use strum::IntoEnumIterator; // 0.25
    use strum_macros::EnumIter; // 0.25

    // create an enum to search the line for spelled out numbers
    #[derive(Debug, Clone, Copy, EnumIter)]
    enum Number {
        One,
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
    }

    impl fmt::Display for Number {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::One => write!(f, "one"),
                Self::Two => write!(f, "two"),
                Self::Three => write!(f, "three"),
                Self::Four => write!(f, "four"),
                Self::Five => write!(f, "five"),
                Self::Six => write!(f, "six"),
                Self::Seven => write!(f, "seven"),
                Self::Eight => write!(f, "eight"),
                Self::Nine => write!(f, "nine"),
            }
        }
    }

    impl Number {
        fn as_digit(&self) -> &str {
            match self {
                Self::One => "1",
                Self::Two => "2",
                Self::Three => "3",
                Self::Four => "4",
                Self::Five => "5",
                Self::Six => "6",
                Self::Seven => "7",
                Self::Eight => "8",
                Self::Nine => "9",
            }
        }
    }

    // search for either a spelled out number or a number as a character
    let numbers: Vec<u32> = input
        .iter()
        .map(|line| {
            let mut index_number: HashMap<usize, String> = HashMap::new();
            for number in Number::iter() {
                let as_digit = number.as_digit().to_owned();

                let words: Vec<_> = line.match_indices(&number.to_string()).collect();
                for (index, _word) in words {
                    index_number.insert(index, as_digit.clone());
                }
                let nums: Vec<_> = line.match_indices(&as_digit).collect();

                for (index, _num) in nums {
                    index_number.insert(index, as_digit.clone());
                }
            }
            let mut keys: Vec<&usize> = index_number.keys().collect();
            keys.sort_unstable();
            (index_number.get(keys.first().unwrap()).unwrap().to_string()
                + index_number
                    .get(keys.last().unwrap_or(keys.first().unwrap()))
                    .unwrap())
            .parse::<u32>()
            .unwrap()
        })
        .collect();

    //finally take the sum of the numbers
    Ok(numbers.iter().sum::<u32>())
}

fn main() -> Result<(), anyhow::Error> {
    let matches = Command::new("AoC 2023: day_1")
        .version("1.0")
        .author("Ricky Hosfelt <ricky@hosfe.lt>")
        .about("Solution to AoC day_1")
        .subcommand_required(true)
        .subcommand(
            Command::new("part1").about("day_1 part 1").arg(
                Arg::new("input")
                    .short('i')
                    .long("input")
                    .help("/path/to/puzzle.input")
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("part2").about("day_1 part 2").arg(
                Arg::new("input")
                    .short('i')
                    .long("input")
                    .help("/path/to/puzzle.input")
                    .required(true),
            ),
        )
        .get_matches();

    // day_1 part_1
    if let Some(ref commands) = matches.subcommand_matches("part1") {
        if commands.try_contains_id("input")? {
            let total_inputs = read_input_file(
                commands
                    .get_one::<String>("input")
                    .context("Error no value supplied for --input")?,
            )?;

            println!("day_1 part 1: {:?}", part_1(parse_input(&total_inputs)?)?);
        }
    }
    // day_1 part_2
    if let Some(ref commands) = matches.subcommand_matches("part2") {
        if commands.try_contains_id("input")? {
            let total_inputs = read_input_file(
                commands
                    .get_one::<String>("input")
                    .context("Error no value supplied for --input")?,
            )?;

            println!("day_1 part 2: {:?}", part_2(parse_input(&total_inputs)?)?);
        }
    }

    Ok(())
}
