#![feature(file_buffered)]

use std::fs::File;
use std::io::BufRead;

fn main() {
    // let input = File::open_buffered("../test.txt").unwrap();
    let input = File::open_buffered("../input.txt").unwrap();

    // rows[columns[]]
    let mut grid: Vec<Vec<bool>> = input.lines().map(|v| v.unwrap()).map(|line| {
        line.trim().chars().map(|c| {
            match c {
                '@' => true,
                '.' => false,
                _ => panic!(""),
            }
        }).collect()
    }).collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut sum = 0;

    loop {
        let mut removed = false;

        let mut newgrid = grid.iter().map(|v| v.clone()).collect::<Vec<_>>();
        
        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] {
                    // run the check

                    let mut n = 0;
                
                    let edge_l = row == 0;
                    let edge_r = row == rows - 1;
                    let edge_t = col == 0;
                    let edge_b = col == cols - 1;

                    // corners are impossible to have 4 adj
                    // if (edge_l || edge_r) && (edge_t || edge_b) {sum += 1; continue}

                    if !edge_l {
                        let r = &grid[row - 1];
                        if !edge_t {if r[col - 1] {n += 1}}
                        if r[col] {n += 1}
                        if !edge_b {if r[col + 1] {n += 1}}
                    }
                    if !edge_r {
                        let r = &grid[row + 1];
                        if !edge_t {if r[col - 1] {n += 1}}
                        if r[col] {n += 1}
                        if !edge_b {if r[col + 1] {n += 1}}
                    }

                    let r = &grid[row];
                    if !edge_t {if r[col - 1] {n += 1}}
                    if !edge_b {if r[col + 1] {n += 1}}

                    if n < 4 {
                        sum += 1;
                        removed = true;
                        newgrid[row][col] = false;
                    }
                }
            }
        }   

        grid = newgrid;
        if !removed {break}
    }
    

    println!("total {sum}");
}
