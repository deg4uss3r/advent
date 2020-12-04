use clap::{App, Arg, SubCommand};

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

type Row = u64;
type Tree = u64;
type Forest = HashMap<Row, Vec<Tree>>;
type Finish = u64;

fn read_input_file(input_path: String) -> Result<String, std::io::Error> {
    let mut file = File::open(input_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_example_input(i: &str) -> (Forest, Finish, u64) {
    let tracks: Vec<&str> = i.split(",").collect();
    let finish: Finish = tracks.len() as u64;
    let modul = (tracks[0].len()) as u64;

    let mut forest: Forest = HashMap::new();

    let mut row = 0;
    for track in tracks.iter() {
        let path: Vec<char> = track.chars().collect();
        for (col, space) in path.iter().enumerate() {
            if *space == '#' {
                let trees = forest.entry(row).or_insert(Vec::new());
                trees.push(col as u64);
            }
        }

        row += 1;
    }

    (forest, finish, modul)
}

fn parse_input(i: &str) -> (Forest, Finish, u64) {
    let tracks: Vec<&str> = i.split("\n").collect();
    let finish: Finish = tracks.len() as u64;
    let modul = (tracks[0].len()) as u64;

    let mut forest: Forest = HashMap::new();

    let mut row = 0;
    for track in tracks.iter() {
        let path: Vec<char> = track.chars().collect();
        for (col, space) in path.iter().enumerate() {
            if *space == '#' {
                let trees = forest.entry(row).or_insert(Vec::new());
                trees.push(col as u64);
            }
        }

        row += 1;
    }

    (forest, finish, modul)
}
fn sled(hill: (Forest, Finish, u64), rise: u64, run: u64) -> u64 {
    let mut hit_trees = 0;
    let mut col: u64 = 0;
    let mut row: u64 = 0;
    let modul = hill.2;

    while row <= hill.1 {
        if hill
            .0
            .get(&row)
            .cloned()
            .unwrap_or_else(|| Vec::new())
            .contains(&(col % modul))
        {
            hit_trees += 1;
        }

        col += rise;
        row += run;
    }

    hit_trees
}

fn part_1(hill: (Forest, Finish, u64)) -> u64 {
    sled(hill, 3, 1)
}

fn part_2(hill: (Forest, Finish, u64)) -> u64 {
    sled(hill.clone(), 1, 1)
        * sled(hill.clone(), 3, 1)
        * sled(hill.clone(), 5, 1)
        * sled(hill.clone(), 7, 1)
        * sled(hill.clone(), 1, 2)
}

fn main() {
    let matches = App::new("AoC 2020: day_3")
        .version("1.0")
        .author("degausser <Ricky@Hosfelt.io>")
        .about("Solution to AoC day_3")
        .subcommand(
            SubCommand::with_name("ex1")
                .about("day_3 part_1 example")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .help("puzzle input on the cmdline")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("part_1").about("day_3 part_1").arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .help("puzzle input as a text file")
                    .takes_value(true),
            ),
        )
        .subcommand(
            SubCommand::with_name("ex2")
                .about("day_3 part_2 example")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .help("puzzle input on the cmdline")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("part_2").about("day_3 part_2").arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .help("puzzle input as a text file")
                    .takes_value(true),
            ),
        )
        .get_matches();

    //day_3 part_1 example
    if let Some(ref matches) = matches.subcommand_matches("ex1") {
        if matches.is_present("input") {
            println!(
                "Hit trees: {}",
                part_1(parse_example_input(matches.value_of("input").unwrap()))
            );
        }
    }

    // day_3 part_1
    if let Some(ref matches) = matches.subcommand_matches("part_1") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(matches.value_of("input").unwrap().to_string());
            let parsed_input = parse_input(&total_inputs.unwrap());

            println!("Hit trees: {}", part_1(parsed_input));
        }
    }

    // day_3 part_2 example
    if let Some(ref matches) = matches.subcommand_matches("ex2") {
        if matches.is_present("input") {
            println!(
                "Product of trees to hit: {}",
                part_2(parse_example_input(matches.value_of("input").unwrap()))
            );
        }
    }

    // day_3 part_2
    if let Some(ref matches) = matches.subcommand_matches("part_2") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(matches.value_of("input").unwrap().to_string());
            let parsed_input = parse_input(&total_inputs.unwrap());

            println!("Product of trees to hit: {}", part_2(parsed_input));
        }
    }
}
