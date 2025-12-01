
const DIAL_MAX: i32 = 100;

fn part1(spins: &Vec<i32>) {
    let mut result = 0;
    let mut dial = 50;
    for spin in spins {
        dial = ((dial + spin) % DIAL_MAX + DIAL_MAX) % DIAL_MAX;
        if dial == 0 {result += 1}
    }

    println!("{result}");
}

fn part2(spins: &Vec<i32>) {
    let mut result = 0;
    let mut dial = 50;
    for spin in spins {
        if spin.abs() > DIAL_MAX {result += spin.abs() / DIAL_MAX} // |spin| > MAX => crosses 0 |spin|/MAX times
        let spin = spin % DIAL_MAX; // remaining spin

        // |spin|<MAX that *leave* 0 don't qualify as crossing 0
        if (dial + spin <= 0 || dial + spin >= DIAL_MAX) && dial != 0 {result += 1} 
        dial = ((dial + spin) % DIAL_MAX + DIAL_MAX) % DIAL_MAX;
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
    part2(&spins); // THE SPINS GO ABOVE 100
}