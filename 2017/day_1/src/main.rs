//http://adventofcode.com/2017/day/1

fn inverse_captcha(input: Vec<usize>) -> usize {
    let mut sum_vec = Vec::new();
    let mut last_number = 0;
    let mut equals_sequence: usize = 0;

    for (count,number) in input.iter().enumerate() {
        //special case if all number are the same in the vec
        if equals_sequence == input.len()-1
        {
            return input.len();
        }

        if count == 0 {
            last_number = *number;
            equals_sequence+=1;
        }
        else if count == input.len()-1 {
            if number == &last_number || number == &input[0] {
                sum_vec.push(*number);
                equals_sequence+=1;
            }
        }
        else {
            if number == &last_number {
                sum_vec.push(*number);
                equals_sequence+=1;
            }
            else {
                last_number = *number;
                equals_sequence = 0;
            }
        }
    }

    sum_vec.sort();
    sum_vec.dedup();
    sum_vec.iter().sum()
}

fn main() {
    //should be 3
    let test_vec_one: Vec<usize> = vec!(1,1,2,2);
    let test_one = inverse_captcha(test_vec_one);
    println!("{}", test_one);

    //should be 9
    let test_vec_two: Vec<usize> = vec!(9,1,2,1,2,1,2,9);
    let test_two = inverse_captcha(test_vec_two);
    println!("{}", test_two);

    //should be 0
    let test_vec_three: Vec<usize> = vec!(1,2,3,4);
    let test_three = inverse_captcha(test_vec_three);
    println!("{}", test_three);

    //should be 4
    let test_vec_four: Vec<usize> = vec!(1,1,1,1);
    let test_four = inverse_captcha(test_vec_four);
    println!("{}", test_four);
}
