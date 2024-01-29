use std::io::{self};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let matrix_size = buffer.split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut matrix = vec![];
    matrix.resize(matrix_size[0] as usize, vec![]);

    //최초 입력
    for i in 0..matrix_size[0] {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        let row = buffer.split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        matrix[i as usize] = row;
    }

    let mut matrix2 = vec![];
    matrix2.resize(matrix_size[0] as usize, vec![]);
    for i in 0..matrix_size[0] {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        let row = buffer.split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        matrix2[i as usize] = row;
    }

    //출력
    for i in 0..matrix_size[0] {
        for j in 0..matrix_size[1] {
            print!("{} ", matrix[i as usize][j as usize] + matrix2[i as usize][j as usize]);
        }
        println!();
    }
}
