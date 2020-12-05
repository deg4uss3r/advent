//http://adventofcode.com/2017/day/17

fn spinlock(input: usize) -> usize {
    let mut out_number = 0;
    let mut index = 0;
    let mut path: Vec<usize> = vec!(0);

    for i in 1..2018 {
        //insert number after skipping ahead input times
        //using the number here unstead of path.len() to optimize slightly
        index = ((input+index)%i)+1;  
        //finding index of 2017 without another O(n)
        if i == 2017 {
            out_number = index;
        }

        path.insert(index, i);
    } 

    path[out_number+1]
}

fn brute_spinlock(input: usize) -> usize {
    let mut index = 0;
    let mut after_zero = 1;

    for i in 1..50000001 {
        index = ((input+index)%i)+1;
        if index == 1 { //check for number after 0 
            after_zero = i;
        }
    } 
    after_zero 
}

fn main() {
    //test 1 should be 638
    let input: usize = 3;
    println!("Test: {}", spinlock(input));

    //challenge
    let input: usize = 355;
    println!("Challenge: {}", spinlock(input));   
    println!("Challenge, Part 2: {}", brute_spinlock(input));
}
