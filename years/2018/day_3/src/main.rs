use std::error::Error;
use std::io::Read;
use std::io::stdin;
use std::collections::HashMap;

trait Trait {}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Claim {
    id: u32,
    area: Vec<Point>,
    length: u32,
    width: u32,
}

fn parse_input(input: String) -> Result<Vec<Claim>,Box<Error>> {
    let input_claims: Vec<&str> = input.split("\n").collect();
    let mut claims: Vec<Claim> = Vec::new(); 

    for c in input_claims.iter() {
        //each claim looks like #123 @ 3,2: 5x4
        let mut parsing_vec: Vec<&str> = c.split(" ").collect();
            parsing_vec.remove(1); //remove "@"

        let id = parsing_vec[0][1..].parse::<u32>()?;
    
        let left_top_str = &parsing_vec[1].split(":").collect::<Vec<&str>>()[0];
        let left_top: Vec<&str> = left_top_str.split(",").collect();
        let left = left_top[0].parse::<u32>()?;
        let top = left_top[1].parse::<u32>()?;
    
        let length_width =  &parsing_vec[2].split("x").collect::<Vec<&str>>();
        let length = length_width[0].parse::<u32>()?;
        let width = length_width[1].parse::<u32>()?;

        //creating a vec of all points on the grid the square occupies 
        let mut points_in_square: Vec<Point> = Vec::new();
        let mut x_in = 0;
        let mut y_in = 0;

        while x_in < width {
            while y_in < length {
                points_in_square.push(Point{x: top+x_in,y: left+y_in});
                y_in+=1;
            }
            x_in+=1;
            y_in = 0;
        }
        claims.push(Claim{id: id, area: points_in_square, length: length, width: width});
    }

    Ok(claims)
}

//more functions for solving the problem!
fn claimed_areas(claims: &Vec<Claim>) -> Result<(i64,HashMap<&Point, i64>), Box<Error>> {
    let mut areas_claimed: HashMap<&Point, i64> = HashMap::new();

    for c in claims.iter() {
        for a in c.area.iter() {
            let p = areas_claimed.entry(a).or_insert(-1);
                *p+=1;
        }
    }

    let mut area_taken = 0;

    for i in areas_claimed.values() {
        if i > &0 {
            area_taken+=1;
        }
    }

    Ok((area_taken, areas_claimed))
}

fn lonely_claim(claims: &Vec<Claim>, points_taken: HashMap<&Point, i64>) -> Result<u32, Box<Error>> {
    for claim in claims.iter() {
        let mut bad_claim = false;
        for point in claim.area.iter() {
            if points_taken.get(point).unwrap() > &0 {
                bad_claim = true;
                break;
            }
        }

        if !bad_claim {
            return Ok(claim.id);
        }
    }

    Err(From::from("Did not find any claims that have no overlap"))
}


fn main() -> Result<(), Box<Error>> {
    let mut puzzle_input = String::new();
        stdin().read_to_string(&mut puzzle_input)?;
    
    let claims = parse_input(puzzle_input)?;
    let (claimed, areas_claimed) = claimed_areas(&claims)?;

    println!("Part one: {}", claimed);
    println!("Part two: {}", lonely_claim(&claims, areas_claimed)?);
    Ok(())
}
