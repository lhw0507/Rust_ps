use std::io::{self};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let matrix_size = buffer.split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut matrix = Vec::new().resize(matrix_size[0] as usize, Vec::new().resize(matrix_size[1] as usize, 0));

    //최초 입력
    for i in 0..matrix_size[0] {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        let row = buffer.split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        matrix[i as usize] = row;
    }

    //더하기
    for i in 0..matrix_size[0] {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        let row = buffer.split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        matrix[i as usize] += row;
    }

    //출력
    for i in 0..matrix_size[0] {
        for j in 0..matrix_size[1] {
            print!("{} ", matrix[i as usize][j as usize]);
        }
        println!();
    }
}
