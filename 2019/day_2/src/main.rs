use clap::{Arg, App, SubCommand};
use std::fs::File;
use std::io::prelude::*;

fn read_input_file(input_path: String) -> Result<String, std::io::Error> {
    let mut file = File::open(input_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_string())
}

fn parse_input(input_values: String) -> Vec<u64> {
    let parsed_values: Vec<&str> = input_values.split(",").collect();
    let converted_parsed: Vec<u64> = parsed_values.iter().map(|x| u64::from_str_radix(x, 10).unwrap()).collect();
    converted_parsed
}

trait Op {
    fn add_v(&mut self, one: usize, two: usize, store: usize);
    fn mul_v(&mut self, one: usize, two: usize, store: usize);
    fn tune(&mut self, noun: u64, verb: u64);
}

impl Op for Vec<u64> {
    fn add_v(&mut self, one: usize, two: usize, store: usize) {
        let store_position = self[store] as usize;
        let add_value = self[self[one] as usize] + self[self[two] as usize];
        self.remove(store_position);
        self.insert(store_position, add_value);
    }

    fn mul_v(&mut self, one: usize, two: usize, store: usize) {
        let store_position = self[store] as usize;
        let mul_value = self[self[one] as usize] * self[self[two] as usize];
        self.remove(store_position);
        self.insert(store_position, mul_value);
    }

    fn tune(&mut self, noun: u64, verb: u64) {
        self.remove(1);
        self.insert(1, noun);

        self.remove(2);
        self.insert(2, verb);
    }
}

fn compute(opcodes: &mut Vec<u64>) -> u64 {
    let mut indx: usize = 0;


    while indx < opcodes.len() {
        let code = opcodes[indx]; 
        
        if code == 1 {
            opcodes.add_v(indx+1, indx+2, indx+3);
        } else if code == 2 {
            opcodes.mul_v(indx+1, indx+2, indx+3);
        } else if code == 99 {
            return opcodes[0];
        } else {
            println!("Error code was not one of 1, 2, or 99: {}", code)
        }

        indx += 4;
    }
    
    opcodes[0]
}

fn compute_for_value(opcodes: &mut Vec<u64>, target: u64) -> u64 {
    let mut noun: u64 = 0;
    let mut verb: u64 = 0;
   
    while noun < opcodes.len() as u64 {
        while verb < opcodes.len() as u64 {
            let mut t_opcodes = opcodes.clone();
            t_opcodes.tune(noun, verb);

            //Compute value and look for 19690720 (target)
            let mut indx: usize = 0;

            while indx < t_opcodes.len() {
                let code = t_opcodes[indx]; 
        
                if code == 1 {
                    t_opcodes.add_v(indx+1, indx+2, indx+3);
                } else if code == 2 {
                    t_opcodes.mul_v(indx+1, indx+2, indx+3);
                } else if code == 99 {
                    indx = usize::max_value()-4;
                } else {
                    println!("Error code was not one of 1, 2, or 99: {}", code);
                    indx = usize::max_value()-4;
                }

                indx += 4;
            }
            if t_opcodes[0] == target {
                return 100 * noun + verb;
            }

            verb += 1;
        }
        noun += 1;
        verb = 0;
    }

    0
}

fn main() {
    let matches = App::new("AoC: day_2")
        .version("1.0")
        .author("degausser <Ricky@Hosfelt.io>")
        .about("Solution to AoC day_2")
        .subcommand(SubCommand::with_name("ex1")
            .about("day_2 part_1 example")
            .arg(Arg::with_name("input")
                .short("i")
                .long("input")
                .help("input of mass (one number to test)")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("part_1")
            .about("day_2 part_1")
            .arg(Arg::with_name("input")
                .short("i")
                .long("input")
                .help("input of masses (your program input) in a file with new line spaces")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("ex2")
            .about("day_2 part_2 example")
            .arg(Arg::with_name("input")
                .short("i")
                .long("input")
                .help("input of mass (one number to test)")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("part_2")
            .about("day_2 part_2")
            .arg(Arg::with_name("input")
                .short("i")
                .long("input")
                .help("input of masses (your program input) in a file with new line spaces")
                .takes_value(true)))
        .get_matches();

    //day_2 part_1 example
    if let Some(ref matches) = matches.subcommand_matches("ex1") {
        if matches.is_present("input") {
            let i_values = matches.value_of("input").unwrap().to_string();
            let mut values_parsed = parse_input(i_values);
            println!("Final Value at index 0: {}", compute(&mut values_parsed));
        }
    }
    
    // day_2 part_1   
    if let Some(ref matches) = matches.subcommand_matches("part_1") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(matches.value_of("input").unwrap().to_string());
            let mut values_parsed = parse_input(total_inputs.unwrap());
            
            //Part of the problem is to replace index 1 with 12 and 2 with 2
            values_parsed.tune(12, 2);
            println!("Final value at index 0: {}", compute(&mut values_parsed));
        }
    }

    // day_2 part_2
    if let Some(ref matches) = matches.subcommand_matches("part_2") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(matches.value_of("input").unwrap().to_string());
            let mut values_parsed = parse_input(total_inputs.unwrap());
            println!("{}", compute_for_value(&mut values_parsed, 19690720));
        }
    }
}  
