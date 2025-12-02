use itertools::Itertools;

fn check_validity_gen(input: isize, splits: &Vec<u32>) -> bool {
    let length = input.checked_ilog10().unwrap_or(0) + 1;
    for &prime in splits {
        if length % prime != 0 {continue}

        let block_length = length / prime;
        let base = input % 10_isize.pow(block_length);
        let mut flag = true;
        for block in 1..prime {
            if base != (input / 10_isize.pow(block_length * block)) % 10_isize.pow(block_length) {flag = false; break;};
        }
        if flag {return false};
    }
    true
}

fn sum_invalids(input: &Vec<(isize, isize)>, splits: Vec<u32>) {
    let mut invalid_sum: isize = 0;
    for (a,b) in input {
        for c in *a..=*b {
            if !check_validity_gen(c, &splits) {invalid_sum += c};
        }
    }
    println!("{invalid_sum}");
}

pub fn main() {
    let input = crate::grab_input("day02");
    
    let ranges: Vec<(isize, isize)> = input.split_whitespace().collect::<String>()
        .split(",")
        .map(|range_full| {
            let (a,b) = range_full.split("-").collect_tuple().unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();
    
    sum_invalids(&ranges, vec![2]); // part 1
    sum_invalids(&ranges, vec![2,3,5,7,11]); // part 2
}