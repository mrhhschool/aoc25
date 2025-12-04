
fn main() {
    // let input = std::fs::read_to_string("../test.txt").unwrap();
    let input = std::fs::read_to_string("../input.txt").unwrap();

    let mut sum = 0;

    for s in input.split(",").map(|s| s.trim()) {
        let (f, l) = s.split_once("-").unwrap();
        let first = usize::from_str_radix(f, 10).unwrap();
        let last = usize::from_str_radix(l, 10).unwrap();

        for v in first..=last {
            let ndigits = v.ilog10() + 1;
            if ndigits & 1 == 1 {
                // println!("not double, ignore");
                continue;
            }
            let halfdigits = ndigits / 2;

            let div = 10_i32.pow(halfdigits) as usize;
            let hi = v / div;
            let lo = v % div;
            if hi == lo {
                // println!("invalid id {v}");
                sum += v;
            }
        }
    }

    println!("total sum {sum}");
}
