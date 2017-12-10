fn elf_hash(lengths: Vec<usize>, iv_length: usize) -> usize {
    //how many skips to do each loop iteration
    let mut skip_size = 0;

    //setting inital array
    let mut iv: Vec<usize> = (0..iv_length).collect();
    
    //copy origial length for caluclations and in place permuations
    let orig_iv_length = iv.len();

    //keep track of the index
    let mut index = 0;

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
    iv[0]*iv[1]
}

fn main() {
    //should return 12 (start vec is 0,1,2,3,4 and the final vec is 3,4,2,1,0) 3*4=12
    //commented out because of the original iv length is 5 here, not 256
    //let lengths = vec!(3,4,1,5);
     //println!("{}", elf_hash(lengths, 5));

    //challenge
    let lengths = vec!(227,169,3,166,246,201,0,47,1,255,2,254,96,3,97,144);
    println!("{}", elf_hash(lengths, 256));
}
