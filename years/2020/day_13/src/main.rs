use std::{fs::File, io::prelude::*, collections::HashMap};

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

#[derive(Debug)]
struct BusStop {
    minutes: u64,
    busses: Vec<Option<u64>>,
}

fn parse_input(input: &str) -> BusStop {
    let waiting_busids: Vec<&str> = input.split("\n").collect();
    let wait_time = waiting_busids[0].parse::<u64>().unwrap();
    let busids: Vec<Option<u64>> = waiting_busids[1]
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.parse::<u64>())
        .map(|s| s.ok())
        .collect();
    BusStop {
        minutes: wait_time,
        busses: busids,
    }
}

fn part_1(busses: BusStop) -> u64 {
    let lowest: HashMap<u64, u64> = busses
        .busses
        .iter()
        .filter(|s| s.is_some())
        .map(|s| {
            let mut bus = s.unwrap();
            let mut bus_schedule = 2;
            while bus < busses.minutes {
                bus = s.unwrap()*bus_schedule;
                bus_schedule += 1;
            }

            (bus, s.unwrap())
        })
        .collect();
        
    (lowest.keys().min().unwrap()-busses.minutes) * lowest[lowest.keys().min().unwrap()]
}

#[derive(Debug, Clone)]
struct BusLine {
    bus_id: u64,
    offset: u64,
}

// find the the inverse mod of these numbers
fn mod_inverse(numerator: u64, denominator: u64) -> u64 {
    let remainder = numerator % denominator; 
    
    for num in 1..denominator { 
        if (remainder * num) % denominator == 1 { 
            return num; 
        }
    }
    
    return 1;
}

// chinese remainder theorem 
fn crt(busses: Vec<BusLine>) -> u64 {
    // the product of all of the denominators (foo % denominator)
    // here these are the bus_ids (or how frequently they appear)
    let mut product = 1; 

    for bus in busses.iter() {
        product *= bus.bus_id;
    }

    // the sum of the denominator - remainder * partial product * inverse 
    let mut crt = 0_i64; 

    for bus in busses.iter() {
        // partial product is the product not including this bus_id
        let partial = product / bus.bus_id; 
        // gets the inverse mod of the partial product and this denominator 
        let inverse_partial = mod_inverse(partial, bus.bus_id);
        // add to the sum 
        crt += (bus.bus_id as i64 - bus.offset as i64) * (partial as i64) * (inverse_partial as i64); 
    }

    // the answer is remainder of the crt/product
    // this is the smallest number that matches
    return (crt % product as i64) as u64;
}

fn part_2(busses: BusStop) -> u64 {
    let mut min = u64::MIN;
    let mut busline: Vec<BusLine> = Vec::new();
    let mut offset = 0;

    for bus in busses.busses {
        if bus.is_some() {
            if bus.unwrap() > min {
                min = bus.unwrap();
            }
            busline.push(BusLine{bus_id: bus.unwrap(), offset: offset});
        }
        
        offset += 1;
    }

    // The answer is a number where % bus_id with remainder of the offset of time the bus comes 
    crt(busline)
}

//

fn main() -> Result<(), anyhow::Error> {
    let matches = App::new("AoC 2020: day_13")
        .version("1.0")
        .author("Ricky <Ricky@Hosfelt.io>")
        .about("Solution to AoC day_13")
        .subcommand(
            SubCommand::with_name("part_1").about("day_13 part_1").arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .help("puzzle input as a text file")
                    .takes_value(true),
            ),
        )
        .subcommand(
            SubCommand::with_name("part_2").about("day_13 part_2").arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .help("puzzle input as a text file")
                    .takes_value(true),
            ),
        )
        .get_matches();

    // day_13 part_1
    if let Some(ref matches) = matches.subcommand_matches("part_1") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(
                matches
                    .value_of("input")
                    .context("Error no value supplied for --input")?,
            )?;
            let parsed_input = parse_input(&total_inputs);

            println!("Minutes waited * busID: {}", part_1(parsed_input));
        }
    }

    // day_13 part_2
    if let Some(ref matches) = matches.subcommand_matches("part_2") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(
                matches
                    .value_of("input")
                    .context("Error no value supplied for --input")?,
            )?;
            let parsed_input = parse_input(&total_inputs);

            println!("sequence starts at: {}", part_2(parsed_input));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*; 

    const PUZZLE: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn ex1() {
        assert_eq!(295, part_1(parse_input(PUZZLE)));
    }

    #[test]
    fn ex2() {
        assert_eq!(1_068_781, part_2(parse_input(PUZZLE)));
    }

    #[test]
    fn ex2_1() {
        let i = "1
17,x,13,19";
        assert_eq!(3417, part_2(parse_input(i)));
    }
    #[test]
    fn ex2_2() {
        let i = "1
67,7,59,61";
        assert_eq!(754018, part_2(parse_input(i)));
    }

    #[test]
    fn ex2_3() {
        let i = "1
67,x,7,59,61";
        assert_eq!(779210, part_2(parse_input(i)));
    }

    #[test]
    fn ex2_4() {
        let i = "1
67,7,x,59,61";
        assert_eq!(1261476, part_2(parse_input(i)));
    }

    #[test]
    fn ex2_5() {
        let i = "1
1789,37,47,1889";
        assert_eq!(1202161486, part_2(parse_input(i)));
    } 

}