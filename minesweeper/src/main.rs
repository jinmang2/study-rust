use rand::Rng;
use std::io;

const SIZE: usize = 10; // 게임판의 크기
const MINES: usize = 10; // 지뢰의 개수

fn main() {
    let mut board = [[0; SIZE]; SIZE];
    place_mines(&mut board);
    loop {
        println!("Enter a position to reveal (format: x y): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.len() != 2 {
            println!("Invalid input. Please enter the x and y coordinates separated by a space.");
            continue;
        }
        let x: usize = match parts[0].parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let y: usize = match parts[1].parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if x >= SIZE || y >= SIZE {
            println!("Position out of bounds!");
            continue;
        }
        // 간단한 로직으로, 여기서는 사용자가 선택한 위치의 주변 지뢰 개수만 출력합니다.
        println!("Mines around ({}, {}): {}", x, y, count_mines_around(&board, x, y));
    }
}

fn place_mines(board: &mut [[i32; SIZE]; SIZE]) {
    let mut rng = rand::thread_rng();
    let mut mines_placed = 0;
    while mines_placed < MINES {
        let x = rng.gen_range(0..SIZE);
        let y = rng.gen_range(0..SIZE);
        if board[x][y] == 0 {
            board[x][y] = -1; // 지뢰를 나타내는 값으로 -1을 사용합니다.
            mines_placed += 1;
        }
    }
}

fn count_mines_around(board: &[[i32; SIZE]; SIZE], x: usize, y: usize) -> i32 {
    let mut count = 0;
    for i in 0..3 {
        for j in 0..3 {
            let nx = x as i32 + i - 1;
            let ny = y as i32 + j - 1;
            if nx >= 0 && nx < SIZE as i32 && ny >= 0 && ny < SIZE as i32 {
                if board[nx as usize][ny as usize] == -1 {
                    count += 1;
                }
            }
        }
    }
    count
}
