use std::collections::HashSet;
use std::error::Error;
use std::io::stdin;
use std::io::Read;

fn compute_frequency(input: &str) -> Result<i64, Box<Error>> {
    let split_freqs = input.split("\n");
    let num_freqs: Vec<i64> = split_freqs
        .map(|x| {
            x.parse::<i64>()
                .expect(&format!("Error could not parse {} as i64", x))
        })
        .collect();

    Ok(num_freqs.iter().fold(0, |accumulator, x| accumulator + x))
}

fn compute_duplicate_frequency(input: &str) -> Result<i64, Box<Error>> {
    let split_freqs = input.split("\n");
    let num_freqs: Vec<i64> = split_freqs
        .map(|x| {
            x.parse::<i64>()
                .expect(&format!("Error could not parse {} as i64", x))
        })
        .collect();

    let mut frequencies: HashSet<i64> = HashSet::new();
    let mut freq_sum = 0;
    let mut freq_index = 0;

    while !frequencies.contains(&freq_sum) {
        frequencies.insert(freq_sum); //insert our current sum after we check it is not in the list

        if freq_index >= num_freqs.len() {
            //loop around the frequencies if we made it to the end of the list
            freq_index = 0;
        }

        freq_sum += num_freqs[freq_index]; //compute the new freq sum
        freq_index += 1;
    }

    Ok(freq_sum)
}

fn main() -> Result<(), Box<Error>> {
    let mut puzzle_input = String::new();
    stdin().read_to_string(&mut puzzle_input)?;

    println!("Part one answer: {}", compute_frequency(&puzzle_input)?);
    println!(
        "Part two answer: {}",
        compute_duplicate_frequency(&puzzle_input)?
    );
    Ok(())
}
