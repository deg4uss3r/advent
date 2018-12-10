use std::error::Error;
use std::io::Read;
use std::io::stdin;

#[derive(Debug)]
struct Node {
    metadata: Vec<u32>,
    children: Vec<Node>,
    len: usize,
}

impl Node {
    pub fn expand(input: Vec<u32>) -> Result<Node, Box<Error>> {
        if input.len() < 2 {
            return Err(From::from("Error length no long enough for Node"));
        }

        let (child_len, metadata_len) = (input[0], input[1]);

        let mut node = Node{
            metadata: Vec::new(),
            children: Vec::new(),
            len: 2,
        };

        for _child_number in 0..child_len {
            let child = Node::expand(input[node.len..].to_vec())?;
            node.len+=child.len;
            node.children.push(child);
        }

        for _metadata_number in 0..metadata_len {
            node.metadata.push(input[node.len]);
            node.len+=1;
        }

        Ok(node)
    }

    pub fn full_tree_metadata(&self) -> Result<u32, Box<Error>> {
        let mut sum = self.metadata.iter().sum();

        for child in self.children.iter() {
            sum += child.full_tree_metadata()?;
        }

        Ok(sum)
    }

    pub fn node_value(&self) -> Result<u32, Box<Error>> {
        let mut value: u32 = 0;
        let mut index;

        if self.children.len() == 0 {
            let out: u32 = self.metadata.iter().sum();
            value += out;
        } else {
            for i in self.metadata.iter() {
                if i == &0 {
                    continue;
                } else {
                    index = i-1;
                }

                let out: u32 = match self.children.get(index as usize) {
                    Some(child) => child.node_value()?,
                    None => 0u32
                };

                value += out;
            }
        }

        Ok(value)
    }
}

fn parse_input(input: String) -> Result<Vec<u32>, Box<Error>> {
    //magic for parsing input to a format I need
    let output: Vec<&str> = input.split(" ").collect();
    let formatted_output: Vec<u32> = output.iter().map(|x| x.parse::<u32>()
        .expect("Error parsing to u32")).collect();

    Ok(formatted_output)
}

//more functions for solving the problem!

fn main() -> Result<(), Box<Error>> {
    let mut puzzle_input = String::new();
        stdin().read_to_string(&mut puzzle_input)?;
    puzzle_input.pop(); //remove new line bullshit

    //part_one: 138
//    let puzzle_input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2".to_string();

    let input = parse_input(puzzle_input)?;
    let tree = Node::expand(input)?;
    println!("Part one: {:#?}", tree.full_tree_metadata()?);
    println!("Part two: {}", tree.node_value()?);

    Ok(())
}
