

fn determine_max_joltage(input: &str, batteries_remaining: u32) -> Option<u64> {
    if batteries_remaining == 0 {return Some(0)}
    if input.len() < batteries_remaining as usize {return None}
    for target in "9876543210".chars() { // uh oh greedy!!
        let this_step = 10u64.pow(batteries_remaining - 1) * u64::from(target.to_digit(10).unwrap());
        if let Some((_, split_rem)) = input.split_once(target) {
            if let Some(remainder_value) = determine_max_joltage(split_rem, batteries_remaining - 1) {
                return Some(this_step + remainder_value)
            }
        }
    }
    None
}

fn determine_cumulative_max_joltage(input: &Vec<&str>, batteries: u32) {
    let result: u64 = input.iter().map(|&row| determine_max_joltage(row, batteries).unwrap()).sum();
    println!("{batteries} batteries: {result}");
}

pub fn main() {
    let input = crate::grab_input("day03");
    let rows: Vec<&str> = input.split_whitespace().collect();

    determine_cumulative_max_joltage(&rows, 2);
    determine_cumulative_max_joltage(&rows, 12);
}