//http://adventofcode.com/2017/day/14
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

fn elf_hash_two(length_string: String) -> Vec<String> {
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

    //change to an actual number so we can count 1s later
    let mut output: Vec<String> = Vec::new();
    for i in hash_xor_result.iter() {
        output.push(format!("{:02x}",i));
    }

    output
}

fn elf_hash_fourteen(input: String) -> u32 {
    let mut grid_count = 0; 
    let mut grid: Vec<Vec<String>> = Vec::new();

    while grid_count < 128 {
        let new_input = format!("{}-{}", input, grid_count);
        let row: Vec<String> = elf_hash_two(new_input);
        grid.push(row);
        grid_count+=1;
    }

    let mut grid_nums: Vec<Vec<usize>> = Vec::new();

    for row in grid.iter() {
        let mut number_row: Vec<usize> = Vec::new();
        for number in row.iter() {
            number_row.push(usize::from_str_radix(number, 16).unwrap()); 
        }
        grid_nums.push(number_row); 
    }

    let mut ones = 0;

    for row in grid_nums.iter() {
        for num in row.iter() {
            ones+=num.count_ones();            
        }
    }

    ones
}

fn main() {
    //test 1 should  be 8108
    let test_input = "flqrgnkx";
    println!("{}", elf_hash_fourteen(test_input.to_string()));

    //challenge
    let input = "hwlqcszp";
    println!("{}", elf_hash_fourteen(input.to_string()));

    let input = "xlqgujun";
    println!("{}", elf_hash_fourteen(input.to_string()));
}