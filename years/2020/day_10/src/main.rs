use std::{collections::HashMap, fs::File, io::prelude::*};

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

fn parse_input(input: &str) -> Vec<u64> {
    parse_example_input(input)
}
fn parse_example_input(input: &str) -> Vec<u64> {
    let jolt_str: Vec<&str> = input.split("\n").collect();
    jolt_str
        .iter()
        .map(|s| s.parse())
        .filter(|s| s.is_ok())
        .map(|s| s.unwrap())
        .collect::<Vec<u64>>()
}

fn part_1(mut jolts: Vec<u64>) -> u64 {
    jolts.push(0);
    jolts.push(*jolts.iter().max().unwrap() + 3);
    jolts.sort();

    let mut diffs = HashMap::new();
    let mut last_num = 0;

    for i in jolts.iter() {
        if i == &0 {
            continue;
        } else {
            let diff = i - last_num;
            let c = diffs.entry(diff).or_insert(0);
            *c += 1;
        }

        last_num = *i;
    }

    diffs.get(&3).unwrap() * diffs.get(&1).unwrap()
}

fn part_2(mut jolts: Vec<u64>) -> u64 {
    let mut paths_to_end: HashMap<u64, u64> = HashMap::new();
    let max = *jolts.iter().max().unwrap() + 3;
    jolts.push(0);
    paths_to_end.insert(0, 1);
    jolts.push(max);
    jolts.sort();

    for adapter in jolts.iter() {
        for diff in 1..4 {
            let next_adapter = adapter + diff;
            if jolts.contains(&next_adapter) {
                let adapter_paths = paths_to_end.get(adapter).unwrap().clone();
                let paths_for_adapt = paths_to_end.entry(next_adapter).or_insert(0);
                *paths_for_adapt += adapter_paths;
            }
        }
    }

    *paths_to_end.get(&max).unwrap()
}

fn main() -> Result<(), anyhow::Error> {
    let matches = App::new("AoC 2020: day_10")
        .version("1.0")
        .author("Ricky <Ricky@Hosfelt.io>")
        .about("Solution to AoC day_10")
        .subcommand(
            SubCommand::with_name("ex1")
                .about("day_10 part_1 example")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .help("puzzle input on the cmdline")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("part_1").about("day_10 part_1").arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .help("puzzle input as a text file")
                    .takes_value(true),
            ),
        )
        .subcommand(
            SubCommand::with_name("ex2")
                .about("day_10 part_2 example")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .help("puzzle input on the cmdline")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("part_2").about("day_10 part_2").arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .help("puzzle input as a text file")
                    .takes_value(true),
            ),
        )
        .get_matches();

    //day_10 part_1 example
    if let Some(ref matches) = matches.subcommand_matches("ex1") {
        if matches.is_present("input") {
            println!(
                "product of 1 and 3 jolt diffs: {}",
                part_1(parse_example_input(
                    matches
                        .value_of("input")
                        .context("Error no value supplied for --input")?
                ))
            );
        }
    }

    // day_10 part_1
    if let Some(ref matches) = matches.subcommand_matches("part_1") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(
                matches
                    .value_of("input")
                    .context("Error no value supplied for --input")?,
            )?;
            let parsed_input = parse_input(&total_inputs);

            println!("product of 1 and 3 jolt diffs: {}", part_1(parsed_input));
        }
    }

    // day_10 part_2 example
    if let Some(ref matches) = matches.subcommand_matches("ex2") {
        if matches.is_present("input") {
            println!(
                "Possible paths to end: {}",
                part_2(parse_input(
                    matches
                        .value_of("input")
                        .context("Error no value supplied for --input")?
                ))
            );
        }
    }

    // day_10 part_2
    if let Some(ref matches) = matches.subcommand_matches("part_2") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(
                matches
                    .value_of("input")
                    .context("Error no value supplied for --input")?,
            )?;
            let parsed_input = parse_input(&total_inputs);

            println!("Possible paths to end: {}", part_2(parsed_input));
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ex1_1() {
        let input = "16
10
15
5
1
11
7
19
6
12
4";
        assert_eq!(35, part_1(parse_example_input(input)));
    }

    #[test]
    fn ex1_2() {
        let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        assert_eq!(220, part_1(parse_example_input(input)));
    }

    #[test]
    fn ex2_1() {
        let input = "16
10
15
5
1
11
7
19
6
12
4";
        assert_eq!(8, part_2(parse_example_input(input)));
    }

    #[test]
    fn ex2_2() {
        let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        assert_eq!(19208, part_2(parse_example_input(input)));
    }
}
