#![allow(non_snake_case)]
use std::{fmt, fs::File, io::prelude::*};

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

use std::convert::TryFrom;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Seat {
    Occupied,
    Unoccupied,
    Floor,
}

#[derive(Debug)]
enum ParseError {
    UnknownInput(String),
}

impl<'a> TryFrom<&str> for Seat {
    type Error = ParseError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        match input {
            "L" => Ok(Self::Unoccupied),
            "#" => Ok(Self::Occupied),
            "." => Ok(Self::Floor),
            _ => Err(ParseError::UnknownInput(input.to_string())),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unexpected character: '{}'", self)
    }
}

impl std::error::Error for ParseError {}

type SeatMap = Vec<Vec<Seat>>;

trait Advance {
    fn advance(&mut self);
    fn advance2(&mut self);
}

trait Count {
    fn count(&self) -> u64;
}

trait Anthropophobia {
    fn anthropophobia(&self, row: usize, column: usize) -> u64;
    fn anthropophobia2(&self, row: usize, column: usize) -> u64;
}

impl Advance for SeatMap {
    fn advance(&mut self) {
        let this_seat_map = self.to_vec();
        for (row_index, seat_row) in self.to_vec().iter().enumerate() {
            for (column_index, seat) in seat_row.iter().enumerate() {
                if *seat == Seat::Unoccupied
                    && this_seat_map.anthropophobia(row_index, column_index) == 0
                {
                    self[row_index][column_index] = Seat::Occupied;
                } else if *seat == Seat::Floor {
                    continue;
                } else {
                    if this_seat_map.anthropophobia(row_index, column_index) >= 4 {
                        self[row_index][column_index] = Seat::Unoccupied;
                    }
                }
            }
        }
    }

    fn advance2(&mut self) {
        let this_seat_map = self.to_vec();
        for (row_index, seat_row) in self.to_vec().iter().enumerate() {
            for (column_index, seat) in seat_row.iter().enumerate() {
                if *seat == Seat::Unoccupied
                    && this_seat_map.anthropophobia2(row_index, column_index) == 0
                {
                    self[row_index][column_index] = Seat::Occupied;
                } else if *seat == Seat::Floor {
                    continue;
                } else {
                    if this_seat_map.anthropophobia2(row_index, column_index) >= 5 {
                        self[row_index][column_index] = Seat::Unoccupied;
                    }
                }
            }
        }
    }
}

impl Count for SeatMap {
    fn count(&self) -> u64 {
        let mut count = 0_u64;
        for c in self.iter() {
            for r in c.iter() {
                if *r == Seat::Occupied {
                    count += 1;
                }
            }
        }
        count
    }
}

impl Anthropophobia for SeatMap {
    fn anthropophobia(&self, row: usize, column: usize) -> u64 {
        let mut occupied_seats = 0_u64;

        //up same index previous vec
        if row > 0 {
            if self[row - 1][column] == Seat::Occupied {
                occupied_seats += 1;
            }
        }

        //down same index next vec
        if row + 1 < self.len() {
            if self[row + 1][column] == Seat::Occupied {
                occupied_seats += 1;
            }
        }

        //left one index down same vec
        if column > 0 {
            if self[row][column - 1] == Seat::Occupied {
                occupied_seats += 1;
            }
        }

        //right one index up same vec
        if column + 1 < self[row].len() {
            if self[row][column + 1] == Seat::Occupied {
                occupied_seats += 1;
            }
        }

        //diagonal UL, one index down, previous vec
        if row > 0 && column > 0 {
            if self[row - 1][column - 1] == Seat::Occupied {
                occupied_seats += 1;
            }
        }

        //diagonal UR, one index up, previous vec
        if row > 0 && column + 1 < self[row].len() {
            if self[row - 1][column + 1] == Seat::Occupied {
                occupied_seats += 1;
            }
        }

        //diagonal LL, one index down, next vec
        if row + 1 < self.len() && column > 0 {
            if self[row + 1][column - 1] == Seat::Occupied {
                occupied_seats += 1;
            }
        }

        //diagonal LR, one index up, next vec
        if row + 1 < self.len() && column + 1 < self[row].len() {
            if self[row + 1][column + 1] == Seat::Occupied {
                occupied_seats += 1;
            }
        }
        occupied_seats
    }

    fn anthropophobia2(&self, row: usize, column: usize) -> u64 {
        let mut occupied_seats = 0_u64;

        //up same index previous vec
        let mut Urow = row;
        while Urow > 0 {
            if self[Urow - 1][column] == Seat::Unoccupied {
                break;
            }
            if self[Urow - 1][column] == Seat::Occupied {
                occupied_seats += 1;
                break;
            }
            Urow -= 1;
        }

        //down same index next vec
        let mut Drow = row;
        while Drow + 1 < self.len() {
            if self[Drow + 1][column] == Seat::Unoccupied {
                break;
            }
            if self[Drow + 1][column] == Seat::Occupied {
                occupied_seats += 1;
                break;
            }

            Drow += 1;
        }

        //left one index down same vec
        let mut Lcolumn = column;
        while Lcolumn > 0 {
            if self[row][Lcolumn - 1] == Seat::Unoccupied {
                break;
            }
            if self[row][Lcolumn - 1] == Seat::Occupied {
                occupied_seats += 1;
                break;
            }

            Lcolumn -= 1;
        }

        //right one index up same vec
        let mut Rcolumn = column;
        while Rcolumn + 1 < self[row].len() {
            if self[row][Rcolumn + 1] == Seat::Unoccupied {
                break;
            }
            if self[row][Rcolumn + 1] == Seat::Occupied {
                occupied_seats += 1;
                break;
            }

            Rcolumn += 1;
        }

        //diagonal UL, one index down, previous vec
        let mut ULrow = row;
        let mut ULcolumn = column;
        while ULrow > 0 && ULcolumn > 0 {
            if self[ULrow - 1][ULcolumn - 1] == Seat::Unoccupied {
                break;
            }
            if self[ULrow - 1][ULcolumn - 1] == Seat::Occupied {
                occupied_seats += 1;
                break;
            }

            ULrow -= 1;
            ULcolumn -= 1;
        }

        //diagonal UR, one index up, previous vec
        let mut URrow = row;
        let mut URcolumn = column;
        while URrow > 0 && URcolumn + 1 < self[row].len() {
            if self[URrow - 1][URcolumn + 1] == Seat::Unoccupied {
                break;
            }
            if self[URrow - 1][URcolumn + 1] == Seat::Occupied {
                occupied_seats += 1;
                break;
            }

            URrow -= 1;
            URcolumn += 1;
        }

        //diagonal LL, one index down, next vec
        let mut LLrow = row;
        let mut LLcolumn = column;
        while LLrow + 1 < self.len() && LLcolumn > 0 {
            if self[LLrow + 1][LLcolumn - 1] == Seat::Unoccupied {
                break;
            }
            if self[LLrow + 1][LLcolumn - 1] == Seat::Occupied {
                occupied_seats += 1;
                break;
            }

            LLrow += 1;
            LLcolumn -= 1;
        }

        //diagonal LR, one index up, next vec
        let mut LRrow = row;
        let mut LRcolumn = column;

        while LRrow + 1 < self.len() && LRcolumn + 1 < self[row].len() {
            if self[LRrow + 1][LRcolumn + 1] == Seat::Unoccupied {
                break;
            }
            if self[LRrow + 1][LRcolumn + 1] == Seat::Occupied {
                occupied_seats += 1;
                break;
            }

            LRrow += 1;
            LRcolumn += 1;
        }
        occupied_seats
    }
}

fn parse_example_input(input: &str) -> Result<SeatMap, anyhow::Error> {
    let mut seats: SeatMap = Vec::new();

    let rows: Vec<&str> = input.split("\n").collect();

    for row in rows.iter() {
        let mut seat_row: Vec<Seat> = Vec::new();
        let str_seat: Vec<&str> = row.split("").collect();
        for s in str_seat.iter() {
            if s == &"" {
                continue;
            }
            seat_row.push(Seat::try_from(*s)?);
        }

        seats.push(seat_row);
    }

    Ok(seats)
}

fn parse_input(input: &str) -> Result<SeatMap, anyhow::Error> {
    parse_example_input(input)
}

fn part_1(mut seats: SeatMap) -> u64 {
    if seats[seats.len() - 1] == Vec::new() {
        seats.pop();
    }
    let mut prev_seats: SeatMap = Vec::new();

    while seats != prev_seats {
        prev_seats = seats.to_vec();
        seats.advance();
    }

    seats.count()
}

fn part_2(mut seats: SeatMap) -> u64 {
    if seats[seats.len() - 1] == Vec::new() {
        seats.pop();
    }
    let mut prev_seats: SeatMap = Vec::new();

    while seats != prev_seats {
        prev_seats = seats.to_vec();
        seats.advance2();
    }

    seats.count()
}

fn main() -> Result<(), anyhow::Error> {
    let matches = App::new("AoC 2020: day_11")
        .version("1.0")
        .author("Ricky <Ricky@Hosfelt.io>")
        .about("Solution to AoC day_11")
        .subcommand(
            SubCommand::with_name("ex1")
                .about("day_11 part_1 example")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .help("puzzle input on the cmdline")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("part_1").about("day_11 part_1").arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .help("puzzle input as a text file")
                    .takes_value(true),
            ),
        )
        .subcommand(
            SubCommand::with_name("ex2")
                .about("day_11 part_2 example")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .help("puzzle input on the cmdline")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("part_2").about("day_11 part_2").arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .help("puzzle input as a text file")
                    .takes_value(true),
            ),
        )
        .get_matches();

    //day_11 part_1 example
    if let Some(ref matches) = matches.subcommand_matches("ex1") {
        if matches.is_present("input") {
            println!(
                "Occupied seats: {}",
                part_1(parse_example_input(
                    matches
                        .value_of("input")
                        .context("Error no value supplied for --input")?
                )?)
            );
        }
    }

    // day_11 part_1
    if let Some(ref matches) = matches.subcommand_matches("part_1") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(
                matches
                    .value_of("input")
                    .context("Error no value supplied for --input")?,
            )?;
            let parsed_input = parse_input(&total_inputs)?;

            println!("Occupied seats: {}", part_1(parsed_input));
        }
    }

    // day_11 part_2 example
    if let Some(ref matches) = matches.subcommand_matches("ex2") {
        if matches.is_present("input") {
            println!(
                "Unoccupied seats with new rules: {}",
                part_2(parse_input(
                    matches
                        .value_of("input")
                        .context("Error no value supplied for --input")?
                )?)
            );
        }
    }

    // day_11 part_2
    if let Some(ref matches) = matches.subcommand_matches("part_2") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(
                matches
                    .value_of("input")
                    .context("Error no value supplied for --input")?,
            )?;
            let parsed_input = parse_input(&total_inputs)?;

            println!("Unoccupied seats with new rules: {}", part_2(parsed_input));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() -> Result<(), anyhow::Error> {
        let x = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        assert_eq!(37, part_1(parse_example_input(x)?));

        Ok(())
    }

    #[test]
    fn ex2() -> Result<(), anyhow::Error> {
        let x = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        assert_eq!(26, part_2(parse_example_input(x)?));

        Ok(())
    }
}
