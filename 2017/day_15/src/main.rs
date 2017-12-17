//http://adventofcode.com/2017/day/15
fn generator_a(seed: usize) -> usize {
    (seed*16807)%2147483647
}

fn generator_b(seed: usize) -> usize {
    (seed*48271)%2147483647
}

fn judge(a: usize, b:usize) -> usize {
    let mut seed_a = a;
    let mut seed_b = b;
    let mut index = 0;
    let mut count = 0;

    while index < 40000000 {
        seed_a = generator_a(seed_a);
        seed_b = generator_b(seed_b);

        if seed_a&0xffff == seed_b&0xffff {
            count+=1;
        }
        index+=1;
    }
    count
}

fn judge_more(a:usize, b:usize) -> usize {
    let mut seed_a = a;
    let mut seed_b = b;
    let mut index = 0;
    let mut count = 0;

    while index < 5000000 {
    
        seed_a = generator_a(seed_a);
        seed_b = generator_b(seed_b);
    
        while seed_a%4 != 0 {
            seed_a = generator_a(seed_a);
        }
        
        while seed_b%8 != 0 {
            seed_b = generator_b(seed_b);
        }
    
        if seed_a&0xffff == seed_b&0xffff {
            count+=1;
        }
        index+=1;
    }
    count
}

fn main() {
/*
//test 1 should be 588
println!("Pairs: {}", judge(65, 8921));

//challenge
let input = "Generator A starts with 512
Generator B starts with 191";
let input_lines: Vec<&str> = input.split("\n").collect();

let input_line_a: Vec<&str> = input_lines[0].split("with ").collect();
let input_line_b: Vec<&str> = input_lines[1].split("with ").collect();
    let a = input_line_a[1].parse::<usize>().unwrap();
    let b = input_line_b[1].parse::<usize>().unwrap();

println!("Pairs: {}", judge(a,b));

//part 2
//test 1 should be 309
println!("Part2 Pairs: {}", judge_more(65,8921));

//challenge part 2
let input = "Generator A starts with 512
Generator B starts with 191";
let input_lines: Vec<&str> = input.split("\n").collect();
let input_line_a: Vec<&str> = input_lines[0].split("with ").collect();
let input_line_b: Vec<&str> = input_lines[1].split("with ").collect();
    let a = input_line_a[1].parse::<usize>().unwrap();
    let b = input_line_b[1].parse::<usize>().unwrap();

println!("Part2 Pairs: {}", judge_more(a,b));
*/

//challenge
let input = "Generator A starts with 591
Generator B starts with 393";
let input_lines: Vec<&str> = input.split("\n").collect();

let input_line_a: Vec<&str> = input_lines[0].split("with ").collect();
let input_line_b: Vec<&str> = input_lines[1].split("with ").collect();
    let a = input_line_a[1].parse::<usize>().unwrap();
    let b = input_line_b[1].parse::<usize>().unwrap();

println!("Pairs: {}", judge(a,b));
println!("Part2 Pairs: {}", judge_more(a,b));
}