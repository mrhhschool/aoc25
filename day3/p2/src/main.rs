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

        // progressively pick digits
        // for example, the first digit CANNOT be in the last 12 digits
        // then 11 and so on, finding the largest of each group until done
        // the search window must be underneath the previous one,
        // because if we have already found the larger one, a lower one but
        // before will just make it smaller

        let mut final_val: u64 = 0;
        let mut last_pos = 0;

        // note log_2(10^12) ~= 40
        // so we need 64 bits

        for i in 0..12 {
            let end = line.len() - 11 + i;

            let (mut max_idx, mut max) = (0, 0);
            for idx in last_pos..end {
                let val = vals[idx];
                // > because we want the FIRST greatest value
                if val > max {max_idx = idx; max = val}
            }
            last_pos = max_idx + 1;
            final_val = final_val * 10 + max as u64;
        }

        println!("val {final_val}");
        sum += final_val;
    }
    println!("total {sum}");
}
