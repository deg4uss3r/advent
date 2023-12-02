use anyhow::Context;
use clap::{Arg, Command};
use core::cmp::Ordering;
use std::{fmt, fs::File, io::prelude::*};

fn read_input_file(input_path: &str) -> Result<String, anyhow::Error> {
    let mut file =
        File::open(input_path).context(format!("Error opening input file '{}'", input_path))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .context(format!("Error reading file '{}'", input_path))?;
    Ok(contents)
}

#[derive(Debug, Default)]
struct Game {
    Id: u32,
    Rounds: Vec<Round>,
}

#[derive(Debug, Default, Eq)]
struct Round {
    Red: Option<u32>,
    Green: Option<u32>,
    Blue: Option<u32>,
}

impl PartialEq<Round> for Round {
    fn eq(&self, other: &Round) -> bool {
        self.Red == other.Red && self.Blue == other.Blue && self.Green == other.Green
    }
}

impl PartialOrd for Round {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Round {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut equal = 0;

        if self.Red.cmp(&other.Red) == Ordering::Greater {
            Ordering::Greater
        } else {
            if self.Red.cmp(&other.Red) == Ordering::Equal {
                equal += 1;
            }
            if self.Blue.cmp(&other.Blue) == Ordering::Greater {
                Ordering::Greater
            } else {
                if self.Blue.cmp(&other.Blue) == Ordering::Equal {
                    equal += 1;
                }

                if self.Green.cmp(&other.Green) == Ordering::Greater {
                    Ordering::Greater
                } else {
                    if self.Green.cmp(&other.Green) == Ordering::Equal {
                        equal += 1;
                    }
                    if equal == 3 {
                        Ordering::Equal
                    } else {
                        Ordering::Less
                    }
                }
            }
        }
    }
}

impl Round {
    fn product(&self) -> u32 {
        let mut product = 1;
        if let Some(x) = self.Red {
            product *= x;
        }

        if let Some(x) = self.Blue {
            product *= x;
        }

        if let Some(x) = self.Green {
            product *= x;
        }

        product
    }
}

fn parse_input(input: &str) -> Result<Vec<Game>, anyhow::Error> {
    //parse the input from lines of strings into several games containing multiple round with colors
    Ok(input
        .split("\n")
        .map(|line| {
            let mut game: Game = Default::default();

            let game_id_rest: Vec<&str> = line.split(":").collect();
            let mut game_id_str = game_id_rest[0].to_string();
            game_id_str.retain(|c| c.is_digit(10));
            let game_id = game_id_str.parse::<u32>().unwrap();
            game.Id = game_id;
            let rounds_before_parse: Vec<&str> = game_id_rest[1].split(";").collect();
            let mut rounds = Vec::new();

            for round_unparsed in rounds_before_parse {
                let mut round: Round = Default::default();
                let colors: Vec<&str> = round_unparsed.trim().split(",").collect();
                for color in colors {
                    let amount_color: Vec<&str> = color.trim().split(" ").collect();
                    if amount_color[1] == "blue" {
                        round.Blue = Some(amount_color[0].parse::<u32>().unwrap());
                    }

                    if amount_color[1] == "red" {
                        round.Red = Some(amount_color[0].parse::<u32>().unwrap());
                    }

                    if amount_color[1] == "green" {
                        round.Green = Some(amount_color[0].parse::<u32>().unwrap());
                    }
                }
                rounds.push(round);
            }
            game.Rounds = rounds;
            game
        })
        .collect())
}

fn part_1(games: Vec<Game>) -> Result<u32, anyhow::Error> {
    // The Elf would first like to know which games would
    //have been possible if the bag contained only
    //12 red cubes,
    //13 green cubes,
    //14 blue cubes?
    let max_amounts = Round {
        Red: Some(12),
        Blue: Some(14),
        Green: Some(13),
    };

    let mut game_ids: Vec<u32> = Vec::new();

    for game in games {
        let game_id = game.Id;

        let pass_amount = game.Rounds.len();
        let mut index = 0;

        for round in &game.Rounds {
            if round <= &max_amounts {
                index += 1;
            }
        }

        if index == pass_amount {
            println!("{game:?}");
            game_ids.push(game_id);
        }
    }

    Ok(game_ids.into_iter().sum())
}

fn part_2(games: Vec<Game>) -> Result<u32, anyhow::Error> {
    // in each round find the max of each color
    // multiple each together and add that product to an array
    let mut products: Vec<u32> = Vec::new();

    for game in games {
        let mut max_round = Round::default();
        for round in game.Rounds {
            if round.Red > max_round.Red {
                max_round.Red = round.Red;
            }

            if round.Green > max_round.Green {
                max_round.Green = round.Green;
            }

            if round.Blue > max_round.Blue {
                max_round.Blue = round.Blue;
            }
        }
        products.push(max_round.product());
    }
    // finally sum all of the cubes together
    Ok(products.into_iter().sum())
}

fn main() -> Result<(), anyhow::Error> {
    let matches = Command::new("AoC 2022: day_2")
        .version("1.0")
        .author("Ricky Hosfelt <ricky@hosfe.lt>")
        .about("Solution to AoC day_2")
        .subcommand_required(true)
        .subcommand(
            Command::new("part1").about("day_2 part 1").arg(
                Arg::new("input")
                    .short('i')
                    .long("input")
                    .help("/path/to/puzzle.input")
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("part2").about("day_2 part 2").arg(
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

            println!("day_2 part 1: {:?}", part_1(parse_input(&total_inputs)?)?);
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

            println!("day_2 part 2: {:?}", part_2(parse_input(&total_inputs)?)?);
        }
    }

    Ok(())
}
