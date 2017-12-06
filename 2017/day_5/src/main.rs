//http://adventofcode.com/2017/day/5

fn maze_jumper(mut input: Vec<isize>) -> usize {
    let mut maze_escape = false; 
    let mut value = 0;
    let mut previous_index = 0;
    let mut index = 0;
    let mut fakedex: isize = 0;
    let mut steps = 1;

    while !maze_escape {
        println!("{:?}", input);
        value = input[index];
        previous_index = index;
        
        if value > 0 {
            index += value as usize;
        }
        else if index < value.abs() as usize {
            maze_escape = true;
        }
        else {
            index -= value.abs() as usize;
        }
        
        fakedex += value;
        input[previous_index]+=(1 as isize);
        
        //we broke out of the maze
        if (fakedex < 0) || (fakedex >= input.len() as isize) {
            maze_escape = true
        }
        else {
            steps+=1;
        }
    }
    steps
}

fn main() {
    let mut test_maze = vec!(0,3,0,1,-3);
    println!("Broke out in {} steps!", maze_jumper(test_maze));
}