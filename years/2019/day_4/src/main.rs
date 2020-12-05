use std::collections::HashMap;
use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("AoC: day_4")
        .version("1.0")
        .author("degausser <Ricky@Hosfelt.io>")
        .about("Solution to AoC day_4")
         .subcommand(SubCommand::with_name("part_1")
            .about("day_4 part_1")
            .arg(Arg::with_name("low")
                .short("l")
                .long("low")
                .help("lower bound value from puzzle input")
                .takes_value(true))
            .arg(Arg::with_name("high")
                .short("h")
                .long("high")
                .help("upper bound value from puzzle input")
                .takes_value(true)))
         .subcommand(SubCommand::with_name("part_2")
            .about("day_4 part_2")
            .arg(Arg::with_name("low")
                .short("l")
                .long("low")
                .help("lower bound value from puzzle input")
                .takes_value(true))
            .arg(Arg::with_name("high")
                .short("h")
                .long("high")
                .help("upper bound value from puzzle input")
                .takes_value(true)))
         .get_matches();

    // day_4 part_1   
    if let Some(ref matches) = matches.subcommand_matches("part_1") {
        if matches.is_present("low") && matches.is_present("high") {
            let mut low: u64 = u64::from_str_radix(matches.value_of("low").unwrap(), 10).unwrap();
            let high: u64 =  u64::from_str_radix(matches.value_of("high").unwrap(), 10).unwrap();
            let mut valid_pwds: u64 = 0;
            
            while low <= high {
                // Lots of conversions to check if sorted
                let num_string = low.to_string();
                let mut num_values: Vec<&str> = num_string.split("").collect();
                
                //There's two blanks before and after the number split when doing this... 
                num_values.pop();
                num_values.remove(0);

                //Creating an array of the numbers as is and checking if they are sorted and then
                //deduped 
                let num_n: Vec<u64> = num_values.iter().map(|x| u64::from_str_radix(x, 10).unwrap()).collect();
                
                //There's some nightly features (.is_sorted()) that will help with this cloning in
                //the future 
                let mut sort_num_n = num_n.clone();
                sort_num_n.sort();
                let mut dedup_num_n = num_n.clone();
                dedup_num_n.dedup();
               
                if num_n == sort_num_n {
                    //checking there's at least double value back-to-back
                    if num_n.len() != dedup_num_n.len() {
                        valid_pwds += 1;
                    }
                }
                low+=1;
            }

            println!("Number of valid passwords: {}", valid_pwds);
        }
    }

    // day_4 part_2
    if let Some(ref matches) = matches.subcommand_matches("part_2") {
        if matches.is_present("low") && matches.is_present("high") {
            let mut low: u64 = u64::from_str_radix(matches.value_of("low").unwrap(), 10).unwrap();
            let high: u64 =  u64::from_str_radix(matches.value_of("high").unwrap(), 10).unwrap();
            let mut valid_pwds: u64 = 0;
            
            while low <= high {
                // Lots of conversions to check if sorted
                let num_string = low.to_string();
                let mut num_values: Vec<&str> = num_string.split("").collect();
                
                //There's two blanks before and after the number split when doing this... 
                num_values.pop();
                num_values.remove(0);

                //Creating an array of the numbers as is and checking if they are sorted and then
                //match the double pair rule set 
                let num_n: Vec<u64> = num_values.iter().map(|x| u64::from_str_radix(x, 10).unwrap()).collect();
                
                //There's some nightly features (.is_sorted()) that will help with this cloning in
                //the future 
                let mut sort_num_n = num_n.clone();
                sort_num_n.sort();
                let mut dedup_num_n = num_n.clone();
                dedup_num_n.dedup();
                
                if num_n == sort_num_n {
                    //checking the number has repeated digits that are exactly 2, somewhere in the
                    //number
                    if num_n.len() != dedup_num_n.len() {

                        let mut hash_n: HashMap<u64, u64> = HashMap::new();

                        for n in num_n.iter() {
                            let n_count = hash_n.entry(*n).or_insert(0);
                            *n_count += 1;
                        }
    
                        let mut valid_digits = 0;
                        for (_number, count) in hash_n.iter() {
                            if count == &2 {
                                valid_digits += 1; 
                            }
                        }

                        if valid_digits > 0 {
                            valid_pwds +=1;
                        }
                    }
                }

                low+=1;
            }

            println!("Number of valid passwords: {}", valid_pwds);
        }

    }
}  
