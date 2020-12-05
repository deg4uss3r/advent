use std::collections::HashMap;
use std::error::Error;
use std::io::Read;
use std::io::stdin;

fn parse_input(input: String) -> Result<Vec<String>, Box<Error>> {
    //magic for parsing input to a format I need
    let test = input.to_owned();
    let new_types: Vec<&str> = test.split("").collect();
    let mut types: Vec<String> = new_types.iter().map(|x| x.to_string()).collect();

    //removing "" at the beginning and end of the string
    types.remove(0);
    let types_len = types.len()-1;
    types.remove(types_len);
    
    Ok(types.to_owned())
}

//more functions for solving the problem!
fn react(mut types: Vec<String>) -> Result<usize, Box<Error>> {

    let mut first;
    let mut second;

    let mut index = 0;

    while index < types.len()-1 {
        first = types[index].to_owned();
        second = types[index+1].to_owned();
    
        if first == "\n" {
            types.remove(index);
        } else if second == "\n" {
            types.remove(index+1);
        } else if first.as_bytes()[0]+32 == second.as_bytes()[0] ||  first.as_bytes()[0] == second.as_bytes()[0]+32 {
            types.remove(index);
            types.remove(index);
            index = 0; //start over (not recursion ha!)
        } else {
            index+=1;
        }
    }

    Ok(types.len())
}

fn remove_polys(types: Vec<String>) -> Result<usize,Box<Error>> {
    let mut smallest_suit: HashMap<String, usize> = HashMap::new();

    for alpha in 65..91 {
        let mut test_suit = types.clone();
        let match_alpha = ((alpha as u8) as char).to_string();
        test_suit.retain(|x| x.to_ascii_uppercase() != match_alpha);
        let size = react(test_suit)?;
        smallest_suit.insert(match_alpha, size);
    }

    let mut smallest = std::usize::MAX;

    for (letter,small) in smallest_suit.iter() {
        if small < &smallest {
            smallest = *small;
        }
    }

    Ok(smallest)

}

fn main() -> Result<(), Box<Error>> {
    let mut input = String::new();
       stdin().read_to_string(&mut input)?;

   // let input = "dabAcCaCBAcCcaDA".to_string();
    
    let puzzle_input = parse_input(input)?;
    let new_input = puzzle_input.clone();
    println!("Part one: {}", react(puzzle_input)?);
    println!("Part two: {}", remove_polys(new_input)?);

    Ok(())
}
