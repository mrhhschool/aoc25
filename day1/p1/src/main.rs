#![feature(file_buffered)]

use std::io::BufRead;

fn main() {
    let input = std::fs::File::open_buffered("input.txt").unwrap();

    let mut position = 50;
    let mut zero_count = 0;

    for line in input.lines().map(|v| v.unwrap()) {
        let left = match line.chars().nth(0) {
            Some('L') => true,
            Some('R') => false,
            Some(c) => panic!("bad input, {c:?}"),
            None => break,
        };
        // 100 turns wrap
        let v = u32::from_str_radix(&line[1..], 10).unwrap() % 100;
        println!("{left} {v}");

        if !left {
            position += v;
            position %= 100;
        }
        else {
            if position < v {position = 100 - (v - position)}
            else {position -= v}
        }

        if position == 0 {zero_count += 1}
        println!("p {position}");
    }
    println!("zero count: {zero_count}");
}
