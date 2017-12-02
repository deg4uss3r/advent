//http://adventofcode.com/2017/day/2

fn checksum(input: Vec<Vec<usize>>) -> usize {
    let mut output: Vec<usize> = Vec::new();
    let mut high = usize::min_value();
    let mut low = usize::max_value();

    for row in input.iter() {
        for (number_count, number) in row.iter().enumerate() {
            if number_count == 0 {
                high = *number; 
                low = *number;
            }
            else {
                if number > &high {
                    high = *number;
                }
                if number < &low {
                    low = *number;
                }
            }
        }

        output.push(high-low);
    }

    output.iter().sum()
}

fn main() {
    let test_spreadsheet: Vec<Vec<usize>> = vec!(vec!(5,1,9,5),vec!(7,5,3),vec!(2,4,6,8));
    println!("{}", checksum(test_spreadsheet));
}
