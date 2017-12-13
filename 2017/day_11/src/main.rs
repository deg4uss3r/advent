//http://adventofcode.com/2017/day/11

#[derive(Debug,Eq,PartialEq,Hash)]
pub struct Direction {
    ns: Option<String>,
    ew: Option<String>,
}

#[derive(Debug)]
pub struct Position { //north is positive, south is negative
    n: usize,
    s: usize,
    ne: usize, 
    nw: usize,
    se: usize,
    sw: usize
}

fn which_way_did_he_go(dir: &Direction, pos: &mut Position) {
    if dir.ns == Some("n".to_string()) && dir.ew == None {
        pos.n+=1;
    }
    else if dir.ns == Some("s".to_string()) && dir.ew == None {
        pos.s+=1;
    }
    else if dir.ns == Some("n".to_string()) && dir.ew == Some("e".to_string()) {
        pos.ne+=1;
    } 
    else if dir.ns == Some("n".to_string()) && dir.ew == Some("w".to_string()) {
        pos.nw+=1;
    }
    else if dir.ns == Some("s".to_string()) && dir.ew == Some("e".to_string()) {
        pos.se+=1;
    } 
    else if dir.ns == Some("s".to_string()) && dir.ew == Some("w".to_string()) {
        pos.sw+=1;
    } 
}

fn reduce(pos: &mut Position) -> Position {
    let mut red_pos: Position = Position{n:0,s:0,ne:0,nw:0,se:0,sw:0};
    
    //remove proper opposites
    if pos.n > pos.s {
        red_pos.n = pos.n-pos.s;
    }
    else {
        red_pos.s = pos.s-pos.n;
    }

    if pos.ne > pos.sw {
        red_pos.ne = pos.ne-pos.sw;
    }
    else {
        red_pos.sw = pos.sw-pos.ne;
    }

    if pos.nw > pos.se {
        red_pos.nw = pos.nw-pos.se; 
    }
    else {
        red_pos.se = pos.se-pos.nw;
    }



    //remove other crap that has carryovers
    
    red_pos
}

fn steps_to_orphan (input: String) -> usize {
    //setting up hashmap, key is direction, value is inverse
    let mut directions: Vec<Direction> = Vec::new();
    let mut steps = 0;
   
    let orphan_steps: Vec<&str> = input.split(",").collect();
   
    for i in orphan_steps.iter() {
        if i.len() == 2 {
                let dir: Vec<&str> = i.split("").collect();
                let step = Direction{ns:Some(dir[1].to_string()), ew:Some(dir[2].to_string())};
                directions.push(step);
        }
        else {
            let step = Direction{ns:Some(i.to_string()), ew:None};
            directions.push(step);
        }
    }

    let mut away = Position{n:0,s:0,ne:0,nw:0,se:0,sw:0};

    for i in directions.iter() {
        which_way_did_he_go(&i, &mut away);
    }
    
    println!("{:?}", away);

    steps
}
fn main() {
    println!("Test 1 ------");
    //test 1, should be 3 
    let steps = "ne,ne,ne";
    println!("{} steps away", steps_to_orphan(steps.to_string()));
    println!("Test 2 ------");    
    //test 2, should be 0
    let steps = "ne,ne,sw,sw";
    println!("{} steps away", steps_to_orphan(steps.to_string()));
     println!("Test 3 ------");   
    //test 3, should be 2 (se,se)
    let steps = "ne,ne,s,s";
    println!("{} steps away", steps_to_orphan(steps.to_string()));
    println!("Test 4 ------");    
    //test 4, should be 3 (s,s,sw)
    let steps = "se,sw,se,sw,sw";
    println!("{} steps away", steps_to_orphan(steps.to_string()));

    //challenge
    let steps = "se,n,sw,ne,n,n,n,se,nw,nw,nw,sw,sw,ne,sw,ne,sw,nw,sw,nw,sw,sw,sw,sw,sw,s,n,ne,s,s,ne,sw,s,sw,n,ne,s,s,s,s,s,s,s,s,s,se,s,s,se,s,se,se,n,sw,s,se,se,se,s,se,se,s,se,n,se,se,n,nw,se,se,ne,se,se,se,ne,sw,se,ne,se,se,ne,ne,ne,ne,se,s,ne,sw,se,se,nw,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,s,ne,ne,ne,sw,ne,n,n,ne,n,n,s,ne,ne,nw,s,n,ne,ne,n,ne,n,n,ne,n,nw,s,n,ne,ne,se,ne,n,n,n,n,n,n,n,n,n,n,n,n,se,n,n,n,n,n,sw,n,n,n,se,nw,n,nw,se,n,n,s,s,nw,nw,nw,n,nw,n,nw,sw,n,nw,nw,se,nw,n,nw,nw,n,sw,nw,n,n,n,nw,nw,n,nw,nw,nw,nw,nw,nw,nw,nw,ne,n,n,nw,nw,nw,nw,se,nw,n,sw,ne,nw,nw,nw,nw,nw,nw,nw,sw,s,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,sw,ne,sw,nw,nw,sw,nw,se,sw,nw,nw,nw,nw,n,sw,nw,nw,sw,sw,se,se,sw,sw,nw,n,nw,sw,nw,sw,sw,nw,n,nw,sw,sw,sw,nw,n,sw,nw,se,sw,sw,sw,ne,nw,ne,sw,s,nw,nw,sw,sw,nw,sw,sw,sw,nw,ne,sw,sw,sw,nw,nw,sw,sw,sw,n,sw,sw,ne,sw,sw,se,sw,sw,nw,sw,sw,sw,sw,sw,ne,sw,sw,sw,sw,sw,sw,sw,sw,sw,nw,sw,sw,s,sw,sw,sw,sw,s,sw,ne,sw,sw,s,sw,sw,sw,s,sw,sw,s,sw,n,se,s,sw,sw,ne,sw,sw,sw,se,sw,sw,se,s,sw,s,sw,se,sw,sw,nw,sw,sw,sw,sw,s,sw,sw,ne,sw,se,sw,n,sw,sw,s,sw,sw,s,se,s,sw,sw,s,sw,sw,s,sw,s,sw,sw,nw,s,sw,s,sw,s,sw,s,sw,sw,s,s,s,s,s,ne,s,s,sw,s,sw,nw,sw,s,sw,s,sw,sw,sw,ne,sw,s,s,s,n,s,s,s,nw,s,s,s,s,sw,sw,sw,s,se,s,sw,s,nw,s,s,s,s,s,s,nw,s,sw,s,s,s,s,s,s,n,s,s,s,ne,s,s,ne,nw,se,ne,s,s,se,s,ne,s,s,sw,s,s,se,s,s,s,s,sw,sw,s,ne,s,s,s,s,s,ne,sw,sw,s,s,s,s,se,se,sw,s,ne,s,se,s,se,n,s,se,se,s,nw,se,se,se,se,s,s,sw,s,s,s,se,s,s,s,n,se,se,n,s,ne,s,s,s,sw,n,s,s,nw,se,s,s,s,s,se,s,s,se,n,nw,se,se,s,s,n,sw,s,se,se,se,ne,se,s,nw,se,s,se,s,s,s,s,s,s,s,se,se,se,se,se,se,se,s,s,se,se,s,s,s,nw,se,s,s,se,se,s,sw,se,ne,n,se,se,se,s,s,ne,se,se,s,s,n,nw,sw,se,s,se,s,nw,s,s,se,se,s,se,nw,se,se,se,se,se,se,s,nw,se,se,se,s,se,sw,se,se,n,ne,nw,se,se,s,se,se,s,n,se,se,se,s,se,se,se,se,se,nw,ne,se,s,se,se,ne,se,se,sw,nw,se,se,se,se,se,se,s,s,sw,se,se,se,se,se,se,se,ne,s,s,se,se,sw,ne,se,ne,se,se,se,se,se,n,s,se,se,sw,se,se,n,s,nw,ne,ne,se,se,n,ne,se,se,se,se,se,se,se,ne,se,se,se,se,se,se,ne,se,s,se,se,nw,n,ne,se,sw,se,nw,s,se,ne,ne,se,nw,se,ne,se,se,ne,ne,se,se,ne,se,se,se,se,ne,se,s,se,se,ne,se,se,se,se,ne,ne,se,se,se,se,se,s,se,sw,ne,s,n,se,se,ne,s,n,ne,se,ne,se,se,n,se,ne,n,se,se,sw,ne,se,se,se,ne,n,se,s,se,sw,sw,ne,nw,se,n,n,ne,se,se,n,se,se,ne,se,ne,ne,nw,ne,nw,se,ne,ne,ne,ne,n,s,se,sw,ne,se,se,se,se,se,n,sw,ne,ne,se,se,se,se,ne,se,ne,ne,ne,sw,s,se,se,s,ne,s,sw,ne,ne,ne,se,sw,nw,ne,n,ne,s,ne,ne,s,se,s,ne,se,n,sw,s,s,ne,se,ne,n,ne,ne,se,s,ne,sw,se,ne,se,ne,n,n,ne,ne,ne,ne,ne,ne,ne,se,n,ne,s,se,ne,n,ne,se,se,ne,ne,se,ne,ne,se,ne,ne,ne,ne,ne,sw,ne,sw,sw,ne,ne,s,se,n,ne,s,ne,ne,ne,nw,ne,ne,s,n,ne,nw,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,se,ne,ne,ne,ne,se,n,ne,ne,ne,n,ne,ne,ne,ne,ne,n,ne,n,ne,ne,nw,ne,ne,ne,ne,sw,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,nw,ne,se,ne,sw,s,sw,ne,ne,n,ne,sw,ne,n,ne,n,ne,ne,ne,sw,ne,ne,ne,ne,n,ne,ne,ne,ne,ne,ne,ne,se,nw,se,n,ne,ne,ne,nw,ne,ne,ne,ne,ne,ne,n,nw,ne,ne,ne,ne,ne,n,sw,ne,n,ne,n,s,ne,ne,ne,nw,n,ne,se,ne,ne,n,ne,n,nw,ne,nw,ne,n,ne,sw,n,nw,n,ne,n,ne,ne,s,ne,ne,ne,n,ne,ne,ne,n,ne,s,ne,n,ne,n,ne,nw,ne,n,ne,n,ne,ne,ne,sw,n,ne,ne,n,n,sw,ne,ne,n,ne,s,n,n,n,ne,ne,n,nw,se,se,se,ne,n,nw,ne,se,ne,ne,ne,n,ne,ne,sw,n,se,n,n,ne,n,nw,ne,n,n,ne,ne,se,ne,sw,ne,ne,n,n,n,sw,n,n,n,n,n,ne,n,n,s,ne,n,ne,n,n,ne,n,n,ne,n,ne,ne,n,n,ne,n,n,sw,n,n,ne,nw,n,nw,n,n,ne,ne,nw,n,n,s,n,ne,n,n,n,n,n,n,ne,n,n,n,ne,n,s,n,se,ne,se,ne,ne,ne,sw,n,sw,nw,n,n,n,se,ne,n,n,nw,n,n,nw,n,ne,n,ne,n,s,ne,n,nw,n,n,n,nw,n,n,nw,se,ne,s,s,n,ne,n,n,nw,n,n,se,s,n,n,se,n,n,n,ne,ne,n,n,s,ne,n,n,nw,ne,n,se,n,n,ne,n,n,n,nw,s,ne,n,sw,sw,n,n,s,n,n,n,s,s,nw,nw,ne,n,n,n,sw,n,n,n,n,se,nw,se,n,nw,n,n,n,n,n,n,sw,se,s,n,n,n,n,n,n,n,n,n,n,n,n,se,n,n,n,n,n,sw,n,n,n,n,n,n,n,n,n,n,nw,n,sw,n,n,n,n,n,n,n,se,n,nw,n,nw,n,n,nw,n,n,n,n,sw,ne,n,n,n,n,nw,n,n,n,ne,n,se,se,s,n,n,n,n,n,n,n,s,nw,n,n,n,n,n,n,se,n,n,s,n,n,nw,n,nw,n,n,se,n,n,n,se,n,n,n,n,se,n,n,n,n,ne,ne,se,ne,n,s,se,n,nw,n,s,s,s,se,n,n,n,n,n,n,n,nw,nw,n,n,nw,nw,n,n,n,nw,n,sw,n,nw,sw,sw,nw,n,sw,n,n,se,nw,n,nw,nw,n,s,n,n,n,nw,se,n,n,se,n,n,n,n,nw,n,sw,n,n,n,n,n,n,n,n,se,n,n,n,n,n,ne,nw,nw,sw,n,nw,n,n,nw,n,n,n,nw,nw,sw,s,n,n,n,sw,n,n,se,n,n,se,nw,n,nw,n,n,nw,nw,n,nw,nw,nw,sw,s,n,nw,nw,nw,n,nw,ne,n,n,se,n,nw,n,n,n,nw,se,s,n,n,n,n,n,n,s,nw,n,nw,n,nw,nw,n,n,nw,nw,nw,n,nw,n,n,nw,s,nw,n,n,sw,n,nw,nw,s,sw,nw,nw,n,n,n,n,n,nw,n,nw,nw,nw,n,nw,nw,nw,n,n,nw,nw,nw,nw,nw,sw,n,n,ne,se,n,n,nw,se,s,n,n,n,nw,nw,nw,se,n,n,nw,s,n,n,se,nw,se,n,n,nw,se,n,n,n,sw,n,s,nw,nw,nw,n,ne,nw,nw,n,nw,ne,se,n,nw,nw,s,sw,s,n,nw,n,nw,n,se,n,n,nw,nw,nw,n,n,nw,nw,nw,nw,n,n,ne,nw,nw,n,nw,n,n,se,nw,se,n,s,n,n,nw,nw,nw,n,n,se,n,se,ne,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,n,n,nw,nw,nw,s,n,nw,nw,nw,n,n,n,nw,n,n,n,nw,nw,nw,ne,nw,nw,sw,nw,n,nw,nw,s,nw,nw,nw,nw,nw,n,se,n,nw,nw,se,nw,nw,nw,se,nw,ne,n,nw,s,n,nw,nw,nw,s,nw,nw,se,n,ne,n,nw,nw,nw,nw,se,nw,n,nw,nw,nw,nw,nw,nw,nw,nw,s,nw,n,s,n,sw,nw,nw,n,nw,nw,n,nw,nw,nw,nw,sw,sw,se,nw,se,n,nw,nw,s,nw,nw,nw,ne,s,n,nw,sw,nw,nw,nw,nw,n,sw,nw,nw,nw,nw,nw,s,nw,nw,nw,ne,nw,n,n,nw,nw,ne,nw,nw,s,nw,nw,n,nw,s,nw,nw,nw,n,nw,s,sw,nw,nw,nw,nw,s,nw,nw,se,sw,nw,nw,ne,n,ne,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,s,nw,nw,se,s,nw,nw,sw,nw,nw,nw,se,nw,s,nw,nw,nw,n,nw,se,nw,sw,nw,nw,nw,se,nw,nw,nw,nw,nw,nw,n,nw,nw,nw,nw,ne,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,ne,nw,nw,nw,n,s,nw,sw,nw,s,nw,nw,nw,nw,nw,s,nw,nw,nw,nw,sw,nw,sw,s,nw,nw,nw,nw,sw,nw,s,sw,sw,se,sw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,se,sw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,sw,sw,sw,nw,se,nw,nw,nw,nw,sw,ne,nw,nw,nw,nw,nw,sw,nw,se,n,ne,nw,sw,s,nw,nw,nw,nw,ne,nw,se,ne,sw,nw,sw,nw,sw,nw,nw,nw,se,s,nw,nw,nw,ne,nw,ne,nw,nw,nw,nw,n,sw,sw,sw,nw,s,nw,nw,nw,nw,nw,nw,n,nw,nw,nw,sw,nw,nw,nw,sw,sw,nw,n,nw,nw,nw,s,nw,nw,nw,nw,n,sw,nw,ne,sw,nw,sw,s,s,sw,sw,nw,nw,nw,se,sw,nw,sw,nw,nw,n,sw,nw,sw,sw,nw,nw,nw,n,nw,s,nw,ne,s,nw,nw,sw,sw,nw,ne,s,nw,nw,n,s,sw,nw,nw,nw,s,sw,sw,sw,nw,nw,nw,se,nw,ne,n,sw,sw,sw,nw,nw,sw,nw,sw,nw,ne,nw,nw,sw,nw,nw,nw,s,nw,sw,nw,n,nw,nw,nw,nw,nw,nw,n,nw,sw,s,se,sw,sw,sw,nw,sw,nw,nw,nw,s,sw,nw,n,nw,nw,nw,ne,sw,nw,sw,nw,nw,sw,sw,nw,sw,nw,se,nw,nw,nw,sw,sw,nw,sw,nw,nw,nw,nw,nw,sw,nw,nw,sw,nw,nw,sw,nw,nw,se,sw,nw,nw,sw,nw,sw,nw,n,sw,nw,sw,sw,sw,nw,s,sw,nw,se,nw,se,sw,se,nw,n,nw,nw,sw,nw,n,sw,nw,nw,sw,nw,se,n,nw,sw,nw,sw,sw,nw,nw,n,sw,sw,sw,sw,nw,ne,sw,sw,s,sw,nw,nw,nw,n,sw,s,nw,nw,se,sw,sw,nw,sw,sw,sw,se,sw,sw,sw,nw,sw,sw,se,se,s,nw,sw,nw,sw,sw,nw,nw,sw,nw,nw,nw,sw,nw,nw,sw,sw,sw,sw,nw,sw,nw,sw,sw,sw,sw,sw,se,sw,ne,nw,sw,nw,sw,se,sw,nw,sw,nw,sw,sw,sw,sw,nw,s,sw,nw,sw,nw,se,nw,sw,s,nw,nw,sw,se,se,se,sw,sw,sw,nw,se,nw,s,nw,nw,sw,nw,ne,sw,nw,n,sw,nw,nw,sw,s,s,nw,n,nw,nw,sw,s,nw,sw,nw,sw,nw,nw,sw,se,ne,nw,sw,ne,sw,sw,nw,sw,nw,s,nw,n,ne,se,sw,sw,sw,se,nw,sw,se,nw,sw,nw,n,sw,s,se,nw,sw,sw,nw,sw,nw,sw,se,sw,nw,sw,n,sw,sw,se,nw,nw,sw,ne,sw,sw,nw,sw,sw,s,sw,sw,nw,sw,sw,nw,sw,sw,sw,nw,sw,s,sw,sw,sw,sw,nw,nw,nw,nw,se,nw,n,nw,sw,se,sw,sw,sw,nw,n,nw,nw,sw,s,sw,sw,sw,nw,nw,s,sw,sw,nw,sw,nw,sw,sw,sw,nw,sw,nw,s,ne,sw,nw,nw,s,sw,sw,sw,sw,sw,sw,se,sw,sw,ne,sw,s,sw,sw,nw,sw,sw,nw,sw,sw,sw,sw,n,nw,nw,sw,sw,sw,sw,s,n,sw,sw,nw,sw,nw,sw,ne,ne,se,sw,sw,se,sw,nw,s,n,nw,sw,sw,nw,sw,sw,nw,sw,nw,sw,sw,ne,sw,sw,nw,s,sw,nw,n,sw,sw,sw,nw,ne,s,sw,sw,sw,se,n,se,nw,sw,sw,nw,sw,sw,ne,sw,sw,sw,n,s,n,sw,sw,sw,n,sw,se,sw,s,se,sw,sw,sw,n,sw,nw,sw,n,nw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,s,sw,sw,sw,sw,nw,nw,sw,sw,s,se,ne,sw,se,sw,s,sw,sw,s,sw,sw,n,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,n,sw,sw,n,sw,ne,sw,sw,nw,sw,sw,sw,sw,sw,ne,sw,sw,sw,sw,s,sw,nw,sw,sw,nw,n,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,ne,sw,sw,s,sw,n,sw,sw,sw,se,n,sw,sw,sw,sw,sw,sw,se,sw,sw,sw,sw,sw,ne,s,nw,sw,se,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,nw,sw,sw,s,sw,s,s,sw,sw,sw,ne,se,sw,sw,sw,sw,s,sw,sw,sw,s,nw,s,s,s,sw,sw,n,sw,sw,n,ne,sw,sw,sw,se,sw,s,sw,sw,s,s,sw,sw,ne,sw,sw,sw,sw,ne,sw,sw,sw,nw,s,se,sw,se,sw,s,sw,n,se,sw,ne,sw,sw,sw,sw,sw,se,sw,sw,sw,sw,s,sw,s,sw,sw,s,nw,n,sw,sw,sw,sw,sw,s,sw,se,nw,sw,sw,sw,n,s,sw,s,sw,sw,sw,nw,sw,n,sw,sw,sw,sw,sw,se,sw,sw,sw,sw,s,sw,sw,sw,sw,ne,s,sw,sw,nw,sw,s,sw,n,sw,se,sw,ne,sw,sw,sw,sw,sw,ne,sw,sw,sw,sw,sw,sw,se,sw,sw,sw,s,se,s,sw,sw,sw,sw,sw,sw,sw,sw,s,se,ne,s,s,sw,ne,sw,sw,ne,sw,ne,nw,sw,ne,sw,nw,sw,sw,nw,sw,n,sw,sw,sw,sw,se,sw,s,sw,sw,sw,sw,sw,sw,sw,nw,nw,s,s,sw,s,ne,s,sw,sw,sw,sw,sw,sw,s,sw,nw,nw,s,sw,sw,n,sw,sw,n,s,sw,sw,sw,s,sw,nw,sw,s,sw,s,nw,n,se,sw,sw,sw,sw,s,sw,s,s,sw,sw,sw,sw,sw,nw,s,s,s,sw,ne,s,sw,ne,sw,sw,sw,s,s,nw,s,n,sw,sw,n,s,sw,sw,sw,s,ne,sw,sw,s,s,sw,s,sw,s,s,ne,sw,sw,sw,sw,sw,sw,sw,sw,nw,sw,sw,sw,sw,n,n,sw,s,sw,sw,ne,s,ne,sw,sw,se,sw,ne,sw,n,sw,sw,sw,n,ne,ne,sw,sw,s,sw,s,sw,n,sw,sw,s,nw,s,sw,se,ne,sw,sw,sw,s,n,sw,ne,s,sw,ne,sw,sw,s,nw,sw,nw,sw,s,s,sw,ne,sw,s,sw,se,s,sw,sw,sw,sw,se,s,n,sw,sw,sw,ne,sw,sw,s,s,sw,sw,sw,n,nw,sw,sw,sw,nw,n,se,sw,se,sw,s,sw,sw,s,s,sw,sw,n,sw,s,sw,sw,sw,s,s,s,sw,nw,s,sw,s,nw,ne,s,sw,sw,s,s,s,s,sw,sw,s,s,sw,sw,s,sw,sw,s,sw,sw,sw,sw,sw,s,sw,sw,s,s,sw,sw,sw,s,sw,ne,s,sw,sw,s,sw,sw,sw,s,sw,sw,sw,nw,s,sw,s,ne,sw,sw,se,s,sw,s,s,s,sw,sw,sw,s,sw,n,sw,s,ne,se,se,sw,s,s,sw,s,s,sw,ne,ne,n,sw,s,sw,s,sw,s,s,sw,ne,sw,s,se,sw,sw,n,sw,n,s,s,nw,s,sw,sw,ne,sw,ne,sw,s,ne,s,s,ne,ne,sw,s,n,sw,s,nw,s,s,sw,sw,sw,n,s,s,s,sw,se,se,sw,s,sw,n,sw,ne,sw,s,sw,sw,s,s,sw,sw,sw,nw,s,sw,s,nw,sw,s,s,nw,s,ne,s,sw,ne,s,se,s,sw,sw,s,s,n,sw,sw,nw,sw,ne,sw,sw,se,sw,sw,sw,sw,se,s,sw,s,sw,sw,sw,s,ne,s,s,sw,sw,s,s,sw,se,n,sw,nw,sw,sw,s,s,s,s,sw,sw,sw,s,s,s,se,nw,s,sw,s,sw,sw,ne,s,sw,s,n,s,s,s,s,sw,sw,ne,sw,sw,sw,sw,s,sw,sw,n,sw,n,s,sw,s,s,ne,s,nw,s,ne,sw,s,sw,n,s,n,s,n,ne,sw,sw,s,s,ne,s,sw,s,s,s,s,sw,s,s,s,se,s,s,nw,s,ne,ne,sw,se,s,sw,ne,s,s,s,s,s,sw,s,s,s,s,s,sw,n,s,sw,s,s,n,s,nw,s,sw,s,se,n,s,sw,sw,s,s,n,sw,sw,sw,se,sw,s,s,s,ne,s,sw,n,s,s,n,sw,s,s,sw,s,s,s,sw,s,s,s,s,n,sw,s,se,s,se,sw,n,se,s,s,s,sw,s,s,ne,s,ne,se,sw,ne,s,s,sw,s,s,s,s,s,s,ne,sw,sw,s,s,s,s,s,sw,s,sw,ne,s,sw,s,s,s,sw,sw,ne,sw,nw,se,s,nw,s,sw,sw,sw,s,s,sw,s,s,s,n,sw,sw,se,s,sw,s,s,s,sw,s,s,s,s,s,s,s,s,s,se,sw,s,s,sw,n,n,s,ne,s,s,sw,s,s,sw,sw,s,s,s,s,s,s,s,s,s,s,s,s,ne,s,s,s,n,se,s,n,sw,s,s,sw,s,sw,s,n,s,s,s,sw,n,se,n,sw,s,sw,n,s,sw,s,s,s,n,sw,sw,se,s,s,s,s,s,ne,sw,ne,s,s,sw,nw,s,s,sw,s,s,n,ne,nw,sw,s,s,s,s,s,sw,s,s,ne,nw,s,s,ne,s,sw,s,s,ne,sw,s,s,s,s,s,s,n,s,s,nw,s,s,s,sw,s,s,s,s,s,s,s,s,s,ne,sw,s,s,sw,s,nw,s,s,s,nw,s,s,se,s,s,ne,s,sw,s,ne,s,n,s,s,nw,nw,s,s,sw,s,s,s,s,s,ne,s,s,s,s,s,s,nw,sw,s,s,s,ne,s,s,s,s,s,s,nw,sw,s,s,nw,s,s,s,se,ne,sw,s,s,nw,s,s,sw,s,sw,s,s,s,nw,n,s,se,sw,sw,sw,s,s,s,nw,s,s,ne,s,s,s,s,ne,n,ne,nw,nw,s,s,s,s,s,se,nw,s,s,s,sw,s,s,s,s,se,s,se,s,ne,n,sw,s,s,s,se,ne,s,s,s,s,s,s,s,s,s,s,s,s,s,s,s,s,s,s,ne,ne,s,sw,ne,s,sw,s,s,s,sw,nw,s,s,ne,s,s,s,s,nw,s,s,s,s,sw,s,s,s,s,s,n,s,s,s,ne,sw,s,s,s,s,s,s,ne,s,s,ne,ne,s,s,ne,s,s,s,s,se,s,s,s,s,s,s,s,s,s,s,se,s,se,se,s,sw,s,s,s,s,sw,s,nw,s,n,n,se,s,s,s,s,se,se,s,ne,s,s,s,se,s,s,s,s,s,s,s,s,n,s,n,s,s,s,s,s,nw,s,s,s,s,s,s,s,s,s,s,s,s,s,s,s,s,s,ne,s,s,s,nw,s,s,s,s,s,s,s,s,s,sw,s,ne,s,s,s,nw,s,se,s,s,ne,s,n,s,s,s,s,se,se,s,s,n,s,s,n,s,s,nw,s,s,s,s,nw,s,s,s,s,ne,s,ne,nw,n,s,s,s,se,s,s,sw,s,s,se,s,s,s,s,sw,s,n,s,se,s,n,s,s,s,n,s,s,se,s,n,nw,s,s,s,s,s,n,se,n,s,se,s,s,s,se,s,s,s,s,s,s,n,se,s,s,se,s,s,s,se,sw,nw,s,se,se,s,s,s,s,ne,se,se,ne,se,s,ne,s,se,s,s,se,s,s,se,s,s,s,n,s,s,s,s,s,s,s,s,nw,se,n,nw,s,se,s,s,se,s,s,s,s,s,sw,sw,n,s,nw,s,s,s,se,s,s,se,s,se,se,ne,se,s,s,s,ne,s,s,sw,s,se,s,ne,sw,ne,se,nw,se,s,ne,nw,s,s,s,sw,sw,se,s,s,s,nw,s,nw,s,ne,se,nw,se,ne,se,sw,s,s,s,se,n,s,n,s,s,se,s,s,se,s,s,s,s,s,s,n,s,sw,se,s,s,s,s,se,se,s,s,s,s,s,s,s,sw,se,sw,se,se,s,s,s,se,se,se,s,se,ne,s,se,se,s,s,s,n,s,sw,s,ne,s,s,nw,s,s,s,s,s,s,se,se,s,s,s,s,s,s,nw,s,se,se,s,sw,s,s,se,s,se,se,s,s,s,se,s,s,s,s,ne,nw,nw,s,s,se,sw,s,s,s,se,s,s,se,se,s,ne,n,nw,s,n,sw,s,s,se,s,s,nw,se,sw,s,s,s,se,se,s,s,s,s,n,ne,se,se,s,se,s,s,se,s,se,s,se,s,se,s,s,s,se,s,se,sw,s,s,ne,ne,s,s,s,sw,se,s,sw,se,se,se,se,se,se,se,se,se,s,sw,sw,se,s,s,se,se,ne,s,s,se,s,se,s,s,s,s,s,s,se,se,se,se,nw,s,nw,s,s,s,s,se,se,se,s,s,s,se,sw,se,se,s,s,se,s,se,sw,se,n,nw,sw,n,se,se,s,s,s,s,s,s,s,s,se,s,nw,s,sw,nw,sw,s,se,se,s,s,se,s,s,s,s,s,se,se,s,se,n,se,n,se,se,s,se,ne,s,ne,n,s,s,s,n,se,n,s,s,nw,n,se,se,se,se,nw,s,s,s,se,nw,s,se,se,s,sw,s,se,se,n,s,ne,s,s,se,se,s,s,sw,se,s,se,se,s,s,sw,s,s,se,s,se,s,se,s,s,s,n,s,s,s,ne,se,s,s,s,s,se,se,sw,s,s,se,s,se,se,se,s,s,se,se,s,se,se,ne,se,s,se,se,s,se,se,se,se,s,s,se,s,se,sw,se,se,se,se,se,s,s,s,nw,ne,s,s,se,s,se,s,s,n,se,se,s,s,s,n,s,s,s,se,s,se,nw,se,s,se,s,se,s,s,se,se,ne,s,s,s,s,ne,s,s,se,s,sw,s,se,n,se,s,s,se,se,s,se,se,se,s,n,se,n,se,se,se,ne,se,s,se,se,s,s,se,s,nw,se,n,se,n,s,s,s,ne,se,s,s,s,se,s,se,se,s,s,ne,s,se,sw,se,s,se,ne,ne,se,se,se,s,s,se,s,se,se,se,s,se,se,se,se,se,s,se,s,sw,s,s,se,sw,se,se,s,s,se,s,s,se,ne,s,s,se,nw,s,n,s,s,se,se,sw,se,s,s,s,ne,se,n,s,se,s,s,se,se,s,se,nw,s,se,se,se,s,se,s,se,se,se,se,s,s,se,se,se,s,s,se,se,s,s,se,se,se,se,se,se,s,s,s,se,s,se,n,se,se,s,s,s,ne,se,se,se,se,s,ne,s,se,se,se,se,se,se,s,se,se,s,se,se,se,se,s,sw,s,se,s,s,se,se,se,se,se,se,s,s,s,s,se,n,se,s,ne,s,s,ne,n,n,s,nw,se,se,se,se,s,se,se,n,s,se,se,sw,se,se,se,se,se,se,se,se,s,se,se,s,ne,se,se,se,nw,n,s,s,se,s,s,n,se,sw,ne,se,se,se,se,se,se,se,se,s,s,s,se,se,ne,s,se,se,se,s,s,s,se,n,n,se,s,sw,se,se,se,s,n,s,se,se,s,nw,se,ne,se,s,se,se,n,se,sw,se,s,s,s,s,se,nw,se,se,s,se,s,ne,se,s,se,s,se,se,se,s,se,n,se,se,se,se,se,se,se,se,se,se,s,se,s,se,se,se,nw,se,se,se,nw,s,s,se,sw,se,se,s,s,s,se,s,se,s,se,s,s,se,se,se,nw,se,se,s,s,se,se,sw,se,se,se,se,se,n,se,se,se,s,se,se,s,se,se,s,se,se,sw,se,se,s,sw,se,se,s,se,se,se,se,s,nw,s,se,se,ne,se,se,s,se,se,se,se,sw,s,s,nw,s,s,se,se,se,se,ne,nw,se,se,se,ne,se,se,se,se,se,se,se,se,se,se,se,se,se,se,s,se,se,se,s,se,sw,se,nw,s,s,se,se,se,se,se,se,nw,s,se,nw,se,se,se,se,s,se,se,s,ne,sw,se,se,se,s,se,se,se,se,se,se,se,sw,se,sw,se,se,n,se,se,s,ne,se,se,se,se,s,se,se,ne,s,s,nw,se,sw,se,se,se,se,se,s,nw,s,se,se,se,se,se,sw,se,ne,s,se,se,se,se,se,ne,se,se,s,sw,s,n,se,se,se,se,s,se,se,se,s,nw,se,s,se,se,se,s,se,sw,se,se,se,s,n,s,se,se,n,se,s,n,se,nw,se,s,se,se,se,ne,nw,se,se,se,se,nw,se,s,s,se,ne,se,se,sw,sw,se,se,s,se,s,se,se,se,se,se,se,se,sw,s,se,se,se,se,nw,se,se,se,se,se,se,se,s,se,se,se,se,s,se,s,ne,n,se,ne,s,se,se,n,se,s,se,s,se,se,se,se,se,se,se,se,se,se,se,se,s,se,se,ne,se,ne,se,s,sw,se,se,nw,se,se,nw,se,se,nw,se,s,s,se,se,n,se,n,se,se,se,se,se,se,se,s,se,se,nw,se,se,se,se,se,nw,se,s,ne,se,se,se,se,se,se,n,se,se,s,se,nw,se,n,ne,se,se,se,se,se,se,ne,n,se,ne,s,se,se,se,nw,se,se,se,nw,se,se,se,se,ne,s,se,se,se,se,se,se,se,se,se,se,se,se,se,se,sw,s,se,se,n,se,sw,se,se,se,s,se,n,se,se,se,se,se,se,s,se,se,se,nw,se,se,se,se,s,se,se,se,se,s,n,se,se,se,sw,se,se,se,sw,s,se,se,se,s,se,se,s,n,se,se,se,n,se,se,se,se,nw,n,se,se,se,se,se,se,ne,sw,n,nw,nw,n,se,n,se,n,ne,n,se,s,ne,ne,ne,sw,ne,ne,ne,ne,ne,ne,ne,ne,se,ne,se,s,nw,se,s,se,ne,se,se,se,se,se,se,n,ne,ne,n,s,se,se,s,s,sw,s,nw,se,s,s,s,s,nw,s,s,se,s,s,s,s,s,s,s,s,s,s,s,s,s,sw,s,s,sw,nw,s,s,s,ne,n,sw,sw,sw,ne,s,sw,n,sw,sw,sw,n,sw,s,nw,sw,sw,n,se,sw,sw,sw,ne,sw,nw,sw,sw,sw,nw,sw,sw,nw,sw,se,nw,nw,sw,nw,sw,sw,s,nw,s,sw,nw,nw,nw,sw,nw,nw,sw,nw,nw,nw,s,nw,sw,nw,nw,nw,nw,nw,n,nw,nw,nw,ne,nw,s,se,nw,nw,sw,nw,nw,nw,sw,nw,nw,nw,nw,n,nw,nw,nw,n,nw,nw,n,nw,nw,nw,nw,nw,n,nw,n,nw,nw,n,s,n,sw,nw,nw,n,n,n,s,n,n,n,n,nw,n,s,n,nw,n,se,s,n,s,se,nw,n,nw,se,n,n,nw,n,n,n,n,n,n,ne,se,n,n,n,n,n,n,s,ne,n,ne,sw,n,n,n,nw,s,n,ne,n,ne,n,nw,n,n,n,n,nw,ne,ne,ne,ne,ne,ne,sw,se,se,ne,nw,n,ne,nw,s,n,n,n,n,n,ne,ne,sw,se,n,n,s,n,se,ne,n,sw,s,ne,sw,sw,n,ne,nw,n,ne,n,s,ne,ne,ne,ne,ne,ne,n,nw,n,nw,ne,ne,sw,ne,n,ne,ne,n,ne,ne,ne,ne,sw,ne,ne,ne,ne,ne,ne,ne,ne,nw,ne,ne,se,ne,ne,ne,ne,se,ne,ne,ne,ne,ne,ne,ne,ne,nw,nw,s,sw,ne,sw,ne,s,se,sw,ne,se,ne,ne,nw,ne,ne,se,ne,sw,ne,ne,se,ne,n,se,ne,sw,ne,sw,nw,ne,ne,ne,se,ne,ne,se,ne,se,se,se,n,ne,se,s,se,ne,se,ne,n,se,ne,ne,se,ne,se,se,nw,ne,ne,ne,ne,n,se,ne,ne,ne,ne,ne,ne,se,ne,se,se,se,se,n,ne,sw,se,se,ne,s,se,ne,ne,se,ne,se,se,se,se,se,ne,se,ne,se,se,se,se,se,se,se,se,ne,se,se,se,n,ne,nw,s,se,se,se,se,sw,se,se,se,sw,se,se,se,se,se,s,se,se,n,se,se,se,se,se,se,sw,se,se,ne,se,s,se,se,se,se,se,sw,s,se,nw,s,se,se,se,se,se,se,se,s,s,s,se,s,nw,n,se,se,nw,se,n,se,se,se,se,n,se,s,s,se,se,nw,se,se,nw,se,sw,se,se,s,sw,s,se,sw,se,s,nw,se,se,ne,se,se,se,se,se,s,s,sw,s,s,s,sw,s,se,se,se,se,se,se,s,s,se,s,s,se,se,se,se,se,s,se,s,se,se,s,sw,s,n,nw,s,se,s,se,se,ne,s,s,s,sw,ne,se,se,s,s,s,nw,nw,s,se,s,se,se,s,n,ne,se,ne,se,s,n,s,se,se,s,se,n,se,s,nw,nw,nw,s,se,s,s,ne,se,se,se,s,se,s,sw,s,nw,s,s,nw,s,ne,s,s,n,s,s,se,s,s,s,s,sw,s,s,se,se,s,ne,s,se,s,s,s,s,s,s,s,s,s,s,ne,s,s,n,s,nw,s,ne,s,s,s,s,s,sw,s,s,s,n,s,ne,nw,s,nw,s,s,se,ne,nw,s,s,s,ne,s,s,s,s,nw,se,s,s,se,s,s,nw,s,s,n,s,sw,s,s,s,s,s,s,s,s,s,s,s,s,s,s,sw,s,s,n,s,s,ne,s,s,s,s,sw,s,s,s,sw,sw,sw,sw,ne,s,s,ne,s,nw,sw,s,s,s,nw,sw,sw,s,s,sw,sw,ne,ne,sw,s,sw,ne,sw,sw,n,s,sw,s,sw,s,s,s,sw,s,sw,ne,s,s,s,sw,s,sw,s,s,s,sw,n,s,sw,sw,s,sw,sw,n,sw,s,sw,ne,s,s,sw,sw,sw,sw,sw,sw,s,sw,s,s,sw,s,s,se,s,se,s,sw,sw,s,sw,s,s,sw,s,sw,sw,s,sw,s,s,se,sw,s,sw,s,s,s,ne,sw,sw,s,sw,s,n,sw,nw,s,s,s,sw,s,sw,sw,se,sw,sw,sw,nw,s,sw,sw,se,sw,nw,s,sw,sw,nw,s,s,sw,sw,s,s,sw,se,s,s,sw,ne,sw,se,sw,sw,s,s,ne,sw,sw,nw,sw,se,sw,sw,sw,s,s,sw,sw,nw,ne,sw,sw,sw,s,s,sw,sw,sw,s,s,sw,sw,sw,s,nw,s,sw,sw,sw,n,sw,sw,se,s,sw,sw,sw,sw,sw,sw,sw,sw,s,sw,sw,sw,sw,sw,sw,sw,sw,s,s,sw,sw,sw,sw,sw,sw,sw,n,sw,sw,sw,sw,sw,sw,sw,sw,n,se,sw,s,sw,sw,sw,sw,sw,sw,sw,s,sw,nw,sw,sw,sw,sw,sw,ne,sw,sw,sw,sw,nw,sw,nw,sw,sw,sw,sw,sw,sw,sw,s,s,n,sw,sw,nw,sw,sw,sw,nw,sw,se,sw,nw,sw,sw,nw,sw,sw,ne,nw,sw,se,se,nw,sw,sw,ne,nw,nw,sw,nw,sw,sw,n,sw,sw,sw,nw,s,sw,sw,sw,sw,nw,sw,se,sw,n,sw,sw,se,s,sw,ne,sw,sw,sw,ne,nw,n,sw,sw,sw,sw,sw,sw,sw,nw,sw,s,ne,sw,nw,sw,sw,se,sw,sw,nw,nw,sw,sw,sw,sw,sw,se,nw,sw,sw,nw,s,sw,s,nw,se,n,nw,sw,sw,sw,sw,sw,nw,se,nw,sw,nw,sw,sw,nw,s,sw,sw,nw,sw,nw,sw,sw,sw,nw,nw,sw,se,sw,nw,sw,nw,nw,ne,nw,nw,nw,nw,sw,nw,sw,nw,nw,sw,sw,n,nw,s,sw,sw,nw,nw,nw,sw,sw,nw,n,s,sw,sw,s,sw,sw,se,nw,sw,sw,nw,sw,sw,sw,nw,sw,nw,nw,sw,nw,nw,nw,nw,nw,nw,nw,nw,sw,s,nw,ne,ne,n,nw,sw,nw,nw,nw,sw,nw,nw,ne,nw,sw,sw,se,sw,nw,ne,nw,sw,sw,sw,nw,n,se,nw,sw,nw,nw,nw,se,sw,sw,nw,sw,nw,sw,nw,n,nw,nw,nw,nw,nw,sw,n,nw,sw,sw,nw,nw,nw,n,nw,sw,sw,n,nw,sw,nw,nw,sw,nw,n,s,nw,n,sw,nw,nw,sw,s,nw,sw,s,nw,n,s,nw,nw,nw,nw,nw,nw,nw,n,nw,sw,sw,se,nw,nw,nw,n,nw,sw,nw,nw,sw,se,nw,nw,nw,sw,nw,sw,nw,sw,nw,nw,se,nw,nw,nw,sw,nw,nw,ne,se,sw,sw,nw,ne,nw,n,nw,sw,nw,n,nw,se,nw,sw,sw,s,nw,nw,se,se,sw,nw,sw,nw,nw,nw,ne,sw,nw,nw,nw,nw,sw,nw,ne,ne,nw,nw,sw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,s,ne,nw,nw,nw,nw,n,nw,n,nw,nw,s,n,nw,nw,nw,ne,nw,nw,nw,nw,nw,ne,nw,nw,nw,nw,nw,n,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,s,sw,nw,n,nw,nw,sw,nw,nw,se,nw,nw,nw,nw,n,nw,nw,nw,ne,nw,nw,nw,nw,n,nw,nw,nw,nw,ne,nw,sw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,n,ne,se,sw,nw,se,ne,nw,nw,nw,sw,nw,n,nw,ne,nw,n,se,n,nw,ne,nw,nw,n,n,nw,nw,nw,se,nw,se,nw,nw,nw,n,nw,nw,nw,nw,nw,nw,nw,nw,nw,s,nw,nw,n,nw,n,nw,nw,nw,n,nw,nw,s,nw,se,nw,nw,ne,nw,ne,nw,se,n,nw,nw,n,n,nw,nw,nw,nw,nw,s,nw,nw,nw,nw,nw,s,nw,nw,nw,nw,nw,nw,nw,nw,nw,n,nw,nw,nw,n,nw,s,nw,nw,n,se,se,nw,nw,se,n,nw,nw,nw,nw,ne,nw,nw,n,nw,s,nw,se,sw,sw,nw,nw,nw,ne,nw,nw,n,nw,n,nw,nw,n,nw,ne,nw,nw,nw,ne,sw,nw,ne,nw,nw,nw,nw,nw,nw,ne,n,nw,nw,n,n,nw,nw,s,nw,ne,nw,nw,nw,n,nw,nw,nw,nw,nw,n,s,se,nw,sw,nw,n,nw,n,nw,s,nw,n,n,ne,n,n,n,n,se,nw,se,nw,n,nw,n,sw,n,s,nw,nw,nw,n,nw,nw,nw,n,sw,n,n,nw,nw,nw,s,nw,ne,n,sw,nw,nw,nw,n,nw,n,nw,nw,nw,n,n,s,n,nw,se,nw,nw,n,n,nw,n,nw,nw,se,nw,n,nw,n,s,se,se,n,n,se,nw,n,n,n,s,nw,se,n,n,n,s,nw,ne,n,n,se,sw,nw,n,nw,n,ne,n,n,n,sw,n,n,n,nw,n,n,nw,n,s,n,n,n,n,sw,n,nw,sw,nw,s,n,n,n,s,n,nw,se,nw,nw,n,n,n,n,n,nw,nw,se,n,nw,n,nw,n,nw,n,nw,n,nw,nw,ne,nw,nw,nw,n,n,n,n,ne,n,nw,n,sw,sw,nw,n,nw,nw,n,n,n,n,n,n,n,ne,n,nw,nw,nw,se,n,n,n,n,n,nw,ne,n,n,sw,n,ne,nw,ne,n,n,ne,n,nw,sw,n,s,n,n,n,s,n,n,nw,se,n,ne,ne,n,n,ne,s,sw,n,n,n,nw,n,nw,n,n,s,nw,s,nw,n,n,n,ne,n,n,sw,sw,n,n,n,n,sw,n,n,nw,n,n,n,n,n,n,n,n,n,n,n,n,sw,n,n,n,nw,n,n,n,n,n,nw,ne,nw,n,n,n,n,nw,n,s,n,n,n,s,n,n,n,n,n,n,sw,n,n,ne,nw,n,n,nw,se,nw,nw,n,n,n,n,n,n,n,n,n,n,n,s,n,n,n,n,s,n,n,n,n,n,n,n,n,n,n,nw,n,n,s,n,nw,n,n,n,n,n,n,n,n,ne,n,n,n,n,nw,n,s,n,n,n,n,n,n,n,n,n,se,n,n,n,n,nw,s,n,sw,n,n,nw,n,n,sw,n,n,n,n,n,n,nw,se,n,n,ne,n,nw,n,nw,se,n,ne,n,n,sw,n,n,n,ne,n,n,n,sw,ne,nw,n,n,nw,nw,n,nw,n,se,sw,n,ne,n,n,n,ne,ne,ne,s,ne,n,n,n,n,n,n,n,n,sw,n,n,se,n,n,sw,n,ne,n,ne,n,n,n,nw,n,ne,ne,sw,nw,n,n,n,n,n,n,n,n,s,n,s,n,ne,n,n,n,ne,ne,sw,se,s,ne,n,n,ne,n,se,sw,n,n,n,ne,n,n,ne,n,n,n,n,n,n,n,se,sw,n,n,ne,nw,n,n,n,nw,n,n,n,n,n,n,n,ne,n,n,ne,n,n,ne,n,n,ne,n,n,ne,nw,n,n,n,se,n,n,n,n,n,n,n,ne,n,n,n,ne,ne,n,ne,ne,n,n,ne,n,n,ne,n,n,ne,n,sw,n,ne,sw,n,ne,ne,n,n,ne,sw,ne,sw,n,ne,n,s,n,n,sw,n,n,ne,n,n,ne,ne,nw,ne,n,n,n,n,n,n,sw,n,nw,ne,se,nw,n,n,ne,n,se,ne,n,n,n,sw,n,n,n,n,ne,n,se,n,n,n,sw,n,n,ne,s,ne,ne,nw,ne,s,s,ne,ne,ne,s,ne,ne,nw,n,n,sw,se,s,se,ne,ne,n,n,n,n,n,n,n,s,s,ne,ne,ne,n,ne,n,ne,ne,nw,n,ne,ne,ne,n,n,n,ne,ne,n,n,s,ne,n,ne,sw,n,ne,n,se,ne,se,ne,se,n,s,n,ne,n,n,n,se,ne,n,ne,nw,ne,ne,n,n,ne,s,n,nw,ne,nw,ne,ne,n,ne,se,sw,n,n,ne,s,ne,n,ne,nw,sw,sw,n,ne,n,nw,n,ne,n,ne,ne,n,s,ne,n,nw,ne,se,ne,ne,ne,nw,ne,ne,n,n,n,ne,n,ne,se,nw,ne,n,ne,ne,ne,ne,n,ne,se,ne,se,ne,n,ne,ne,ne,ne,ne,ne,se,n,n,n,n,s,n,se,n,s,se,s,ne,n,ne,s,sw,n,ne,n,n,ne,ne,ne,ne,n,sw,sw,n,sw,n,ne,nw,ne,n,n,n,ne,n,n,s,n,n,se,s,ne,s,n,n,ne,n,n,ne,sw,ne,ne,ne,n,n,ne,sw,ne,ne,n,nw,ne,ne,ne,n,n,ne,ne,n,ne,n,s,n,n,nw,s,ne,ne,ne,ne,ne,ne,ne,n,ne,ne,ne,ne,s,ne,se,ne,nw,ne,ne,ne,ne,ne,ne,ne,n,ne,n,n,ne,ne,nw,n,ne,n,ne,nw,ne,n,ne,ne,ne,ne,n,nw,ne,nw,ne,ne,n,ne,s,n,ne,ne,ne,ne,ne,ne,sw,ne,n,n,se,s,s,ne,n,n,n,s,n,s,ne,nw,n,ne,s,ne,sw,n,ne,n,n,ne,ne,ne,ne,s,ne,s,ne,nw,n,ne,n,ne,ne,ne,ne,se,se,ne,ne,ne,s,ne,se,ne,ne,ne,ne,ne,ne,sw,ne,se,ne,ne,ne,ne,ne,ne,n,ne,n,n,n,ne,s,nw,ne,n,ne,n,n,nw,s,ne,n,ne,ne,ne,ne,ne,s,ne,ne,ne,se,ne,se,ne,ne,ne,sw,n,ne,ne,n,ne,ne,n,se,ne,ne,sw,ne,n,ne,n,n,ne,n,s,nw,ne,sw,ne,n,n,ne,nw,n,ne,se,n,s,ne,ne,nw,n,ne,ne,ne,sw,nw,ne,n,n,nw,se,ne,ne,ne,ne,ne,ne,n,ne,ne,sw,ne,s,ne,s,ne,ne,ne,n,ne,ne,nw,ne,ne,n,nw,ne,se,ne,ne,n,sw,ne,ne,ne,sw,nw,ne,ne,sw,ne,ne,ne,ne,sw,ne,ne,ne,ne,ne,ne,ne,ne,s,ne,ne,ne,ne,ne,ne,sw,ne,se,n,ne,sw,sw,ne,ne,ne,ne,ne,sw,ne,ne,ne,ne,ne,ne,s,ne,ne,ne,ne,n,ne";
    println!("{} steps away", steps_to_orphan(steps.to_string()));
}