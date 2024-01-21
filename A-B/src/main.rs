use std::io::{self};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let input_string = buffer.to_string();
    let numbers : Vec<&str> = input_string.split_whitespace().collect();

    let first = i32::from_str_radix(numbers[0], 10).unwrap();
    let second = i32::from_str_radix(numbers[1], 10).unwrap();

    println!("{}", first - second);
}
