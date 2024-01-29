use std::io::{self};

fn main() {
    //baekjoon problem 15964

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    
    let numbers = buffer.split_whitespace()
    .map(|x| x.parse::<i64>().unwrap())
    .collect::<Vec<i64>>();
    
    let first = numbers.first().unwrap();
    let second = numbers.last().unwrap();

    println!("{}", (first + second) * (first - second));
}
