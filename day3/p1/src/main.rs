#![feature(file_buffered)]
#![feature(int_from_ascii)]

use std::fs::File;
use std::io::BufRead;

fn main() {
    // let input = File::open_buffered("../test.txt").unwrap();
    let input = File::open_buffered("../input.txt").unwrap();

    let mut sum = 0;

    for line in input.lines().map(|v| v.unwrap()) {
        let vals = line.trim().chars().map(|c| {
            c.to_digit(10).unwrap()
        }).collect::<Vec<u32>>();

        println!("{vals:?}");

        let (mut max_idx, mut max) = (0, 0);
        for (idx, &val) in vals.iter().enumerate() {
            // > because we want the FIRST greatest value
            if val > max {max_idx = idx; max = val}
        }

        let (v1, v2) = if max_idx == vals.len() - 1 {
            // max is at the end, so it HAS to be 2nd digit
            // search from beginning right before prev max
            (*vals[0..max_idx].iter().max().unwrap(), max)
        }
        else {
            // search after max to the end for greatest
            (max, *vals[max_idx+1..].iter().max().unwrap())
        };

        let v = v1 * 10 + v2;
        println!("val {v}");

        sum += v;
    }
    println!("total {sum}");
}
