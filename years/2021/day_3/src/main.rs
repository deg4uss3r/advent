use std::{fs::File, io::prelude::*};

use anyhow::Context;
use array2d::Array2D;
use clap::{App, Arg, SubCommand};

fn read_input_file(input_path: &str) -> Result<String, anyhow::Error> {
    let mut file =
        File::open(input_path).context(format!("Error opening input file '{}'", input_path))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .context(format!("Error reading file '{}'", input_path))?;
    Ok(contents)
}

fn parse_input(input: &str) -> Array2D<u8> {
    // each line is a bit "word" (x number of bits)
    let words: Vec<&str> = input.split("\n").collect();

    let mut bits = vec![];

    for word in words.iter() {
        let data: Vec<u8> = word
            .trim()
            .split_inclusive(|c| c == '1' || c == '0')
            .into_iter()
            .map(|s| if s == "1" { 1_u8 } else { 0_u8 })
            .collect();
        bits.push(data)
    }

    Array2D::from_rows(&bits)
}

fn find_most_common(grid: &Array2D<u8>, column: usize) -> u8 {
    let ones = grid.column_iter(column).filter(|n| **n == 1).count();
    let zeros = grid.column_iter(column).filter(|n| **n == 0).count();

    if ones > zeros {
        1
    } else {
        0
    }
}

fn find_least_common(grid: &Array2D<u8>, column: usize) -> u8 {
    let ones = grid.column_iter(column).filter(|n| **n == 1).count();
    let zeros = grid.column_iter(column).filter(|n| **n == 0).count();

    if zeros > ones {
        1
    } else {
        0
    }
}

fn convert(input: &Vec<u8>) -> u64 {
    let mut dec: u64 = 0;

    for number in input.iter() {
        dec = (dec << 1) ^ (*number as u64);
    }

    dec
}

fn keep_matching(grid: &Array2D<u8>, column: usize, prefix: u8) -> Array2D<u8> {
    let mut new_grid = vec![];

    for (index, bit) in grid.column_iter(column).enumerate() {
        if *bit == prefix {
            new_grid.push(grid.as_rows()[index].clone())
        }
    }

    Array2D::from_rows(&new_grid)
}

fn part_1(bits: Array2D<u8>) -> Result<u64, anyhow::Error> {
    let mut gamma = vec![];
    let mut epsilon = vec![];

    for (column, _) in bits.row_iter(0).enumerate() {
        gamma.push(find_most_common(&bits, column));
        epsilon.push(find_least_common(&bits, column));
    }

    Ok(convert(&gamma) * convert(&epsilon))
}

fn part_2(bits: Array2D<u8>) -> Result<u64, anyhow::Error> {
    let mut oxygen_grid = bits.clone();
    let mut co2_grid = bits.clone();

    let mut o_column = 0;
    let mut c_column = 0;

    // for each value we check the most common bit in the column
    // then create a new grid with only numbers that also have that same digit in its column
    while oxygen_grid.column_len() > 1 {
        let most_common = find_most_common(&oxygen_grid, o_column);
        let least_common = find_least_common(&oxygen_grid, o_column);
        // if they are the same use 1 for oxygen
        if most_common == least_common {
            oxygen_grid = keep_matching(&oxygen_grid, o_column, 1)
        } else {
            oxygen_grid = keep_matching(&oxygen_grid, o_column, most_common);
        }

        o_column += 1;
    }

    // for each value we check the least common bit in the column
    // then create a new grid with only numbers that also have that same digit in its column
    while co2_grid.column_len() > 1 {
        let most_common = find_most_common(&co2_grid, c_column);
        let least_common = find_least_common(&co2_grid, c_column);
        // if they are the same use 0 for CO2
        if most_common == least_common {
            co2_grid = keep_matching(&co2_grid, c_column, 0)
        } else {
            co2_grid = keep_matching(&co2_grid, c_column, least_common);
        }
        c_column += 1;
    }

    Ok(convert(&oxygen_grid.as_rows()[0]) * convert(&co2_grid.as_rows()[0]))
}

fn main() -> Result<(), anyhow::Error> {
    let matches = App::new("AoC 2021: day_3")
        .version("1.0")
        .author("Ricky Hosfelt <ricky_github@hosfe.lt>")
        .about("Solution to AoC day_3")
        .subcommand(
            SubCommand::with_name("part1").about("day_3 part 1").arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .help("/path/to/puzzle.input")
                    .required(true)
                    .takes_value(true),
            ),
        )
        .subcommand(
            SubCommand::with_name("part2").about("day_3 part 2").arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .help("/path/to/puzzle.input")
                    .required(true)
                    .takes_value(true),
            ),
        )
        .get_matches();

    // day_3 part_1
    if let Some(ref matches) = matches.subcommand_matches("part1") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(
                matches
                    .value_of("input")
                    .context("Error no value supplied for --input")?,
            )?;

            println!("day_3 part 1: {}", part_1(parse_input(&total_inputs))?);
        }
    }

    // day_3 part_2
    if let Some(ref matches) = matches.subcommand_matches("part2") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(
                matches
                    .value_of("input")
                    .context("Error no value supplied for --input")?,
            )?;

            println!("day_3 part 2: {}", part_2(parse_input(&total_inputs))?);
        }
    }

    Ok(())
}
