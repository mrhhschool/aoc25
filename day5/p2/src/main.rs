#![feature(file_buffered)]

use std::fs::File;
use std::io::BufRead;

fn merge_range(ranges: &mut Vec<(u64, u64)>) -> bool {
    for i in 0..ranges.len() {
        for j in i+1..ranges.len() {
            // yes, I just like writing `orange`
            let range = ranges[i];
            let orange = ranges[j];

            // NOTE: we use < instead of <= because
            // containing ranges have already been trimmed
            // XXX: except a problem: adjacent ranges
            // that's why <= on the edge
            
            // fold `orange` into `range`
            if orange.0 < range.0 && orange.1 <= range.1 && orange.1 >= range.0 {
                // extend range down
                ranges[i].0 = orange.0;
                ranges.remove(j);
                return true
            }
            else if orange.1 > range.1 && orange.0 >= range.0 && orange.0 <= range.1 {
                // extend range up
                ranges[i].1 = orange.1;
                ranges.remove(j);
                return true
            }
        }
    }
    false
}

fn swallow(ranges: &Vec<(u64, u64)>) -> Option<usize> {
    for (i, range) in ranges.iter().enumerate() {
        for (j, orange) in ranges.iter().enumerate().filter(|&(j, _)| j != i) {
            // does range fully contain orange?
            if range.0 <= orange.0 && range.1 >= orange.1 {
                return Some(j);
            }
        }
    }
    None
}

fn main() {
    // let input = File::open_buffered("../mytest.txt").unwrap();
    // let input = File::open_buffered("../test.1.txt").unwrap();
    let input = File::open_buffered("../input.1.txt").unwrap();

    let mut ranges: Vec<(u64, u64)> = input.lines().map(|v| v.unwrap()).map(|s| {
        let (l, r) = s.split_once("-").unwrap();
        (u64::from_str_radix(l, 10).unwrap(),
         u64::from_str_radix(r, 10).unwrap())
    }).collect();

    // swallow redundant ranges:
    // - [   {} ]
    // - {  []  }

    // NOTE: I'm doing everything iteratively now because it didn't work
    // in single-passes before: likely some issues like ranges merging later
    // into fully encompassing ranges that can't be swallowed because swallow
    // pass is over

    loop {
        let mut changed = false;
        
        // swallow everything
        loop {
            match swallow(&ranges) {
                Some(i) => {ranges.remove(i); changed = true},
                None => break,
            }
        }    

        // merge everything
        loop {
            let found = merge_range(&mut ranges);
            if found {changed = true}
            else {break}
        }

        if !changed {break}
    }
    
    // println!("{ranges:?}");

    let total: u64 = ranges.into_iter().map(|(l, h)| h - l + 1).sum();
    println!("total {total}");
}
