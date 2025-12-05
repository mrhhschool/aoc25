
// starting at 2 digits
// possible mirror # of digits:
// turns out this is just factors...
const DIGITS_DIV: &[&[usize]] = &[
    &[1], // 2
    &[1], // 3
    &[1, 2], // 4 xxbb
    &[1], // 5
    &[1, 2, 3], // 6 aaabbb aabbcc
    &[1], // 7
    &[1, 2, 4], // 8 aabbccdd aaaabbbb
    &[1, 3], // 9 threes
    &[1, 2, 5], // 10
];

const DIGITS: [usize; 5] = const {
    let mut ret = [0; 5];
    let mut i = 0;
    while i < 5 {
        let v = i + 1;
        ret[i] = 10_usize.pow(v as u32);
        i += 1;
    }
    ret
};

fn is_repeat(v: usize, ndigits: usize) -> bool {
    let divs = DIGITS_DIV[ndigits - 2];
    for div_exp in divs {
        let n = ndigits / div_exp;
        let div = DIGITS[div_exp - 1];

        let mut cur = 1;

        let check = (v / cur) % div;
        cur *= div;

        let mut i = 1;
        loop {
            if i == n {return true}
            let n = (v / cur) % div;
            if n != check {break}

            cur *= div;
            i += 1;
        }
    }
    false
}

fn main() {
    let input = std::fs::read_to_string("../test.txt").unwrap();
    // let input = std::fs::read_to_string("../input.txt").unwrap();

    let mut sum = 0;
    let mut maxdigits = 0;

    for s in input.split(",").map(|s| s.trim()) {
        let (f, l) = s.split_once("-").unwrap();
        let first = usize::from_str_radix(f, 10).unwrap();
        let last = usize::from_str_radix(l, 10).unwrap();

        for v in first..=last {
            let ndigits = v.ilog10() as usize + 1;
            if ndigits > maxdigits {maxdigits = ndigits}
            if ndigits == 1 {continue}

            if is_repeat(v, ndigits) {
                // println!("invalid id {v}");
                sum += v;
            }
        }
    }

    println!("maxdigits {maxdigits}");
    println!("total sum {sum}");
}
