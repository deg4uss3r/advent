//http://adventofcode.com/2017/day/4
use std::collections::HashSet;

fn valid_passphrase(input: String) -> bool {
    let mut password_hash: HashSet<&str> = HashSet::new();
    
    let passphrase_list: Vec<&str> = input.split(' ').collect();
    
    for i in passphrase_list.iter() {
        if password_hash.contains(i) {
            return false;
        }
        else {
            password_hash.insert(i);
        }
    }
    true       
}

fn main() {
    //true
    println!("{}",valid_passphrase("aa bb cc dd ee".to_string()));

    //false
    println!("{}",valid_passphrase("aa bb cc dd aa".to_string()));

    //true
    println!("{}",valid_passphrase("aa bb cc dd aaa".to_string()));
}