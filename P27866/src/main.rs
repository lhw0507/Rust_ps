use std::io::{self};

fn main() {
    //baekjoon problem 27866

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let mut index = String::new();
    io::stdin().read_line(&mut index).unwrap();
    
    //parsing index to u32 not verctor and minus 1
    let index: u32 = index.trim().parse::<u32>().unwrap() - 1;

    //print string(character) at index
    println!("{}", buffer.chars().nth(index as usize).unwrap());
}
