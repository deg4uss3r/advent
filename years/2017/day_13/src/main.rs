//http://adventofcode.com/2017/day/13

#[derive(Debug)]
pub struct Brick {
    range: usize,
    depth: usize,
    scanner_index: usize,
    decrement: bool
}

fn parse_input(input:String) -> Vec<Brick> {
    let collect: Vec<&str> = input.split("\n").collect();
    let mut output: Vec<Brick> = Vec::new();
    
    for c in collect.iter() {
        let out:Vec<&str> = c.split(": ").collect();
        
        let brick = Brick{range:out[0].parse::<usize>().unwrap(),
            depth:out[1].parse::<usize>().unwrap(),
            scanner_index:0,
            decrement:false
        };
        
        output.push(brick);
    }
    
    output
}

fn increment_scanner(firewall:&mut Vec<Brick>) {
    for i in firewall.iter_mut() {
        if !i.decrement {
            if i.scanner_index < i.depth-1 {
                i.scanner_index+=1;
            }
            else {
                i.scanner_index-=1;
                i.decrement = true;
            }
        }
        else {
            if i.scanner_index == 0 {
                i.scanner_index+=1;
                i.decrement = false;
            }
            else {
                i.scanner_index-=1;
            }
        }
    }
}

fn packet_run(input:String) -> usize {
    let mut firewall = parse_input(input);
    let mut severity = 0;
    let mut packet_index = 0;
    let packet_scanner_index = 0;
    let mut ranges:Vec<usize> = Vec::new();
    let mut packet_index_check = 0;
    
    for i in firewall.iter() {
        ranges.push(i.range);
    }
    //lazy way to get max
    ranges.sort();
    let max_range = ranges[ranges.len()-1];

    while packet_index <= max_range {
        if ranges.iter().any(|x| x==&packet_index) {
            if firewall[packet_index_check].scanner_index == packet_scanner_index {
                severity+=firewall[packet_index_check].range*firewall[packet_index_check].depth;
            }
            packet_index_check+=1;
        }
           
        increment_scanner(&mut firewall);
        packet_index+=1;
    }
    severity
}

fn main() {

    let input = "0: 3
1: 2
4: 4
6: 4";

    println!("final severity: {}", packet_run(input.to_string()));
    
    let input = "0: 4
1: 2
2: 3
4: 4
6: 6
8: 5
10: 6
12: 6
14: 6
16: 8
18: 8
20: 9
22: 12
24: 8
26: 8
28: 8
30: 12
32: 12
34: 8
36: 12
38: 10
40: 12
42: 12
44: 10
46: 12
48: 14
50: 12
52: 14
54: 14
56: 12
58: 14
60: 12
62: 14
64: 18
66: 14
68: 14
72: 14
76: 14
82: 14
86: 14
88: 18
90: 14
92: 17";

    println!("final severity: {}", packet_run(input.to_string()));
    
}