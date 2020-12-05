//http://adventofcode.com/2017/day/10
#[allow(dead_code)]
#[allow(unused_assignments)]

//abstraction of the hash function
fn elf_hash(lengths: &Vec<usize>, mut iv:Vec<usize>, mut skip_size:usize, mut index:usize) -> (Vec<usize>,usize,usize) {
    //copy origial length for caluclations and in place permuations
    let orig_iv_length = iv.len();

    //calling the index from the parent level keep track of the index
    //calling the skip_size from the parent level

    for i in lengths.iter() {

        let mut rev_slice: Vec<usize> = Vec::new();

        //reset the index if we overflow (go around the circle)
        if index >= orig_iv_length {
            index = index%orig_iv_length;
        }

        if i == &orig_iv_length { 
                //grab from the index to the end
                rev_slice = iv.drain(index..).collect();

                //index to split for piecing back together
                let mut out_slice = Vec::new();

                //get the rest
                out_slice = iv.drain(..).collect();

                //put the rest at the end
                rev_slice.append(&mut out_slice); 
                rev_slice.reverse();

                let mut rev_slice_new = rev_slice.split_off(orig_iv_length-index);
                iv.append(&mut rev_slice_new);
                iv.append(&mut rev_slice);

        }
        else if i+index >= orig_iv_length { //in this case we just need to split reverse and put both back at the correct indexs
                //this is the first part

                rev_slice = iv.drain(index..).collect();
                let future_split = rev_slice.len();

                //how much more do we need (length - length so far)
                let remainder = i-rev_slice.len(); 

                //getting the remaining bits
                let mut front_pull = Vec::new();
                front_pull = iv.drain(..remainder).collect();

                //putting it all together to reverse
                rev_slice.append(&mut front_pull);
                rev_slice.reverse();

                //splitting back
                let back_portion = rev_slice.split_off(future_split);

                //filling index 0 first
                iv.splice(0..0, back_portion);
                iv.append(&mut rev_slice);

        }
        else {
            rev_slice = iv.drain(index..i+index).collect();
            rev_slice.reverse();
            iv.splice(index..index, rev_slice);
        }

        index+=i+skip_size;
        skip_size+=1;
    }

    (iv, skip_size, index)
}

fn elf_hash_one(length_string:String) -> usize {
    //how many skips to do each loop iteration
    let mut skip_size = 0;
    let mut index = 0;

    //setting inital array (0 -> 255)
    let mut iv: Vec<usize> = (0..256).collect(); 

    let mut ov:Vec<usize> = Vec::new();

    let length_string_vec: Vec<&str> = length_string.split(",").collect();
    let mut lengths: Vec<usize> = Vec::new();
    for i in length_string_vec.iter() {
        lengths.push(i.parse::<usize>().unwrap());
    }

    match elf_hash(&lengths, iv, skip_size, index) {
        (x,y,z) => {ov=x;skip_size=y;index=z;}
    }
    ov[0]*ov[1]
}

fn elf_hash_two(length_string: String) {
    //converting string to ASCII decimal
    let mut lengths: Vec<usize> = length_string.into_bytes().iter().map(|x| *x as usize).collect();
    //adding magic word
    lengths.extend(vec!(17, 31, 73, 47, 23));
    
    //globals
    let mut hash_result: Vec<usize> = (0..256).collect();
    let mut hash_run = 0;
    let mut total_index = 0;
    let mut total_skip_size = 0;

    //run the hash 64 times using the seeded skip_size and index
    while hash_run < 64 {
        match elf_hash(&lengths, hash_result, total_skip_size, total_index) {
            (x,y,z) => {hash_result=x; total_skip_size=y; total_index=z;}
        }
        hash_run+=1;
    }

    //grab 16 byte blocks of result and XOR
    let mut hash_xor_index: usize = 0;
    let mut hash_xor_result: Vec<usize> = Vec::new();


    while hash_xor_index < 256 {
        let xor:Vec<usize> = hash_result[hash_xor_index..hash_xor_index+16].to_vec();
        hash_xor_index+=16;

        let mut xor_byte = 0;
        for x in xor.iter() {
            xor_byte = xor_byte^x;
        }

        hash_xor_result.push(xor_byte);
    }

    let mut output = String::new();

    for i in hash_xor_result.iter() {
        output = format!("{}{:02x}",output,i);
    }

    println!("Knot hash sparse: {}", output);
}

fn main() {
    //should return 12 (start vec is 0,1,2,3,4 and the final vec is 3,4,2,1,0) 3*4=12
    //commented out because of the original iv length is 5 here, not 256
    //let lengths = vec!(3,4,1,5);
    //println!("{}", elf_hash(lengths, 5));

    //challenge
        //part one still works fine
    let lengths = "227,169,3,166,246,201,0,47,1,255,2,254,96,3,97,144";
    println!("Part1: {}", elf_hash_one(lengths.to_string()));
        //part two, challenge line
    elf_hash_two(lengths.to_string());

    //part 2: test 1, should be 33efeb34ea91902bb2f59c9920caa6cd
    //let lengths = "AoC 2017";
    //elf_hash_two(lengths.to_string());
}