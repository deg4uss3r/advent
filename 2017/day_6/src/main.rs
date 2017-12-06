//http://adventofcode.com/2017/day/6
use std::collections::HashSet;

#[derive(Debug,Eq,Hash,PartialEq)]
pub struct State {
    banks: Vec<usize>
}

fn memory_clear(input_memory: Vec<usize>) -> usize {
    let mut state_hash: HashSet<State> = HashSet::new();
    let mut steps = 0; //counting number of steps program takes
    let mut memory = input_memory.clone(); 

    loop { //infinite loop to keep reallocating memory
        let memory_state: State = State{banks: memory.clone()};

        //checking to see if we've seen this memory state before
        if state_hash.contains(&memory_state) {
            return steps;
        }
        else {
            state_hash.insert(memory_state);
        }

        let mut highest = 0;
        let mut highest_index = 0;
        let mut last_number = 0; 

        for (index,number) in memory.iter().enumerate() {
            if number > &last_number {
                highest = *number;
                highest_index = index;
                last_number = *number;
            }
        }

        //reallocating highest number
        let mut reallocation_amount = 1;
        if highest == memory.len()-1 {
            reallocation_amount = 0;
        }
        else if highest%(memory.len()-1) != 0 {
            reallocation_amount = highest%(memory.len()-1);
        }
        else {
            if highest > memory.len() {
                reallocation_amount = highest%memory.len();
            }
            else {
                reallocation_amount = 1;
            }
        }

        let mut starting_index = highest_index+1;
        
        if highest == reallocation_amount {
            reallocation_amount = 0;
        }

        for _ in 0..(highest-reallocation_amount) {
            //go back to the beginning of vector
            if starting_index >= memory.len() {
                starting_index = 0;
            }

            //skip the highest number
            if starting_index == highest_index {
                starting_index+=1;
            }
            memory[starting_index]+=1;
            memory[highest_index]-=1;
            starting_index+=1;        
        }

        steps+=1;
    }
}

fn main() {
    //should be 5
    let test_memory = vec!(0,2,7,0);
    println!("Broke out in {} steps!", memory_clear(test_memory));

    //challenge vec
    let challenge_memory = vec!(0,5,10,0,11,14,13,4,11,8,8,7,1,4,12,11);
    println!("Broke out in {} steps!", memory_clear(challenge_memory)); 
}