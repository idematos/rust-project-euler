// Problem #96: Su Doku
// https://projecteuler.net/problem=96

use std::fs;

const SIZE: usize = 9;
const SUBGRID_SIZE: usize = 3;

fn is_valid(board: &[[u8; SIZE]; SIZE], row: usize, col: usize, num: u8) -> bool {
    for i in 0..SIZE {
        if board[row][i] == num || board[i][col] == num {
            return false;
        }
    }
    
    let start_row = (row / SUBGRID_SIZE) * SUBGRID_SIZE;
    let start_col = (col / SUBGRID_SIZE) * SUBGRID_SIZE;
    
    for r in 0..SUBGRID_SIZE {
        for c in 0..SUBGRID_SIZE {
            if board[start_row + r][start_col + c] == num {
                return false;
            }
        }
    }
    
    true
}

fn solve_sudoku(board: &mut [[u8; SIZE]; SIZE]) -> bool {
    for row in 0..SIZE {
        for col in 0..SIZE {
            if board[row][col] == 0 { 
                for num in 1..=SIZE as u8 {
                    if is_valid(board, row, col, num) {
                        board[row][col] = num;

                        if solve_sudoku(board) {
                            return true; 
                        }

                        board[row][col] = 0; 
                    }
                }
                return false; 
            }
        }
    }
  
    true 
}

fn main() {
    let content = fs::read_to_string("sudoku.txt").expect("Unable to read file");
    let mut total_sum = 0;

    for puzzle in content.split("\n\n") {
        let mut board = [[0; SIZE]; SIZE];

        for (row, line) in puzzle.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if let Some(digit) = c.to_digit(10) {
                    board[row][col] = digit as u8; 
                }
            }
        }

        if solve_sudoku(&mut board) {
            let top_left_number = board[0][0] * 100 + board[0][1] * 10 + board[0][2];
            total_sum += top_left_number;
        }
    }

    println!("The sum of the top-left three-digit numbers is {}.", total_sum);
}
