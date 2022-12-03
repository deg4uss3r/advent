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

#[derive(Debug)]
enum Error {
    ParseError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ParseError(s) => write!(
                f,
                "Could not parse ({}) into Rock, Paper, or Scissors please check input",
                s
            ),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
enum Opponent {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Throw {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum GameResult {
    Win,
    Lose,
    Draw,
}

trait Score {
    fn score(&self) -> u32;
}

impl Score for Throw {
    fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl Score for GameResult {
    fn score(&self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Lose => 0,
            Self::Draw => 3,
        }
    }
}

impl TryFrom<&str> for Opponent {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Error> {
        match value {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            _ => Err(Error::ParseError(value.to_string())),
        }
    }
}

impl TryFrom<&str> for Throw {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Error> {
        match value {
            "X" => Ok(Self::Rock),
            "Y" => Ok(Self::Paper),
            "Z" => Ok(Self::Scissors),
            _ => Err(Error::ParseError(value.to_string())),
        }
    }
}

impl From<&Throw> for GameResult {
    fn from(value: &Throw) -> GameResult {
        match value {
            Throw::Rock => GameResult::Lose,
            Throw::Paper => GameResult::Draw,
            Throw::Scissors => GameResult::Win,
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<(Opponent, Throw)>, anyhow::Error> {
    let mut round = vec![];

    for line in input.split("\n") {
        let values = line.split(" ").collect::<Vec<&str>>();
        round.push((Opponent::try_from(values[0])?, Throw::try_from(values[1])?));
    }

    Ok(round)
}

fn match_result(o: &Opponent, y: &Throw) -> GameResult {
    match (o, y) {
        (Opponent::Rock, Throw::Rock) => GameResult::Draw,
        (Opponent::Rock, Throw::Paper) => GameResult::Win,
        (Opponent::Rock, Throw::Scissors) => GameResult::Lose,
        (Opponent::Paper, Throw::Rock) => GameResult::Lose,
        (Opponent::Paper, Throw::Paper) => GameResult::Draw,
        (Opponent::Paper, Throw::Scissors) => GameResult::Win,
        (Opponent::Scissors, Throw::Rock) => GameResult::Win,
        (Opponent::Scissors, Throw::Paper) => GameResult::Lose,
        (Opponent::Scissors, Throw::Scissors) => GameResult::Draw,
    }
}

fn calc_score(opponent: &Opponent, you: &Throw) -> u32 {
    let result = match_result(opponent, you);
    result.score() + you.score()
}

fn predict_throw(opponent: &Opponent, end_result: &GameResult) -> Throw {
    match (opponent, end_result) {
        (Opponent::Rock, GameResult::Win) => Throw::Paper,
        (Opponent::Rock, GameResult::Draw) => Throw::Rock,
        (Opponent::Rock, GameResult::Lose) => Throw::Scissors,
        (Opponent::Paper, GameResult::Win) => Throw::Scissors,
        (Opponent::Paper, GameResult::Draw) => Throw::Paper,
        (Opponent::Paper, GameResult::Lose) => Throw::Rock,
        (Opponent::Scissors, GameResult::Win) => Throw::Rock,
        (Opponent::Scissors, GameResult::Draw) => Throw::Scissors,
        (Opponent::Scissors, GameResult::Lose) => Throw::Paper,
    }
}

fn part_1(round: Vec<(Opponent, Throw)>) -> Result<u32, anyhow::Error> {
    Ok(round.iter().map(|(o, y)| calc_score(o, y)).sum())
}

fn part_2(round: Vec<(Opponent, Throw)>) -> Result<u32, anyhow::Error> {
    Ok(round
        .iter()
        .map(|(o, y)| {
            let t = predict_throw(o, &y.into());
            calc_score(o, &t)
        })
        .sum())
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
    //day_1 part_2
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
