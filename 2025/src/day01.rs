
fn part1(spins: &Vec<i32>) {
    let mut result = 0;
    let mut dial = 50;
    let dial_max = 100;
    for spin in spins {
        dial = (dial + spin + dial_max) % dial_max;
        if dial == 0 {result += 1}
    }

    println!("{result}");
}

pub fn main() {
    let input = crate::grab_input("day01");

    let spins = input.split_whitespace()
        .map(|val| {
            let mut vals = val.chars();
            let dir: i32 = if vals.next().unwrap() == 'L' {-1} else {1};
            let dist: i32 = vals.collect::<String>().parse().unwrap();

            dir * dist
        })
        .collect::<Vec<_>>();

    part1(&spins);
}