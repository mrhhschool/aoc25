#![feature(file_buffered)]

use std::io::BufRead;

fn main() {
    // let input = std::fs::File::open_buffered("test.txt").unwrap();
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
        let v = u32::from_str_radix(&line[1..], 10).unwrap();

        println!("{left} {v}");

        // floor divide for number of full wraps
        zero_count += v / 100;
        // 100 turns wrap
        let turns = v % 100;
        assert_ne!(turns, 0);


        if !left {
            position += turns;
            if position >= 100 {zero_count += 1;}
            position %= 100;
        }
        else {
            if position < turns {
                if position != 0 {zero_count += 1}
                position = 100 - (turns - position)
            }
            else if position == turns {
                zero_count += 1;
                position = 0;
            }
            else {
                position -= turns;
            }
        }

        // if position == 0 {zero_count += 1}
        println!("p {position} {zero_count}");
    }
    println!("zero count: {zero_count}");
}
