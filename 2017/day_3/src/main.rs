//http://adventofcode.com/2017/day/3
use std::collections::HashMap;

#[derive(Debug,Eq,Hash,PartialEq)]
pub struct Coordinates {
    x: isize,
    y: isize
}

fn find_coordinates(input: usize) -> Coordinates {
    let mut grid: HashMap<usize, Coordinates> = HashMap::new();
    let input_x = input as isize;
    let input_y = input as isize;
    let mut index = 1;
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut dx: isize  = 0;
    let mut dy: isize = -1;
    let mut t = std::cmp::max(input_x,input_y);
    //let max_i = t.pow(2);
    
    for _i in 0..t {
        if (-input_x/2 <= x) && (x <= input_x/2) && (-input_y/2 <= y) && (y <= input_y/2) {
            grid.insert(index as usize, Coordinates{x:x,y:y});
            index+=1;
        }
        
        if (x == y) || ((x < 0) && (x == -y)) || ((x > 0) && (x == 1-y)) {
            t = dx;
            dx = -dy;
            dy = t;
        }
        
        x += dx;
        y += dy;
    }
    let return_point: Coordinates = Coordinates{x:grid.get(&input).unwrap().x, y:grid.get(&input).unwrap().y};

    return_point
}

fn walk_it_out(point: Coordinates) -> usize {
    point.x.abs() as usize + point.y.abs() as usize
}
fn main() {
    //1, 0 steps
    let point = find_coordinates(1);
    println!("It will take {} steps to get back to the origin", walk_it_out(point));
    
    //12, 3 steps
    let point = find_coordinates(12);
    println!("It will take {} steps to get back to the origin", walk_it_out(point));
    
    //23, 2 steps
    let point = find_coordinates(23);
    println!("It will take {} steps to get back to the origin", walk_it_out(point));
 
    //1024, 31 steps
    let point = find_coordinates(1024);
    println!("It will take {} steps to get back to the origin", walk_it_out(point));


    //102400, 319 steps
    let point = find_coordinates(102400);
    println!("It will take {} steps to get back to the origin", walk_it_out(point));
}