use std::collections::HashMap;
use std::error::Error;
use std::io::stdin;
use std::io::Read;

fn calculate_checksum(input: &str) -> Result<u64, Box<Error>> {
    let words: Vec<&str> = input.split("\n").collect();
    let mut master_word_count: Vec<HashMap<char, u64>> = Vec::new();

    for word in words.iter() {
        let mut letter_count: HashMap<char, u64> = HashMap::new();
        for c in word.chars() {
            let counter = letter_count.entry(c).or_insert(0);
            *counter += 1;
        }
        master_word_count.push(letter_count);
    }

    let mut two_count = 0;
    let mut three_count = 0;

    for word in master_word_count.iter() {
        let mut add_two = false;
        let mut add_three = false;

        for (_letter, count) in word.iter() {
            if count == &2 {
                add_two = true;
            }

            if count == &3 {
                add_three = true;
            }
        }
        if add_two {
            two_count += 1;
        }
        if add_three {
            three_count += 1;
        }
    }

    Ok(two_count * three_count)
}

fn find_prototype(input: &str) -> Result<String, Box<Error>> {
    let mut same_letters = String::new();

    let words: Vec<&str> = input.split("\n").collect();
    let mut letters: Vec<Vec<char>> = Vec::new();

    for w in words.iter() {
        letters.push(w.chars().collect());
    }

    let mut found = false;

    while !found {
        let test_word = letters.remove(0);

        for l in letters.iter() {
            let mut count = 0;
            let matching_letters: Vec<_> = test_word
                .iter() //for each in the first vector
                .zip(l.iter()) //zip the vectors together to compare each element of the first to the  second vector
                .filter_map(|(a, b)| { //desugar the tuple from the zip to a and b
                    if a != b { //if a!=b increase count, and don't return the letter
                        count += 1;
                        None
                    } else { //return the letter so we can report matching letters if equal
                        Some(a)
                    }
                })
                .collect(); //collect the returned matching letters to a vector

            if count == 1 {
                found = true; //break out of the loop there's only one match in a set
                same_letters = matching_letters.into_iter().collect(); //move the vector of letters into a string 
            }
        }
    }

    Ok(same_letters)
}

fn main() -> Result<(), Box<Error>> {
    let mut puzzle_input = String::new();
    stdin().read_to_string(&mut puzzle_input)?;

    println!("Part one: {}", calculate_checksum(&puzzle_input)?);
    println!("Part two: {}", find_prototype(&puzzle_input)?);

    Ok(())
}
