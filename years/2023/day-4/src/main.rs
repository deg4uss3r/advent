use std::{fs::File, io::prelude::*};

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

#[derive(Debug, Clone)]
struct Card {
    id: u32,
    your_numbers: Vec<u32>,
    winning_numbers: Vec<u32>,
}

fn parse_input(input: &str) -> Result<Vec<Card>, anyhow::Error> {
    let string_lines: Vec<&str> = input.split("\n").collect();

    let mut cards: Vec<Card> = Vec::new();

    for card in string_lines {
        let game_number_split: Vec<&str> = card.split(": ").collect();
        let id_string: String = game_number_split[0].replace("Card", "");
        let active_winning: Vec<&str> = game_number_split[1].split(" | ").collect();

        cards.push(Card {
            id: id_string.trim().parse::<u32>().unwrap(),
            your_numbers: active_winning[0]
                .trim()
                .split(" ")
                .filter(|c| *c != "")
                .map(|x| x.trim().parse::<u32>().unwrap())
                .collect(),
            winning_numbers: active_winning[1]
                .trim()
                .split(" ")
                .filter(|c| *c != "")
                .map(|x| x.trim().parse::<u32>().unwrap())
                .collect(),
        })
    }

    Ok(cards)
}

fn part_1(cards: Vec<Card>) -> Result<u32, anyhow::Error> {
    let mut total: Vec<u32> = Vec::new();
    for card in cards {
        let mut power = 0;
        for number in card.your_numbers {
            if card.winning_numbers.contains(&number) {
                power += 1;
            }
        }

        if power == 0 {
            total.push(0);
        } else if power == 1 {
            total.push(1);
        } else {
            total.push(2_u32.pow(power - 1));
        }
    }

    Ok(total.iter().sum())
}

fn part_2(cards: Vec<Card>) -> Result<u32, anyhow::Error> {
    use std::collections::HashMap;

    // score the card and a multiplier to the next card score
    // cardid, number of cards
    let mut card_map: HashMap<u32, u32> = HashMap::new();

    for card in cards.iter() {
        let card_bank = card_map.entry(card.id).or_insert(1);
        let mut multipler: u32 = 0;
        let mut wins: u32 = 0;

        for number in card.your_numbers.iter() {
            if card.winning_numbers.contains(&number) {
                multipler += 1_u32 * *card_bank;
                wins += 1_u32;
            }
        }

        // the id is 1 based but the index is 0 so
        // a card_to_add of 1 is equal to card 2.
        let mut card_to_add = card.id;

        while multipler > 0 {
            if card_to_add >= card.id + wins {
                card_to_add = card.id;
            }
            //if we find the card doesn't exist here it _should_ be at a second
            //instance so seed the map with 2 instead of 1
            card_map
                .entry(cards[usize::try_from(card_to_add).unwrap()].id)
                .and_modify(|counter| *counter += 1)
                .or_insert(2);
            card_to_add += 1;
            multipler -= 1;
        }
    }

    Ok(card_map.values().sum())
}

fn main() -> Result<(), anyhow::Error> {
    let matches = Command::new("AoC 2022: day_4")
        .version("1.0")
        .author("Ricky Hosfelt <ricky@hosfe.lt>")
        .about("Solution to AoC day_4")
        .subcommand_required(true)
        .subcommand(
            Command::new("part1").about("day_4 part 1").arg(
                Arg::new("input")
                    .short('i')
                    .long("input")
                    .help("/path/to/puzzle.input")
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("part2").about("day_4 part 2").arg(
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

            println!("day_4 part 1: {:?}", part_1(parse_input(&total_inputs)?)?);
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

            println!("day_4 part 2: {:?}", part_2(parse_input(&total_inputs)?)?);
        }
    }

    Ok(())
}
