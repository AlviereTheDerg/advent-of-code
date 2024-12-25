

fn parse_keylock(input: &str) -> (Vec<usize>, bool) {
    let rows = input.split_whitespace().collect::<Vec<_>>();
    (
        (0..5)
            .map(|column| {
                rows.iter()
                    .filter_map(|row| row.chars().nth(column))
                    .filter(|val| *val == '#')
                    .count() - 1
            })
            .collect(),
        input.chars().next().unwrap() == '.'
    )
}

fn part1(keys: &Vec<Vec<usize>>, locks: &Vec<Vec<usize>>) {
    let mut result = 0;
    for key in keys {
        for lock in locks {
            if key.iter().zip(lock.iter()).all(|(a,b)| a+b <= 5) {
                result += 1;
            }
        }
    }
    println!("{result}");
}

pub fn main() {
    let input = crate::grab_input("day25");
    let mut keys: Vec<Vec<usize>> = vec![];
    let mut locks: Vec<Vec<usize>> = vec![];
    for keylock in input.split("\n\n") {
        let (keylock, key) = parse_keylock(keylock);
        if key {
            keys.push(keylock);
        } else {
            locks.push(keylock);
        }
    }
    
    part1(&keys, &locks);
}