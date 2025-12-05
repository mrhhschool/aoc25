#![feature(file_buffered)]

use std::fs::File;
use std::io::BufRead;


fn main() {
    // let input = File::open_buffered("../test.1.txt").unwrap();
    let input = File::open_buffered("../input.1.txt").unwrap();

    let ranges: Vec<(u64, u64)> = input.lines().map(|v| v.unwrap()).map(|s| {
        let (l, r) = s.split_once("-").unwrap();
        (u64::from_str_radix(l, 10).unwrap(),
         u64::from_str_radix(r, 10).unwrap())
    }).collect();

    // let input = File::open_buffered("../test.2.txt").unwrap();
    let input = File::open_buffered("../input.2.txt").unwrap();

    let ingredients: Vec<u64> = input.lines().map(|v| v.unwrap()).map(|s| {
        u64::from_str_radix(&s, 10).unwrap()
    }).collect();
    
    // NOTE: could probably be made more efficient with sorting of
    // ranges or whatever

    let total = ingredients.into_iter().filter(|&v| {
        for &(l, h) in ranges.iter() {
            if l <= v && v <= h {return true}
        }
        false
    }).count();
    println!("total {total}");
}
